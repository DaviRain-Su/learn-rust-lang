# Untapped potential in Rust's type system

> > Rust 类型系统中未开发的潜能

今天，我将介绍除了检查代码属性之外还可以使用哪些类型。它将涉及大量的动态类型，是的，它是在 Rust 中。里面有一些疯狂的想法，所以系紧你的安全带准备好去兜风吧！

## Overview

文章分为导言、背景、包含主要内容的三个部分和结论。中间的三个部分各自涵盖了自己的想法，有独立的动机。连接起来的是应用运行时类型评估的方式。在这方面，它们是建立在彼此之上的。

- Overview
- Introduction
- Background: Dynamic Types in Rust
- Section 1: A Heterogenous Collection of Singletons
    - Storing heterogenous data
    - Real world applications and AnyMap
- Section 2: Type-Oriented Message Passing
    - What I want to achieve and why
    - Implementation
    - More about Nuts
    - Comparison to existing implementations
- Section 3: Universal Type IDs
    - Dirty secrets about TypeId
    - Why I’m not using TypeId
    - Deriving my own Type ID
    - How would this be used?
- Final Thoughts

## Introduction

类型是一个非常抽象的概念。它们到底是什么？对我来说，答案在很大程度上取决于编程语言和讨论的一般背景。

当我在C++中写第一行程序代码时，类型对我来说只是定义一个变量的东西。随着我在C++和Java中的实践越来越多，类型在我心中基本上等同于类或基元。但无论如何，我并没有过多地考虑类型问题。它们只是为了让编译器高兴而必须的。

扩展到JavaScript，我意识到类型也可以被隐藏在后台。在这种情况下，它们必须是正确的以使运行时满意，而运行时似乎比编译器更宽容。另一方面，我讨厌在运行时才出现的错误，我知道编译器之前可以告诉我。

然后，我学习了Haskell。类型成了一个完全不同的概念。似乎整个程序都可以用类型系统本身来写。我对此印象深刻。

在所有这些之后，我学习了Rust。我喜欢Rust的强类型的感觉。与C和C++相比，Rust去除了它们中最令人沮丧的部分。忘记初始化变量不再可能，空指针也不复存在，内存管理也变得非常有趣。

快进到今天。Rust向我展示了几个全新的概念，可以通过其聪明的类型系统来实现。生命周期将内存管理方面的问题纳入了类型内部。`&mut`和`&`类型之间的区别定义了是否允许别名。而在某种程度上，实现`Future`特性的类型描述了整个有限状态机。

但今天我想谈谈Rust中的运行时类型评估。我遇到了一些实际的编程问题，如果没有一些（安全的）downcasts，我是无法解决的。然后我把它带到了动态类型的极端水平，这是我没有想到的。在这个过程中，我不得不再次重新考虑类型到底是什么。由于我发现结果相当有趣和令人惊讶，我想在这篇文章中分享。

## Background: Dynamic Types in Rust

在一些语言中，每个（非原始）值的类型都被嵌入到机器代码中。这就像一个隐含在每个对象中的隐藏字段。这是实现动态类型的一种方式。但是Rust并不包括每个值的类型信息开销。

然而，Rust提供了手动存储类型信息的方法，这也可以在运行时使用。有可能将一个静态已知类型的值转化为一个胖指针，将该值与一个特征的虚拟函数表（vtable）相结合。这些胖指针被称为[trait object](https://doc.rust-lang.org/reference/types/trait-object.html)。

trait object 本质上提供了选择的运行时类型信息。但是它们的力量是相当有限的，因为它们只允许访问一个特定的特征和它的父特征的功能。为了知道我们是否在处理一个特定的类型，还需要一个技巧。

只需使用[核心标准库](https://doc.rust-lang.org/std/any/index.html)中的工具，我们就可以要求编译器提供任何类型的TypeId，并在运行时存储起来供我们自己使用。然后编译器会在那里放一个唯一的常数作为类型ID。

下面是类型ID是如何创建的。

```rust
use core::any::{Any, TypeId};
fn main() {
    let one_hundred = 100u32;
    // Get the type ID usaing a value of that type.
    let t0 = one_hundred.type_id();
    // Get the type ID directly
    let t1 = TypeId::of::<u32>();
    assert_eq!(t0, t1)
}
```

有两种变体显示，一种是有一个类型的值，另一种是只有类型作为通用参数。两者在源代码层面上都是函数调用。但是编译器应该把它们优化掉，在它们的位置上放一个常量值。

然后，TypeId值可以在运行时被用来做三件事。我们可以将它与另一个TypeId进行比较，它可以被用作哈希键，我们可以为调试目的打印ID，这只是显示一个随机的整数值。但是我们不能做其他的事情，比如说查询该类型ID的特征是否被实现。

下面是如何使用类型ID来模拟动态类型检查的。
```rust
fn count_rectangles(shapes: &[Box<dyn Shape>]) -> usize {
    let mut n = 0;
    for shape in shapes {
        // Need to derefernce once or we will get the type of the Box!
        let type_of_shape = shape.deref().type_id();
        if type_of_shape == TypeId::of::<Rectangle>() {
            n += 1;
        } else {
            println!("{:?} is not a Rectangle!", type_of_shape);
        }
    }
    n
}
```
type_id()方法是在Any属性上定义的，它对任何类型都有一个全面的实现，这并不奇怪。(对类型有一个小的限制，但这不在本文的讨论范围之内）。

当我们使用 dyn Any 的特质对象时，真正的动态类型化就开始了。它可以执行所谓的checked downcast，从一个一般类型到一个更具体的类型。(参见官方文档中的 downcast_ref 和 downcast）。

下面是一个使用例子。
```rust
fn remove_first_rectangle(shapes: &mut Vec<Box<dyn Any>>)
    -> Option<Box<Rectangle>>
{
    let idx = shapes
        .iter()
        .position(|shape| shape.deref().type_id() == TypeId::of::<Rectangle>())?;
    let rectangle_as_unknown_shape = shapes.remove(idx);
    rectangle_as_unknown_shape.downcast().ok()
}
```
不过，这里的降序并不神奇。如果我们想手动实现它（没有编译器的帮助），我们也可以检查类型ID是否符合我们的期望，然后用[transmute](https://doc.rust-lang.org/std/mem/fn.transmute.html)调用来跟进。

但现在背景已经足够了。让我们在下面的三个部分中对这些概念进行创新吧

## Section 1: A Heterogenous Collection of Singletons
本节展示了像这样的魔法如何在Rust中工作以及为什么它很重要。
```

// Putting two different types in the same collection, with no keys.
collection.set( 3.14 );
collection.set( 888 );

// Taking out the values of the two types again, 
// automatically getting the value of the correct type
assert_eq!( 3.14, *collection.get::<f32>() );
assert_eq!(  888, *collection.get::<u32>() );
```
### Storing heterogenous data
Rust中的大多数集合是同质的，也就是说，它们存储的对象都是同一类型的。例如，Vec<f32>只存储浮点数。但是我们可以通过使用指向特质对象的指针使其成为单向异质的。

例如，Vec<Box<dyn ToString>存储一个指针集合。这个向量可以接受的指针类型包括Box<f32>、Box<u64>和许多其他类型。因此，我们可以放进去的数据类型是异质的。但我们得到的只是一个指向特质对象的指针（Box<dyn ToString>），内部值的实际类型无法恢复。

为了拥有一个完全异质的集合，getter-方法应该能够返回不同类型的对象。这在动态类型的语言中是微不足道的，比如Python或JavaScript。然而，在静态类型语言中，一个函数只能返回一种特定的类型，如函数签名所定义的。

作为一个简单的方法，具有子类型的语言通常有一个最一般的类型，它是所有其他类型的超级类型。例如，Java中的Object是所有类的超级类型。这可以在函数签名中用来定义返回类型。然后，调用者可以对返回的值进行下转换。

在Rust中，dyn Any类型的trait对象可以被认为是最通用的类型。它是唯一的类型，（几乎）所有其他类型都可以被胁迫为它。正如在背景部分所解释的，Any也是（唯一的）允许下移的特质。因此，我们可以在getter方法中返回&Box<dyn Any>，而调用者可以进行下转换。

不过直接返回Box<dyn Any>并不是一个好的接口。为了避免在调用者一方手动下转换，可以把它隐藏在一个泛型函数后面。这里有一个完整的例子。

```rust
use core::any::*;
use std::collections::HashMap;

fn main() {
    let mut collection = HeteroCollection::default();
    collection.set("f32", 3.14f32);
    collection.set("f64", 2.71f64);
    collection.set("another f32", 1.618f32);
    
    let f32_output = *collection.get::<f32>("f32").unwrap();
    assert_eq!( 3.14, f32_output);
}

#[derive(Default)]
struct HeteroCollection {
    data: HashMap<&'static str, Box<dyn Any>>,
}
impl HeteroCollection {
    fn get<T: 'static>(&self, key: &'static str) -> Option<&T> {
        let unknown_output: &Box<dyn Any> = self.data.get(key)?;
        unknown_output.downcast_ref()
    }
    fn set<T: 'static>(&mut self, key: &'static str, value: T) {
        self.data.insert(key, Box::new(value));
    }
}
```
上面的代码基本上模拟了一个Python字典。任何键都可以容纳任何类型。调用者必须确保键和类型匹配。

这里有一个疯狂的想法，我们让编译器做这个检查怎么样？下面是一个实现，它就是这样做的。
```rust
use core::any::*;
use std::collections::HashMap;

struct SingletonCollection {
    data: HashMap<TypeId, Box<dyn Any>>,
}
impl SingletonCollection {
    pub fn get<T: Any>(&self) -> &T {
        self.data[&TypeId::of::<T>()]
            .downcast_ref()
            .as_ref()
            .unwrap()
    }
    pub fn set<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }
}
```
通过这种方法，通用类型充当钥匙。因此，这将集合限制在每个类型的单一元素上。但在许多情况下，这并不是一种限制。新的类型很便宜! 正如下面的片段所展示的，比较之前和之后。
```rust
/// Before
collection.set("name", "Jakob");
collection.set("language", "Rust");
collection.set("dominant hand", DominantHand::Right);

let name = collection.get::<&'static str>("name");
let language = collection.get::<&'static str>("language");
let dominant_hand = collection.get::<DominantHand>("dominant hand");

// After
collection.set(Name("Jakob"));
collection.set(Language("Rust"));
collection.set(DominantHand::Right);

let name = collection.get::<Name>().0;
let language = collection.get::<Language>().0;
let dominant_hand = collection.get::<DominantHand>();

// For completeness: Type Definitions
struct Name(&'static str);
struct Language(&'static str);
enum DominantHand {
    Left,
    Right,
    Both,
    Neither,
    Unknown,
    Other,
}
```

唯一的功能区别是，类型键必须在编译时知道，而字符串可以在运行时确定。这一点目前还没有问题。在第三节中，我将展示一种方法来绕过这个限制。

在语法上，有一点烦人，因为必须为每个键定义一个新的类型。但我个人认为，这并不比维护一个 "魔法字符串 "的列表差。反正它们最终可能会成为独立的常量，这也是一行模板代码。

类型-键的好处是编译器可以检查键是否有效，以及存储的值是否符合要求的类型。

### Real world applications and AnyMap

现在要问的是，我们什么时候会想使用单子异质集合？也许最常见的用法是在一个希望管理由库用户定义的一般状态的库中。

在这种情况下，这种模式就很方便了，因为它允许用户存储任意多的任何类型的对象。而库可以管理它们，甚至不需要知道它们的类型。第二节也会有一些这方面的好例子。

然而，值得注意的是，我并没有发明这种模式。事实上，它被广泛使用。我想我第一次看到它是在[Amethyst/Shred](https://github.com/amethyst/shred)的[struct world](https://github.com/amethyst/shred/blob/63778ae268970b7526f74fca7ec6e0364a7514c9/src/world/mod.rs)中。

在写这篇文章的时候，我深入研究了一下，发现[Chris Morgan](https://chrismorgan.info/)将这种模式包装在一个通用的集合[AnyMap](https://github.com/chris-morgan/anymap)中。在写这篇文章的时候，这个板块已经有超过130万次的下载。我想说的是，这属于广泛使用。

所以，类型可以作为键来使用，而且社区已经在这样做了。为了发掘未开发的潜力，让我们在下一节看一下除此之外的机会。

## Section 2: Type-Oriented Message Passing
在本节中，我们将看到一些基于类型的动态调度。不是基于名字和类型的动态调度，不是，是只基于类型的调度。另外，即使是对象也将根据其类型进行动态查找，这意味着调用者甚至不需要访问对象！我将向你展示的是面向对象的方法。

我将向你展示的东西可以被描述为面向对象的消息传递，其特点是类型被用作对象地址，同时也用于动态调度。

但让我清楚地说明一下这里的术语。我指的是[面向对象编程（OOP）](https://en.wikipedia.org/wiki/Object-oriented_programming)的一般概念，它不需要类。我所使用的只是对象和方法。

此外，在这种情况下，[消息传递](https://en.wikipedia.org/wiki/Message_passing)是在一个对象上调用方法的一个特定术语。从本质上讲，一个带有方法标识符和参数值的消息被发送到一个对象，该对象在内部分配并执行。

这个过程可以在Rust中实现，而动态类型就非常方便了。

### What I want to achieve and why

去年，我写了一个关于我通过WASM在浏览器中运行Rust所面临的问题。(见[Rust遇到网络--编程范式的冲突](https://www.jakobmeier.ch/blogging/Rust_on_the_Web.html))

长话短说，这归根结底是线程在浏览器中不能连续运行。取而代之的是，闭包必须被注册，以便在一个时间间隔内被调用。在这些线程之间共享数据会变得很麻烦，我在那篇文章中描述了这一点。

下面是一个人为的例子，说明了浏览器的代码如何使用回调闭包。

```rust
fn main() {
    let window = get_window_from_browser();
    let body = get_body_from_browser();
    let state = MyDummyState::new();

    window.set_interval(
        100,
        move || {
            // do something every 100ms
            state.update();
        }
    );
    body.on_click(
        move |x,y| {
            // do something on every click
            state.apply_click(x,y);
        }
    );
}
```
这个例子不能编译。(即使在假设所有的函数都有正确的签名的情况下）。问题在于，状态被移到了两个闭包里面，这是不可行的。借用而不是移动也是不行的，因为闭包比主函数使用的当前堆栈帧要长。

为了解决这个问题，我必须把数据放在一个共享的智能指针后面，比如Arc<>，然后引入内部可变性。这很烦人，我希望有一个更好的方法。

当我写文章抱怨这些问题的时候，我并没有真正解决这个问题，我只是指出了它。但是到现在，我想我已经有了一个令人满意的解决方案，我已经用了很多个月了。

正如前面所暗示的，我最终想出的解决方案涉及到全局存储单子对象，并对这些方法进行动态方法注册和动态调度。让我给你看一些代码，希望它能使事情变得更清楚一些。

```rust
struct MyObject {
    counter: u32,
}
struct MethodA;
struct MethodBWithArguments {
    text: String,
}
impl MyObject {
    fn method_a(&mut self, _arg: MethodA) {
        self.counter += 1;
        println!(
            "Object invoked a method {} times. This time without an argument.",
            self.counter
        );
    }
    fn method_b(&mut self, arg: MethodBWithArguments) {
        self.counter += 1;
        println!(
            "Object invoked a method {} times. This time with argument: {}",
            self.counter, arg.text
        );
    }
}

fn main() {
    /* registration */
    let obj = MyObject { counter: 0 };
    my_library::register_object(obj);
    my_library::register_method(MyObject::method_a);
    my_library::register_method(MyObject::method_b);

    /* invocations */
    my_library::invoke::<MyObject, _>(MethodA);
    my_library::invoke::<MyObject, _>(MethodBWithArguments {
        text: "Hello World!".to_owned(),
    });

    /* Output */
    // Object invoked a method 1 times. This time without an argument.
    // Object invoked a method 2 times. This time with argument: Hello World!
}
```

这里发生的事情是，我注册了一个对象（obj）和它的方法到my_library的全局管理状态。之后，我在该对象上调用方法，而没有实际引用obj。这是有可能的，因为my_library有全局存储。

全局存储只保留每种类型的一个对象。(它在内部使用一个异质的单子集合。)因此，只要指定了类型，应该被调用的对象就是已知的。

这在使用闭包作为回调时变得非常有用。我们现在可以有许多不同的回调，这些回调都调用一个共享对象上的方法，而不必真正担心数据共享部分。

```rust
fn main() {
    // ...

    div.on_click(
        || {
            my_library::invoke::<MyObject>(MethodBWithArguments{
                test: "Clicked something!".to_owned(),
            }
        }
    );
}
```
因此，我在一个叫做[Nuts](https://github.com/jakmeier/nuts)的库中实现了这个（以及更多）。在实际的库中，命名是有点不同的。比如说，对象被称为活动。这只是因为在我开始写这篇文章的第三次尝试之前，我并没有把它当成对象和方法。

关于目标是什么，就说这么多。现在的挑战是如何实现my_library的功能。

### Implementation

为了实现我刚才介绍的接口，我们需要一堆隐藏在后台的全局状态来存储对象和方法。我们不要担心全局状态是如何存储和提取的。为了保持对动态类型的关注，我们只是假设 register_object 和 register_method 方法是在 Nut 对象上调用的。如果你想自己运行它，Playground包含了使之工作的胶水。

在这个假设下，Nut里面应该有什么？让我们从一个存储对象的集合开始。
```rust
pub struct Nut {
    objects: HashMap<TypeId, Box<dyn Any>>,
}
```
这正是我在第1节向你展示的SingletonCollection。一个可以容纳不同对象的集合，以其类型为索引。

有了这种状态，我们已经可以实现register_object。
```rust
impl Nut {
    fn register_object<OBJECT>(&mut self, obj: OBJECT)
    where
        OBJECT: Any,
    {
        let key = TypeId::of::<OBJECT>();
        let boxed_obj = Box::new(obj);
        self.objects.insert(key, boxed_obj);
    }
}
```
对于方法来说，它变得更加棘手。我们需要存储任意数量的具有不同类型的方法。为了将它们存储在一个集合中，我们需要找到一个涵盖所有这些方法的通用特征对象。

Box<dyn Any>可以用来存储它们。但是我们需要在以后调用这些方法。这就需要对实际类型进行下转换。

说实话，这可能是可以这样做的。但是如果我们存储可调用的函数指针，我们的生活就会变得更容易。我们只需要找到一个足够普遍的可调用类型。

首先，我们必须从特性Fn、FnOnce和FnMut中挑选一个作为我们的基础特性。[FnMut](https://doc.rust-lang.org/nomicon/hrtb.html)是其中最通用的，我们将使用它来不限制用户。(你可以在FnMut的文档中了解它们之间的区别，也可以在Rustonomicon的[Higher-Rank Trait Bounds](https://doc.rust-lang.org/nomicon/hrtb.html)一章中了解它们到底是什么）。

接下来，参数是什么？每个方法都会有一个可被借用的对象作为第一个参数（&mut self），还有一些参数结构作为第二个参数。因此，我们可以尝试FnMut(&mut dyn Any, dyn Any)这样的方法。

但是像这样按值传递特质对象是行不通的，因为dyn Any的大小是未知的。至少对于第二个参数，我们需要把它包在一个盒子里。因为我们无论如何都要存储对象的盒子，所以我们也要把第一个参数包起来。这让我们看到了FnMut(&mut Box<dyn Any>, Box<dyn Any>)。

最后，我们必须把它放在一个哈希图中。哈希图的值是 FnMut 特质的一个特质对象，所以它必须被包装成另一个 Box。

哈希图的键应该是两种类型的组合，（TypeId,TypeId）。第一个类型ID是对象的，第二个是方法参数的。这允许为每个对象存储许多方法。而查询仍然只有一个哈希值。

把这一切放在一起，Nut结构看起来是这样的。

```rust
pub struct Nut {
    objects: HashMap<TypeId, Box<dyn Any>>,
    methods: HashMap<(TypeId, TypeId), Box<dyn FnMut(&mut Box<dyn Any>, Box<dyn Any>)>>,
}
```
哇，这是一个可以吓跑任何读者的类型定义。但是，请你忍耐一下吧! 好的是，现在调用这些方法只是一个简单的三个步骤。

- 查阅对象。
- 查找方法。
- 用对象和调用参数调用该方法。
或者，用代码包装一下，它看起来像这样。
```rust
pub fn invoke<OBJECT, ARGUMENT>(&mut self, arg: ARGUMENT)
where
    OBJECT: Any,
    ARGUMENT: Any,
{
    let object_key = TypeId::of::<OBJECT>();
    let method_key = (TypeId::of::<OBJECT>(), TypeId::of::<ARGUMENT>());
    if let Some(obj) = self.objects.get_mut(&object_key) {
        if let Some(method) = self.methods.get_mut(&method_key) {
            method(obj, Box::new(arg));
        }
    }
}
```
方法键被构建为对象和参数类型的一个元组。然后，我们检查对象和方法是否存在，如果两个查询都成功了，就简单地调用它。

在这个过程中，参数类型基本上是用来进行动态调度的，因为它选择了对象上的方法。

接下来是最难的部分，把方法放在集合里面。编译器并不只是使用特质对象而不是特定类型将一个方法自动转换为它的对应部分。不幸的是，这里没有强制执行的情况。

我们需要一个围绕注册方法的包装闭包。通过将下转换代码移到包装闭包内，这个新的闭包可以是我们先前得出的更通用的类型，而内部方法则保留了原始签名。下面是相关的代码。

```rust
pub fn register_method<OBJECT, ARGUMENT, FUNCTION>(&mut self, mut method: FUNCTION)
where
    FUNCTION: FnMut(&mut OBJECT, ARGUMENT) + 'static,
    ARGUMENT: Any,
    OBJECT: Any,
{
    let key = (TypeId::of::<OBJECT>(), TypeId::of::<ARGUMENT>());
    let wrapped_method =
        Box::new(move |any_obj: &mut Box<dyn Any>, any_args: Box<dyn Any>| {
            let obj: &mut OBJECT = any_obj.downcast_mut().expect("Type conversion failed");
            let args: ARGUMENT = *any_args.downcast().expect("Type conversion failed");
            method(obj, args)
        });
    self.methods.insert(key, wrapped_method);
}
```
这里有很多东西需要解读。让我们从函数签名开始。

确切的通用类型约束在这里是相当有趣的。我们有三个类型参数来描述我们接受的允许的函数和闭包。在这里，我们没有在界面上设置任何方框，因为这不是我们想让用户关心的事情。

如果你想知道特质约束中的'static'是干什么用的，这是对函数类型的寿命的必要约束。如果你从未见过这样的约束，不要担心，这并不重要，我宁愿让你的注意力放在我在这里向你展示的更广泛的概念上。(但如果你一定要知道，请随时查看Playground上的错误信息，如果你去掉了绑定，并从那里追踪。🙂️)

接着说说正文。在 invoke 中，键的构造与之前一样。然后，键和方法被移到一个盒式闭包内。

这个闭包再次有一个签名和一个主体。签名必须与Nut中方法字段的定义完全匹配。因此，它包括方框，并且只有特质对象而不是特定类型。

在闭合体中，我们只需执行两个下转换，并调用所提供的方法。请注意，这里的下转换不应该失败，因为 invoke 方法按类型查找方法，因此总是提供正确底层类型的 Any trait 对象。

噗，你已经成功了! 结合所有的片段，前面的my_library接口得到了100%的支持。如果你想看看它的操作，这里有一个Playground链接。

有了这种方法，我就能接受到处使用回调的事件驱动的浏览器世界了。任何注册的对象都可以从任何地方访问，包括从闭包内访问。

### More about Nuts
我在前面提到的Nuts库，不仅仅涵盖了我到目前为止向你展示的案例。这个概念还可以进一步扩展到一个完整的发布-订阅库。这样就可以在不知道哪个对象有这样一个方法的情况下发送一个方法调用，如下图所示。

```rust
struct A;
struct B;
pub fn main() {
    /* registration */
    let a = nuts::new_activity(A);
    let b = nuts::new_activity(B);

    a.subscribe(|&mut A, msg: &&'static str| println!("A received: {}", msg));
    b.subscribe(|&mut B, msg: &&'static str| println!("B received: {}", msg));

    /* invocations */
    nuts::publish("Hello World");

    /* Output */
    // A received: Hello World
    // B received: Hello World
}
```
这里有几件事是不同的。首先，对象被称为活动，方法被称为订阅。第二，当注册一个活动时，会返回一个活动的ID，注册订阅只对这样的ID有效。在我看来，这使得API更加简洁，因为以前，一个方法可以在没有对象的情况下被注册，这不应该发生。

最后，现在有了publish，而不是invoke，它不需要接收者的类型参数。Nuts内部保留了一个监听每种消息类型（本例中为&'static str）的订阅列表，当这种消息被发布时，会全部调用它们。

因此，对发布的单一调用会导致多个订阅被调用。因此，订阅只得到一个借用的值来工作。(对于&'static str作为消息类型，这导致了奇怪的双借&&)。

这种泛化使得Nuts更像是一个发布-订阅库。但Nuts仍然支持自有的数据传输，以[私有通道](https://docs.rs/nuts/0.2.1/nuts/struct.ActivityId.html#method.private_channel)的名义进行。然后，调用必须使用类似send_to::<Receiver>(msg)的语法，以明确哪个对象应该接收该消息。

但要真正解决我最初遇到的问题，Nuts需要做得更多。在一个对象上调用一个方法已经很不错了，但有时数据还需要在活动之间共享。因此，Nuts支持在[域](https://docs.rs/nuts/0.2.1/nuts/struct.DomainState.html)中对活动进行分组。每个域都有一个单子集合，正如第一节中介绍的那样。订阅处理程序可以访问这个集合，而且是可变的。

这使得他们可以共享任意的状态。下面是一个例子
```rust
let a = nuts::new_domained_activity(A, &nuts::DefaultDomain);
let b = nuts::new_domained_activity(B, &nuts::DefaultDomain);

nuts::store_to_domain(&nuts::DefaultDomain, 0u32);
nuts::store_to_domain(&nuts::DefaultDomain, "This is Nuts!");

a.subscribe_domained(|_, domain, _msg: &()| {
    let counter = domain.get_mut::<u32>();
    *counter += 1;
    println!("A counts to {}", counter);
});
b.subscribe_domained(|_, domain, _msg: &()| {
    let counter = domain.get_mut::<u32>();
    *counter += 1;
    println!("B counts to {}", counter);
});
b.subscribe_domained(|_, domain, _msg: &()| {
    let message = domain.get::<&'static str>();
    println!("B reports message: {}", message);
});

nuts::publish(());

/* Output */
// A counts to 1
// B counts to 2
// B reports message: This is Nuts!

```
这个例子使用了单元类型（）作为消息，它作为一个主题来听，效果很好。而且它使用了与活动交互的所有方法的域化版本。在注册时，这意味着我们还必须提供该活动应该属于哪个域。为了保持简单，两个活动都使用默认域。

然后，我们可以使用nuts::store_to_domain来存储一些值。这将它们放在与该域相关的单子集合中，该集合被作为第二个参数提供给用 subscribe_domained 注册的回调。

结合所有这些特性，我三次使用类型作为哈希图的关键。对于活动，对于订阅，以及对于域。如果没有Rust在core::any中提供的倒置功能，这一切都不可能实现（至少不安全）。

对我来说，经历这一切值得吗？是的，值得。[Paddlers](https://github.com/jakmeier/paddlers-browser-game/)的代码已经变得非常干净了。在许多其他的好处中，它允许我实现对来自浏览器的用户输入的抽象，并将其准确地转发给那些对它们感兴趣的活动。不幸的是，Rust和浏览器的交互还有更多的问题，但这是另一个话题了。

### Comparison to existing implementations

在我为这篇文章所做的研究中，我发现我并不是第一个有想法将异质函数存储在哈希图中并通过其参数调用的人。[QuietMisdreavus](https://github.com/QuietMisdreavus)已经以[handler_map](https://crates.io/crates/handler_map)的名义发布了crate。

虽然handler_map采取了保守的方法，只调用函数，但我对这个概念更加疯狂了。我还动态地存储对象，然后在这些对象上分派方法，而不是简单地调用函数。

另一个有趣的发现是名为Eventbus的板块。它缺乏文档，但我从代码中读到，事件的共享类似于Nuts中消息的发布方式。但也有一些关键的区别。

在功能上，最大的区别是[Eventbus](https://crates.io/crates/eventbus)中的每个处理程序都可以修改事件/消息，后续的处理程序会看到这些变化。而且与handler_map类似，Eventbus中的处理程序没有像Nuts中的对象那样的状态。在语法层面上，Eventbus使用宏（register_hook！和post_event！），而Nuts使用普通的函数调用就可以了。

在这一点上，我还想简单地将Nuts与[actor模型](https://en.wikipedia.org/wiki/Actor_model)区分开来。

- Nuts中的活动（对象）可以共享状态，这与actor的常规不同。
- Nuts中的方法总是按顺序执行的。而不是典型的行为体的并发执行。
- 为了与其他行为体进行通信，行为体通常需要明确地获得接收者的地址。要么是作为父/子依赖关系，要么是通过接收消息中的地址。在Nuts中，地址是接收者的Rust类型，因此不需要明确的设置就可以获得。

这些是我不认为Nuts是一个角色系统的主要原因，尽管它类似于使用消息传递来定义程序流。

第二节到此结束。接下来，我们将看看作为core::any::TypeId的泛化的通用类型ID。

## Section 3: Universal Type IDs
第1节和第2节展示了类型ID在一个单一的二进制中是如何有用的。它们允许我们编写不知道具体类型的库代码。然后库的用户在编译时定义这些类型。

但是，如果我们想把类型ID带到二进制边界之外呢？如果类型在编译时根本就不知道呢？

你看，我有一个梦想，那就是像Nuts那样的API可以被用于网络系统。端点可以注册和调用远程程序，就像我在第二节中注册和调用对象的方法一样。

我承认，还有其他方法可以实现功能相当的系统。我想到了1998年诞生的[SOAP](https://en.wikipedia.org/wiki/SOAP)，它是一种在机器之间共享类型化对象的标准化方式。还有许多更现代的替代方法存在。(请原谅我没有把它们都列在这里。）然而，它们都没有在本地的Rust类型上操作！这就是那种疯狂的想法。这就是我带来的那种疯狂的想法。这就是有趣之处。

公平地说，具有本地Rust定义的远程过程调用（RPC）也已经存在。有了[tarpc](https://github.com/google/tarpc)，RPC接口可以在纯Rust代码中指定，这与我想实现的目标很接近。

但我的系统并不只是用于点对点的RPC。它将是一个动态的发布-订阅系统，就像第二节中的Nuts一样，但这次是联网的。和Nuts一样，节点之间的路由将完全基于编译时的类型，不会有一个由弱类型字符串定义的单层，就像一般REST API中的URI。

但是，等一下。如果类型必须在二进制文件编译时就知道，这怎么会有任何灵活性呢？好吧，我想做的关键保证是，端点可以安全地被重新编译和更新。所有没有被改变的类型应该仍然与旧的二进制文件兼容。
最后，为了实现我的梦想，我需要一种方法来比较独立编译的二进制文件之间的类型。我可以使用TypeId吗？

### Dirty secrets about TypeId
在写这篇文章的时候，TypeId只是一个私有u64的包装器。那个整数值是由一个哈希值构建的，在编译器的中间端执行。(链接到源代码)

我想知道对一个结构的改变究竟会影响其TypeId值。我不觉得对编译器代码挖掘得太深，只是测试了一些东西。这里列出了一些确实会改变数值的事情。

- 重命名结构
- 重命名字段
- 将定义移到另一个模块
- 语法改变（例如，将MyType{}改为MyType）

另一方面，这些事情不会改变TypeId。
- 改变一个字段的类型
- 在植入块中或通过#[derive(..)]添加方法
然而，编译器团队也可以在每次更新时自由改变哈希结构。
下面是TypeId的官方文档中的一段话。
> While TypeId implements Hash, PartialOrd, and Ord, it is worth noting
that the hashes and ordering will vary between Rust releases. 
Beware of relying on them inside of your code!

实际上，在不久的将来出现重大变化也不是不可能的。目前，Rust最[古老的未解决的健全性问题](https://github.com/rust-lang/rust/issues/10389)是这些哈希值可能（理论上）发生碰撞的事实。一个增加整数大小的[拉动请求](https://github.com/rust-lang/rust/pull/75923)在几个月前就已经被讨论并拒绝了。

### Why I’m not using TypeId

考虑到这一切，我不能真的使用TypeId。

我意识到，TypeId根本不能很好地反映我的使用情况。它被设计成在一个单一的二进制中使用，而不是像我所设想的那样在许多人之间共享。

换句话说，TypeId在静态代码库的一组类型中是唯一的。除此之外，"类型 "一词的含义并没有很好的定义。但是一个不断变化的代码库正是我想要处理的情况。

例如，我可能有一个由struct A { counter: i32}定义的类型，后来决定它应该是struct A { counter: i64}。在这种情况下，在当前的编译器版本中，标准的TypeId不会改变。但对我来说，这是两个不兼容的类型。而这两种类型将被允许在许多二进制文件的同一系统中共存。所以，为了避免内存损坏，如果一个字段类型发生变化，我所要使用的类型ID必须改变。

我的计划变得清晰了。我必须创建我自己的、通用的类型ID

### Deriving my own Type ID
程序性宏似乎是计算类型ID的最佳方式。一个#[derive(UniversalType)]可以被贴在任何结构体、枚举和联盟上。不幸的是，我还没有想出一个主意来涵盖其他类型，比如函数指针或闭合类型。但为了实现我的网络化动态发布-订阅系统的梦想，这已经足够了。

现在，程序性宏应该做什么？我的想法是为每个类型创建一个字符串，这个字符串是唯一的，当且仅当这些类型被命名为相同的，并且它们的数据表示是兼容的。

然后，我在宏内对字符串进行散列，所有这些都是在编译时进行的，因此在实际的二进制文件中只有一个数字值结束。

重要的决定是哪些成分会进入这个独特的字符串。我将在这里告诉你我的理由。

首先，让我使用结构体的源代码（我们暂且忽略枚举和联合），并剥离所有注释和空白。这样一来，对字段或结构名称的任何改变都会导致类型ID的改变，因此它被认为是一个不同的类型。这正是我所希望的。

那么，该类型定义在哪个模块中呢？或者板块？起初，我认为我并不关心。或者更准确地说，我想让这成为类型唯一性的一个非因素。

为什么，我听到你问。答案是最大的灵活性。让我们分别看一下这两个。(crate和module)

在我看来，一个小小的重构，比如重命名一个父模块，不应该改变我的类型的身份。因此，模块不应该是它的一部分。

让它甚至独立于板条箱是另一个决定。我喜欢也不喜欢一个板块可以冒充另一个板块的类型的想法。喜欢的是可以在没有货物依赖的情况下共享类型，不喜欢的是其中的风险。

当然，内存安全是没有风险的，因为字段必须完全匹配。而且，无论如何都必须使用适当的序列化代码（没有内存魔法）。问题在于每个字段都会突然成为一个板块的公共接口的一部分，如果它派生出这个ID的话。这样做的一个大问题是，多个板块可能意外地共享类型。小的更新可以改变这一点，从而导致各种奇怪的错误。

但是，有一个非常重要的原因是，板条箱和模块链确实应该是类型ID的一部分。那就是，否则，生成的类型ID会与编译器对类型的概念不一致，甚至在一个单一的二进制中也是如此。

我的意思是，我希望在不同的二进制文件之间比较类型时有差异，这就是问题所在。但是在一个二进制文件中，如果有这些差异，那就很有缺陷了。因此，在使用例子中，我将假设模块名称和crate被考虑用于计算ID。

另一方面，一个灵活的解决方案可以是引入命名空间作为程序宏的输入。如果没有指定，将使用模块的完全合格名称，包括crate名称。然后，在默认情况下，生成的类型ID应该等同于（就其平等关系而言）core::any::TypeId。

好的一点是，有了这种灵活性，知道自己在做什么的人仍然可以通过覆盖该命名空间来做奇怪的共享。无论谁决定改变一个命名空间，都应该意识到具有相同字段的同样命名的结构（在不同的模块中）具有相同的通用ID，即使Rust编译器认为它们是不同的。

我开始在程序性宏中实现一个通用类型想法的原型，[代码在Github上](https://github.com/jakmeier/universal-type-id)。如果它成熟了，我可能会在某个时候在crates.io上发布它。但目前，这个实现还不完整，而且还有一些开放的设计问题。(如何处理通用类型参数？)

我们现在先不谈这个问题。在本节的最后，我将简要介绍一下类型ID如何在代码中使用。

### How would this be used?
在我的示例实现中，我创建了一个名为[UniversalType的特质](https://github.com/jakmeier/universal-type-id)，可以被派生。对于实现它的类型，可以检索到一个UniversalTypeId，类似于Rust核心的TypeId。

UniversalTypeId最好与标准TypeId一起使用。在每个二进制中，这两者之间应该有一个一对一的映射。(忽略命名空间共享以保持简单）。换句话说，模块名称和起源箱对类型ID很重要）。只是，另一个二进制文件可能有另一个 TypeId 关联，而我的 UniversalTypeId 规则将它们视为同一类型。

有了这个认识，我们可以使用HashMap<UniversalTypeId, Box<dyn Any>>，然后做我们之前用HashMap<TypeId, Box<dyn Any>>做的所有技巧。在一个单一的二进制里面，这完全等同于我在第一节和第二节所做的事情。

但在跨二进制文件发送数据时，我们必须要小心。Rust的内存布局并不稳定，所以我们不能直接发送纯二进制。幸运的是，Rust有很好的工具，可以用[serde](https://github.com/serde-rs/serde)和例如[bincode](https://crates.io/crates/bincode)进行安全序列化。

发布消息的实现总是知道类型的，所以调用序列化可以照常进行。然后，原始数据将通过网络发送，同时还有它的通用类型ID。

```rust
/* Send message */
let message = Ticket { number: 1 };
let header = message.universal_type_id();
let serialized_message: Vec<u8> = bincode::serialize(&message).unwrap();
// Now (header, serialized_message) is sent over the network
```
反序列化则更有趣。我们必须将反序列化调用（带有一个单态的类型参数）包装成一个闭包。要做到这一点，我们期望收到的每个类型都应该被注册并存储在一个由UniversalTypeId索引的哈希图中。

我在下面一个完全可行的例子中说明了这个概念。请注意，在主函数中注册一个消息类型是多么简单。尽管register_message_type()的实现中隐藏着复杂性。
```rust
#[derive(UniversalType, Serialize, Deserialize, Debug)]
struct Ticket {
    number: i32,
}

fn main() {
    /* Setup */
    let mut lib_state = SubscriptionManager::default();
    lib_state.register_message_type::<Ticket>();

    /* Send message */
    let message = Ticket { number: 1 };
    let header = message.universal_type_id();
    let serialized_message: Vec<u8> = bincode::serialize(&message).unwrap();

    // Now assume (header, serialized_message) is sent over the network

    /* Receive message and call subscriber */
    lib_state.forward(header, &serialized_message);

    // Received: Ticket { number: 1 }
}

#[derive(Default)]
struct SubscriptionManager {
    subscribers: HashMap<UniversalTypeId, Box<dyn Fn(&[u8])>>,
}

impl SubscriptionManager {
    fn register_message_type<T>(&mut self)
    where
        T: Any + UniversalType + DeserializeOwned + Debug,
    {
        let deserializer: Box<dyn Fn(&_)> =
            Box::new(|data| match bincode::deserialize::<T>(data) {
                Ok(msg) => {
                    println!("Received: {:?}", msg);
                }
                Err(err) => {
                    println!("ERROR: Failed to parse incoming message. {}", err);
                }
            });
        let uid = UniversalTypeId::of::<T>();
        self.subscribers.insert(uid, deserializer);
    }
    fn forward(&self, uit: UniversalTypeId, raw_data: &[u8]) {
        self.subscribers[&uit](raw_data);
    }
}
```
当然，与其在调试时打印数值，不如用它做一些更有用的事情。解码器函数可以将其转换为一个适当的Box<dyn Any>特征对象，并将其传递给对其进行处理的函数。

或者，这个值可以由现在的Nuts来处理。也就是说，我们可以调用nuts::publish(msg)，所有的本地订阅者都可以采取行动。

## Final Thoughts
好吧，我很高兴看到你还在阅读！让我们回顾一下我在这篇文章中展示的内容。让我们回顾一下我在这篇文章中展示的内容。

首先，我展示了类型可以直接被用作异质集合的键。有效地消除了对字符串键的需求，同时也没有放弃灵活性。

然后我更进一步。我将函数存储在一个集合中，并通过其参数的类型ID对它们进行索引。这允许动态调度，我用它来实现一个发布-订阅库，叫做Nuts。就可用性而言，Nuts最重要的特点是，它可以在任何地方使用，包括在回调闭包内，而不需要接收者地址或任何其他状态。

最后，我勾画了一种方法，以使动态类型超越从具有不稳定ABI的编译器中继承的限制。

所有这些想法在动态类型和编译时检查类型之间提供了一个有趣的组合。当我开始玩这些概念的时候，我认为我所做的事情是完全疯狂的。我喜欢Rust的静态类型系统，完全采用动态类型似乎是一个愚蠢的想法！但后来我开始看到，动态类型对Rust是多么有用。

但后来我开始发现它是多么有用。而最大的惊喜是，静态类型检查也基本上统治了所有的动态类型化代码。正如前面展示的所有复杂的类型参数所证明的那样，你根本无法逃脱Rust编译器的控制。再一次，我被Rust类型系统的惊人力量所震撼。这就是我想写这篇文章的原因。

然而，这并不是一篇容易写的文章。在我最初的几次尝试中，我意识到我自己都不知道这应该有什么好处。当然，我知道我的代码是有意义的，我一直在有效地使用它。但是，阐明为什么我的（公认的怪异）方法有任何意义是具有挑战性的。除此之外，要在简洁和不遗漏重要细节之间取得平衡是非常困难的。

无论如何，我希望这个最终版本能清楚地表明我的观点：我们（作为一个社区）可以使用Rust类型比我们目前所做的更多。

我总是试图为尽可能多的人写文章。但我担心这一次，可能只有资深的Rust程序员才能消化。如果你对此有任何反馈，请告诉我。(或其他方面。)我一直在寻求改进我的写作。

最后，我真的很想听听大家对这种动态类型的看法。(比AnyMap更进一步。)你认为这是一个隐藏的宝石，等待在Rust中更广泛地应用？或者你认为它非常小众，应该很少，甚至永远不会被使用？

本博客已在Rust编程论坛上分享。


