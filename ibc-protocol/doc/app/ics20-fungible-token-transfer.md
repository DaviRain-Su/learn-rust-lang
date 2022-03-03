# [ICS020 Fungible Token Transfer](https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer)

> 中文翻译

## 摘要

这个标准文档规定了数据包结构、状态机处理逻辑，以及用于在不同链的两个模块之间通过IBC Channel传输可替换的代币编码细节。提出的状态机逻辑允许 安全的多链面额处理与无权限通道开放。该逻辑构成了一个"可替换的代币转移的桥梁模块"，
在IBC路由模块和主机状态机上的现有资产跟踪模块之间进行对接。

## 动机

通过IBC协议连接的一组链的用户可能希望在另一个链上使用一个链上发行的资产，也许可以利用额外的功能，如交换或隐私保护，同时保留与发行链上原始资产的可替代性。
这个应用层标准描述了一个在用IBC连接的链之间转移可替换代币的协议，该协议保持了资产可替换性，保留了资产所有权，限制了拜占庭式错误的影响，并且不需要 额外的许可。

## 定义

IBC处理程序接口和IBC路由模块接口分别在ICS25和ICS26定义

### 所需属性

- 保存可替代性(双向挂钩)
- 保持总供应量(单一源链和模块上的恒定或通货膨胀)
- 无许可令牌传输，不需要将连接、模块或面额列入白名单
- 对称性（所有链都实现相同的逻辑，不区分枢纽和区域的协议内差异）
- 故障遏制：防止由于B链的拜占庭行为（尽管任何向b链发送令牌的用户都可能面临风险），而导致源自A链的令牌的拜占庭膨胀。

## 技术规范

### 数据结构

只需要一种数据包数据类型: FungibleTokenPacketData, 它指定面额、数额、发送账户和接受账户。

```typescript
interface FungibleTokenPacketData {
    denomination: string
    amount: uint256
    sender: string
    receiver: string
}
```

```rust
struct FungibleTokenPacketData {
    denomination: String,
    amount: u256,
    sender: String,
    receiver: String,
}
```

当代币使用ICS20协议跨链发送时，它们开始累积他们被传送过的通道的记录。这一信息被编码到面额字段中。

ICS20代币的面额以{ics20Port}/{ics20Channel}/{denom}的形式表示,其中ics20Port和icsChannel是当前链上存在资金的一个ics20端口和通道。
前缀的端口和通道对表示资金以前通过哪个通道发送的。如果{denom}包含/，那么它也必须是ics20形式的，表示这个令牌有一个多跳记录。注意，这要求在非ibc 令牌面额中禁止使用/（斜杠字符）。

一个发送链可以充当源区或汇区。当一个链在一个端口和通道发送令牌，而该端口和通道不等于最后一个前缀端口和通道对时，它是作为一个源区行事。 当代币从源区发送时，目地端口和通道将被预缀到名称上（一旦接收到令牌），为代币记录增加一个跳数。
当一个链在一个端口和通道上发送代币，而这个端口和通道与最后一个前缀的端口和通道对等时，他就充当一个汇入区。
当代币从汇合区发送时，该名称上的最后一个前缀的端口和信道对被移除（一旦收到代币），撤销代币记录中的最后一跳。IBC-go实现中提供链更完整的解释。

确认数据类型描述了传输是成功还是失败，以及失败的原因（如果有的话）

```typescript
type FungibleTokenPacketAcknowledgement = FungibleTokenPacketSuccess | FungibleTokenPacketError;

interface FungibleTokenPacketSuccess {
  // This is binary 0x01 base64 encoded
  result: "AQ=="
}

interface FungibleTokenPacketError {
  error: string
}
```

请注意，FungibleTokenPacketData和FungibleTokenPacketAcknowledgement在序列化为数据包时必须是json编码的(而不是Protobuf编码的)。
还要注意，unint256在转换为JSON时是字符串编码的，但必须是有效的[0-9]+形式的十进制数字。

可替换代币转移桥模块跟踪与状态中特定通道相关的托管地址，ModuleStated的字段被认为是在范围内。

```typescript
interface ModuleState {
  channelEscrowAddresses: Map<Identifier, string>
}
```

### 次级协议

这里描述的子协议应该在"可替代的代币转移桥"模块中实现，该模块可以访问bank模块和ibc路由模块。

#### 端口和通道设置

当模块被创建时（也许是在区块链本身被初始化时）`setup`函数必须被精确地调用一次，以绑定到合适的端口并创建一个托管地址（由模块拥有）。

```typescript
function setup() {
  capability = routingModule.bindPort("bank", ModuleCallbacks {
    onChanOpenInit,
    onChanOpenTry,
    onChanOpenAck,
    onChanOpenConfirm,
    onChanCloseInit,
    onChanCloseConfirm,
    onRecvPacket,
    onTimeoutPacket,
    onAcknowledgePacket,
    onTimeoutPacketClose
  } )
  claimCapability("port", capability)
}
```

一旦setup函数被调用，就可以通过ibc路由模块在不同链上的可替换代币传输模块实例之间创建通道。

管理员（拥有在主机状态上创建连接和通道的权限）负责设置与其他状态机的连接，并在其他链上创建与该模块（或支持该接口的其他模块）的其他实例的通道。
本规范只定义数据包处理语义，并且这样的方式定义它们，即模块本身不需要担心在任何时间点上存在或不存在什么连接或通道。

#### 路由模块回调

##### 通道生命周期管理

机器A和机器B都接受来自另一台机器上任何模块的新通道，当且仅当：

- 所创建的通道是无序的
- 版本字符串是ics20-1

```typescript
function onChanOpenInit(
  order: ChannelOrder,
  connectionHops: [Identifier],
  portIdentifier: Identifier,
  channelIdentifier: Identifier,
  counterpartyPortIdentifier: Identifier,
  counterpartyChannelIdentifier: Identifier,
  version: string) {
  // only unordered channels allowed
  abortTransactionUnless(order === UNORDERED)
  // assert that version is "ics20-1"
  abortTransactionUnless(version === "ics20-1")
  // allocate an escrow address
  channelEscrowAddresses[channelIdentifier] = newAddress()
}
```

```typescript
function onChanOpenTry(
  order: ChannelOrder,
  connectionHops: [Identifier],
  portIdentifier: Identifier,
  channelIdentifier: Identifier,
  counterpartyPortIdentifier: Identifier,
  counterpartyChannelIdentifier: Identifier,
  version: string,
  counterpartyVersion: string) {
  // only unordered channels allowed
  abortTransactionUnless(order === UNORDERED)
  // assert that version is "ics20-1"
  abortTransactionUnless(version === "ics20-1")
  abortTransactionUnless(counterpartyVersion === "ics20-1")
  // allocate an escrow address
  channelEscrowAddresses[channelIdentifier] = newAddress()
}
```

```typescript
function onChanOpenAck(
  portIdentifier: Identifier,
  channelIdentifier: Identifier,
  version: string) {
  // port has already been validated
  // assert that version is "ics20-1"
  abortTransactionUnless(version === "ics20-1")
}
```

```typescript
function onChanOpenConfirm(
  portIdentifier: Identifier,
  channelIdentifier: Identifier) {
  // accept channel confirmations, port has already been validated, version has already been validated
}
```

```typescript
function onChanCloseInit(
  portIdentifier: Identifier,
  channelIdentifier: Identifier) {
  // no action necessary
}
```

```typescript
function onChanCloseConfirm(
  portIdentifier: Identifier,
  channelIdentifier: Identifier) {
  // no action necessary
}
```

##### 包中继

简单地说，在a和b之间：

- 当作为源区时，桥接模块在发送链上托管现有的本地资产面额，并在接收链上铸造凭证。
- 当作为汇入区时，桥接模块在发送链上销毁本地凭证，并在接收链上解除对本地资产面值的托管
- 当数据包超时时，本地资产将被解押返还给发送方，或者将凭证适当地铸回发送方
- 确认数据用于处理失败，例如无效的面值或无效的目标账户。返回失败的确认比中止交易更可取，因为它更容易使发送链根据失败的性质采取适当的行动。

createOutgoingPacket 必须由模块中的交易处理程序调用，该处理程序执行适当的签名检查，具体到主机状态机上的账户所有者。

```typescript
function createOutgoingPacket(
  denomination: string,
  amount: uint256,
  sender: string,
  receiver: string,
  source: boolean,
  destPort: string,
  destChannel: string,
  sourcePort: string,
  sourceChannel: string,
  timeoutHeight: Height,
  timeoutTimestamp: uint64) {
  prefix = "{sourcePort}/{sourceChannel}/"
  // we are the source if the denomination is not prefixed
  source = denomination.slice(0, len(prefix)) !== prefix
  if source {
    // determine escrow account
    escrowAccount = channelEscrowAddresses[sourceChannel]
    // escrow source tokens (assumed to fail if balance insufficient)
    bank.TransferCoins(sender, escrowAccount, denomination, amount)
  } else {
    // receiver is source chain, burn vouchers
    bank.BurnCoins(sender, denomination, amount)
  }
  FungibleTokenPacketData data = FungibleTokenPacketData{denomination, amount, sender, receiver}
  handler.sendPacket(Packet{timeoutHeight, timeoutTimestamp, destPort, destChannel, sourcePort, sourceChannel, data}, getCapability("port"))
}
```

onRecvPacket 当收到发送给该模块的数据包时，由路由模块调用。

```typescript
function onRecvPacket(packet: Packet) {
  FungibleTokenPacketData data = packet.data
  // construct default acknowledgement of success
  FungibleTokenPacketAcknowledgement ack = FungibleTokenPacketAcknowledgement{true, null}
  prefix = "{packet.sourcePort}/{packet.sourceChannel}/"
  // we are the source if the packets were prefixed by the sending chain
  source = data.denomination.slice(0, len(prefix)) === prefix
  if source {
    // receiver is source chain: unescrow tokens
    // determine escrow account
    escrowAccount = channelEscrowAddresses[packet.destChannel]
    // unescrow tokens to receiver (assumed to fail if balance insufficient)
    err = bank.TransferCoins(escrowAccount, data.receiver, data.denomination.slice(len(prefix)), data.amount)
    if (err !== nil)
      ack = FungibleTokenPacketAcknowledgement{false, "transfer coins failed"}
  } else {
    prefix = "{packet.destPort}/{packet.destChannel}/"
    prefixedDenomination = prefix + data.denomination
    // sender was source, mint vouchers to receiver (assumed to fail if balance insufficient)
    err = bank.MintCoins(data.receiver, prefixedDenomination, data.amount)
    if (err !== nil)
      ack = FungibleTokenPacketAcknowledgement{false, "mint coins failed"}
  }
  return ack
}
```

onAcknowledgePacket是由路由模块在该模块发送的数据包被确认时调用。

```typescript
function onAcknowledgePacket(
  packet: Packet,
  acknowledgement: bytes) {
  // if the transfer failed, refund the tokens
  if (!ack.success)
    refundTokens(packet)
}
```

onTimeoutPacket是由路由模块调用的，当该模块发送的数据包已经超时（比如他不会在目标链上被接收）

```typescript
function onTimeoutPacket(packet: Packet) {
  // the packet timed-out, so refund the tokens
  refundTokens(packet)
}
```

refundTokens在失败时由onAcknowledgePacket和onTimeoutPacket调用 ，以将托管的代币退还给原始发件人。

```typescript
function refundTokens(packet: Packet) {
  FungibleTokenPacketData data = packet.data
  prefix = "{packet.sourcePort}/{packet.sourceChannel}/"
  // we are the source if the denomination is not prefixed
  source = denomination.slice(0, len(prefix)) !== prefix
  if source {
    // sender was source chain, unescrow tokens back to sender
    escrowAccount = channelEscrowAddresses[packet.srcChannel]
    bank.TransferCoins(escrowAccount, data.sender, data.denomination, data.amount)
  } else {
    // receiver was source chain, mint vouchers back to sender
    bank.MintCoins(data.sender, denomination, data.amount)
  }
}
```

```typescript
function onTimeoutPacketClose(packet: Packet) {
  // can't happen, only unordered channels allowed
}
```

#### 推理

##### 正确性

这种实现方式既保持链可替代性和供应。

可替代性：如果代币被发送到对手链上，它们可以在源链上以相同的面额和金额被赎回。 供应：将供应重新定义为解锁的代币。所有send-recv对总和为净零。源链可以改变供应。

##### 多链笔记📒

该规范没有直接处理"钻石问题"，即用户将源自链A的代币发送到链B，然后到链D，并希望通过D -> C -> A返回，由于供应被追踪为链B拥有（并且命名为
"{PortOnD}/{ChannelOnD}/{PortOnB}/{ChannelOnB}/denom")，链C 不能作为中介。目前尚不清楚这种情况是否应该在协议内处理--只需要赎回的原始
路径就可以了（如果两条路径都存在频繁的流动性和一些盈余，diamond路径在大多数情况下都会发挥作用）。长期的赎回路径所带来的复杂性可能会导致 网络拓扑中心链的出现。

为了跟踪所有在不同路径的连锁网络中移动的面额，对某一特定连锁实施注册可能是由帮助的，它将跟踪每种面额的'全局"源连锁。最终用户服务提供者（比如钱包作者）
可能希望集成这样的注册中心，或者保留自己的规范源链和人们可读的名称映射，以便改进用户体验。

#### 可选附录

- 每个链，本地，可以选择保持一个查找表，在状态下使用简短的、用户友好的本地面值，在发送和接收数据包时被翻译成较长的面额。
- 可以对那些其他机器可以被连接懂啊和那些通道可以被建立施加额外的限制。

## 向后兼容

不实用

## 向前兼容

这个最初的标准在通道握手中使用版本“ics20-1”。

该标准的未来版本可以在通道握手中使用不同的版本，并安全地改变数据包数据格式和数据包处理程序语义。

## 示例实现

即将到来

## 其他实现

即将到来

## 历史

2019年7月15日-草稿

2019年7月29日-重大修订; 清理

2019年8月25日-重大修订，更多清理工作

2020年2月3日-处理承认成功和失败的修订

2020年2月24日-修订源字段推断，包含版本字符串

2020年7月27日-源场的重新加入

## 版权

所有内容均在Apache 2.0许可

