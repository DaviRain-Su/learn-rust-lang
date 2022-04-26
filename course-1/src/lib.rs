///！ # 01 内存： 值放在堆上还是放在栈上，这是一个问题
///
/// 前提背景问题
/// 很多人并没有搞懂什么时候数据应该放在栈上，什么时候应该放在堆上。
///
/// 代码中最基本的概念就是变量和值，而存放他们的地方就是内存，所以我们从内存开始。
///
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    /// "hello, world!" 作为一个字符串常亮，在编译时被存入可执行文件的.RODATA段，然后在程序加载时，获得一个固定的内存地址。
    /// 当执行"hello world".to_string()时，在堆上，一块新的内存被分配出来，并把"hello world"逐个字节拷贝过去。
    /// 当把堆上的数据赋值给s时，s作为分配在栈上的一个变量，他需要知道堆上内存的地址，另外由于堆上数据大小不确定且
    /// 可以增长，我们还需要知道他的长度以及他现在有多大。
    /// 为了表述这个字符串，我们使用了三个word，第一个表示指针，第二个表示字符串的当前长度，第三个表示这片内存的总容量。
    /// 字符串的内容放在堆上，而指向字符串的指针等信息放在栈上。
    ///
    /// ## 问题，数据什么时候可以放在栈上，什么时候可需要放在对上？
    ///
    fn in_stack_memory() {
        let s = "hello, world!".to_string();
        // println!("addr of ss: {:p}, s: {:p}, len: {}, capacity: {}, size: {}", &"hello world", &s, s.len(), s.capacity(), std::mem::size_of_val(&s));
        println!(
            "addr of ss: {:p}, s: {:p}, len: {}, capacity: {}, size: {}",
            &"hello, world!",
            &s,
            s.len(),
            s.capacity(),
            std::mem::size_of_val(&s)
        );
    }
}
