/// # 第二章 - 基本控制流
///
/// 在本章中，我们将讨论 Rust 中的基本控制流方法。 如果你熟悉类 C 的语言，你会感到宾至如归，兴许还能享受到一些小惊喜
pub mod content {
    /// # if/else if/else
    ///
    /// Rust 中的代码分支不足为奇。
    ///
    /// Rust 的条件判断没有括号！~~需要括号干什么。~~我们现有的逻辑就看起来就很干净整洁呀。
    ///
    /// 不过呢，所有常见的逻辑运算符仍然适用：==，!=， <， >， <=， >=， !， ||， &&
    /// ## Code
    /// ```rust
    /// let x = 42;
    /// if x < 42 {
    ///     println!("less than 42");
    /// } else if x == 42 {
    ///     println!("is 42");
    /// } else {
    ///     println!("greater then 42");
    /// }
    /// ```
    pub fn if_else_example() {
        let x = 42;
        if x < 42 {
            println!("less than 42");
        } else if x == 42 {
            println!("is 42");
        } else {
            println!("greater then 42");
        }
    }

    #[test]
    fn test_if_else_example() {
        if_else_example();
    }

    /// # 循环
    ///
    /// 需要一个无限循环？
    ///
    /// 使用 Rust 很容易实现。
    ///
    /// break 会退出当前循环。但 loop 还有个秘密，我们很快讲到。
    ///
    /// # Code
    ///```rust
    /// let mut x = 0;
    /// loop {
    ///     x += 1;
    ///     if x == 42 {
    ///         break;
    ///    }
    /// }
    ///
    /// println!("x = {}", x);
    /// ```
    pub fn loop_example() {
        let mut x = 0;
        loop {
            x += 1;
            if x == 42 {
                break;
            }
        }
        println!("x = {}", x);
    }

    #[test]
    fn test_loop_example() {
        loop_example();
    }

    /// #while
    ///
    /// while 允许你轻松地向循环添加条件。
    ///
    /// 如果条件一旦变为 false，循环就会退出
    /// # Code
    ///```rust
    /// let mut x = 0;
    /// while x != 42 {
    ///     x += 1;
    /// }
    /// ```
    pub fn while_example() {
        let mut x = 0;
        while x != 42 {
            x += 1;
        }
    }

    #[test]
    fn test_while_example() {
        while_example();
    }

    /// # for
    /// Rust 的 for 循环是一个强大的升级。它遍历来自计算结果为迭代器的任意表达式的值。 迭代器是什么？迭代器是一个你可以一直询问“下一项是什么？”直到没有其他项的对象。
    ///
    /// 我们将在以后的章节中进一步探讨这一点，与此同时，我们知道 Rust 使创建生成整数序列的迭代器变得容易。
    ///
    /// - .. 运算符创建一个可以生成包含起始数字、但不包含末尾数字的数字序列的迭代器。
    ///
    /// - ..= 运算符创建一个可以生成包含起始数字、且包含末尾数字的数字序列的迭代器。
    /// # Code
    /// ```rust
    /// for x in 0..5 {
    ///     println!("The item is {}",x);
    /// }
    ///
    /// for x in 0..=5 {
    ///     println!("The item is {}",x);
    /// }
    /// ```
    pub fn for_example() {
        for x in 0..5 {
            println!("The item is {}", x);
        }

        for x in 0..=5 {
            println!("The item is {}", x);
        }
    }

    #[test]
    fn text_for_example() {
        for_example();
    }

    /// # match
    /// 想念你的 switch 语句吗？Rust 有一个非常有用的关键字，用于匹配值的所有可能条件， 并在匹配为真时执行相应代码。我们先来看看对数字的使用。在未来章节中，我们将有更多 更复杂的数据模式匹配的说明，我向你保证，它将值得等待。
    ///
    /// match 是穷尽的，意为所有可能的值都必须被考虑到。
    ///
    /// 匹配与解构相结合是迄今为止你在 Rust 中看到的最常见的模式之一。
    /// # code
    /// ```rust
    /// let x = 42;
    /// match x {
    ///     0 => println!("found zero!"),
    ///     // 我们可以匹配多个值
    ///     1 | 2 => println!("found 1 or 2!"),
    ///     // 我们可以匹配迭代器
    ///     3..=9 => println!("found a number 3 to 9 inclusively!"),
    ///     // 我们可以将匹配数值绑定到变量
    ///     matched_num @ 10..=100 => println!("found {} number between 10 and 100!",matched_num),
    ///     // 这是默认匹配，如果没有处理所有情况，则必须存在该匹配
    ///     _ => println!("found something else!"),
    /// }
    /// ```
    pub fn match_example() {
        let x = 42;
        match x {
            0 => println!("found zero!"),
            // 我们可以匹配多个值
            1 | 2 => println!("found 1 or 2!"),
            // 我们可以匹配迭代器
            3..=9 => println!("found a number 3 to 9 inclusively!"),
            // 我们可以将匹配数值绑定到变量
            matched_num @ 10..=100 => println!("found {} number between 10 and 100!", matched_num),
            // 这是默认匹配，如果没有处理所有情况，则必须存在该匹配
            _ => println!("found something else!"),
        }
    }

    #[test]
    fn test_match_example() {
        match_example();
    }

    /// # 从循环中返回值
    /// loop 可以被中断以返回一个值。
    ///
    /// ## Code
    ///``` rust
    /// let mut x = 0;
    /// let v = loop {
    ///     x += 1;
    ///     if x == 13 {
    ///     break "found the 13";
    /// }
    /// };
    /// println!("from loop: {}", v);
    ///```
    pub fn break_value_from_loop() {
        let mut x = 0;
        let v = loop {
            x += 1;
            if x == 13 {
                break "found the 13";
            }
        };
        println!("from loop: {}", v);
    }

    #[test]
    fn test_break_value_from_loop() {
        break_value_from_loop()
    }

    /// # 从块表达式返回值
    /// if，match，函数，以及作用域块都有一种返回值的独特方式。
    ///
    /// 如果 if、match、函数或作用域块中的最后一条语句是不带 ; 的表达式， Rust 将把它作为一个值从块中返回。这是一种创建简洁逻辑的好方法，它返回一个 可以放入新变量的值。
    ///
    /// 注意，它还允许 if 语句像简洁的三元表达式一样操作。
    /// # Code
    /// ```rust
    ///     let x = 42;
    ///
    ///     let v = if x < 42 { -1} else { 1};
    ///     println!("from if: {}", v);
    ///
    ///     let food = "hamburger";
    ///     let result = match food {
    ///      "hotdog" => "is hotdog",
    ///      _ => "is not hotdog",
    ///     };
    ///     println!("identifying food: {}", result);
    ///
    ///     let v = {
    ///         let a = 1;
    ///         let b = 2;
    ///         a + b
    ///     };
    ///
    ///     println!("from block: {}", v);
    /// ```
    pub fn return_block_expression_value() {
        let x = 42;

        let v = if x < 42 { -1 } else { 1 };
        println!("from if: {}", v);

        let food = "hamburger";
        let result = match food {
            "hotdog" => "is hotdog",
            _ => "is not hotdog",
        };
        println!("identifying food: {}", result);

        let v = {
            let a = 1;
            let b = 2;
            a + b
        };

        println!("from block: {}", v);
    }

    #[test]
    fn test_return_block_expression_value() {
        return_block_expression_value();
    }

    /// # 第二章总结
    ///
    /// 希望即便是在最基本的语言特性中，我也已经向你展示了 Rust 的强大功能。
    /// 我们将在后续章节更深入地讨论 for 和 match，因为我们将获得更多可以 利用它们能力的知识。
    /// 接下来，我们将讨论 Rust 的基本数据结构。
    pub fn chapter2_summary() {
        println!(
            "\
第二章总结
希望即便是在最基本的语言特性中，我也已经向你展示了 Rust 的强大功能。
我们将在后续章节更深入地讨论 for 和 match，因为我们将获得更多可以 利用它们能力的知识
。接下来，我们将讨论 Rust 的基本数据结构。
        "
        );
    }
}
