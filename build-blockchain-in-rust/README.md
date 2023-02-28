# Build block chain by Rust

- 定义块， 实现链
  - 持久化块 kv-db
  - 定义交易，交易入块
- 网络层，节点ID，网络管理
  - 发送交易算法，交易池
  - 出块算法（交易定序），发送块，块校验，共识算法
- 不需要wasm on-chain runtime , eightFish 链上逻辑固化在代码中，
  - 定义EF相关交易类型，状态直接存储在kv-DB, 交易的执行MPT？ 需要方便算 state_root
- 实现（双向）RPC子系统， 抛出Event，实现RPC client
- 不需要链上账户系统，（最多需要admin管理， 参考传统db实现）
  - 账户系统实现在wasm worker中
- 加强密码学管理，提供链上，链上随机数
- 各种运维相关事物，
- 性能优化
