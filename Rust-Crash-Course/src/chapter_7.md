# 07｜所有权：值的生杀大权到底在谁手上？

先开始rust最难的部分，rust的所有权和生命周期问题：

所有权和生命周期是rust和其他编程语言的主要区别，也是rust其他知识点的基础。


## 思路是从一个变量的堆栈的行为开始，探究rust设计所有权和生命周期。

### 变量在函数调用时发生了什么呢？

```aquascope,interpreter
fn main() {
    let data = vec![10, 42, 9, 8];

    let v = 42;

    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}

```

动态数据因为大小在编译期无法确定，所以放在堆上，并且在栈上包含了一个长度和容量的胖指针指向堆上的内容。

按照传统的语言的做法是，这样会使得在堆上内存有两个引用。对于堆上的内容什么时候能够释放，尤其是在多个调用引用的时候，很难分辨清楚什么是放内存。

对于堆内存多次引用的问题。不同的语言有自己不同的解决方案：

- C/C++要求开发者手工处理，需要遵守规范，要不很容易犯错。
- Java等语言使用追踪式GC，定期扫描堆上不使用的内存，删除。但是GC会带来STW的问题，有性能损耗。
- Object/Swift使用自动引用计数（ARC），在编译时自动添加维护引用计数的代码，减轻开发者维护堆内存的负。同样也是有运行时负担。

从源头来看，问题出在了堆上的内容，可以被随意的引用。 那我们是不是可以限制引用的行为呢？

### Rust的解决方法

我们的学到的经验时，恰到好处的限制，反而会释放无穷的创新和生产力。

由此引出了。具体的做法来怎么限制数据的引用的行为呢？
回答这个问题，我们需要回答的是，谁是真正拥有数据或者说值的生杀大权的呢，这种权利可以是共享还是独占的？

### 所有权和Move语义

我们直观的思考，那就是一个值最好只有一个拥有着，因为所有权共享，会带来使用和释放上的不明确。

Rust中所有权的规则：

- 一个值只能被一个变量所拥有，这个变量成为所有者。each value in rust has a variable that's called its owner.
- 一个值同一时刻只能有一个所有者。There can only be one owner at a time, 也就是说不能有两个变量拥有相同的值。
- 当所有者离开作用域，其拥有多饿值会被丢弃。when the owner goes out of scope, the value will be dripped. 内存得到释放。

```aquascope,interpreter
fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = data;
    println!("sum of data1: {}", sum(data1));
    println!("data1: {:?}", data1); // error
    println!("sum of data: {}", sum(data)); // error
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}
```

所有权规则，解决了真正拥有数据的生杀大权，让堆上数据的多重引用不复存在，这是他最大的优势。


对于一些简单的数据结构也存在堆上很不好，效率低。简单的数据结构可以直接存放在栈上。

rust提供了两个解决的方案：

- 当不希望转移所有权的时候，rust还提供了Copy语义，如果一个数据结构实现了Copy trait, 那么他就会使用Copy trait. 
这样在赋值或者传递参数的时候，值会被自动按位拷贝。
- 当不希望使用移动语义，还有Copy语义的时候，还可以使用“借用”


### Copy语义和Copy trait

符合Copy 语义的类型，在你赋值或者传递参数的时候，值会自动按位拷贝。

当你移动一个值的时候，如果值类型实现了Copy trait， 就会自动使用Copy 语义，进行拷贝。否则使用Move移动语义进行移动。


rust中什么类型实现了移动语义呢？

```aquascope,interpreter
fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    // function (actually a pointer) is Copy
    is_copy::<fn()>();

    // raw pointer is Copy
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // immutable reference is Copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    // array/tuple with values which is Copy is Copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

/*
fn types_not_impl_copy_trait() {
    // unsized or dynamic sized type is not Copy
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();
    is_copy::<String>();

    // mutable reference is not Copy
    is_copy::<&mut String>();

    // array / tuple with values that not Copy is not Copy
    is_copy::<[Vec<u8>; 4]>();
    is_copy::<(String, u32)>();
}
*/

fn main() {
    types_impl_copy_trait();
    // types_not_impl_copy_trait();
}
```

- 原生类型，包括函数，不可变引用和裸指针实现了Copy
- 数组和元组，如果其内部的数据结构实现了Copy, 那么他们也实现了Copy
- 可变引用没有实现Copy 
- 非固定大小的数据结构，没有实现Copy



## summary

- 所有权： 一个值只能被一个变量所拥有，且同一时刻只能有一个所有者，当所有者离开作用域，其拥有的值会被丢弃。内存得到释放。
- Move语义：赋值或者传递参数导致值Move,所有权被转移，一旦所有权被转移，之前的变量就不能访问。
- Copy语义： 如果值实现了Copy trait，那么赋值或者传参会使用Copy语义，相应的值会被按位拷贝（浅拷贝），产生新的值。

核心点：Rust 通过单一所有权来限制任意引用的行为，就不难理解这些新概念背后的设计意义。

## 思考题

1. 在 Rust 下，分配在堆上的数据结构可以引用栈上的数据么？
   1. 只要栈上的数据生命周期大于堆上数据的生命周期就可以
2. 为什么？main() 函数传递给 find_pos() 函数的另一个参数 v，也会被移动吧？为什么图上并没有将其标灰？
   1. 因为其为基本数据类型，实现了Copy trait

Rust 在设计时就已经保证了你无法为一个在堆上分配内存的结构实现 Copy。所以 Vec / String 等结构是不能实现 Copy 的。因为这条路已经被堵死了：Copy trait 和 Drop trait 不能共存。一旦你实现了 Copy trait，就无法实现 Drop trait。反之亦然。

有同学看到裸指针 `*const T/ *mut T` 实现了 Copy，就会想如果我用 unsafe 把 `Vec<T>` 的指针取出来，组成一个数据结构，到处 Copy，然后其中一个 drop 后，岂不就造成 use after free，破坏了 Rust 的安全性保证？很遗憾，Rust 并不允许你这么做。因为你无法实现 Drop。

我写了一段代码，感兴趣的同学可以看一下：https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4828e734f6f161dfce32098333a1aaa5

```rust
use std::{fmt, slice};

#[derive(Clone, Copy)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释掉，看看会出什么问题
// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
//         drop(data)
//     }
// }

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4];

    let buf: RawBuffer = data.into();

    // 因为 buf 允许 Copy，所以这里 Copy 了一份
    use_buffer(buf);

    // buf 还能用
    println!("buf: {:?}", buf);
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);

    // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
    drop(buf)
}
```

## 参考资料

trait是Rust用于定义数据结构行为的接口。如果一个数据结构实现了 Copy trait，那么它在赋值、函数调用以及函数返回时会执行 Copy 语义，
值会被按位拷贝一份（浅拷贝），而非移动。

堆内存成了像栈一样的受控内存，只不过栈内存是受栈帧控制，堆内存受其栈上的所有者控制。再回顾一下堆内存的使用场景：1. 存放栈无法处理的内存（过大，或者长度不定，或者需要动态增长），2. 在同一个调用栈中真正需要被多个数据结构共享 3. 在多个调用栈中共享 