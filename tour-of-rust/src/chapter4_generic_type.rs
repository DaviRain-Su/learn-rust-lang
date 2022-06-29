/// # 第四章 - 泛型
///
/// 泛型在 Rust 中非常重要。它们用于表示可空值（即可能还没有值的变量）、错误处理、集合等等！
/// 在本章中，我们将学习你可能将会经常使用的基本泛型知识。
pub mod content {
    /// # 泛型是什么？
    /// 泛型允许我们不完全定义一个 `struct` 或 `enum`，
    /// 使编译器能够根据我们的代码使用情况，在编译时创建一个完全定义的版本。
    ///
    /// Rust 通常可以通过查看我们的实例化来推断出最终的类型，
    /// 但是如果需要帮助，你可以使用 `::<T>` 操作符来显式地进行操作，
    /// 该操作符也被称为 `turbofish` （它是我的好朋友！）。
    pub fn generic_type() {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct BagOfHolding<T> {
            item: T,
        }

        // 注意：通过使用泛型，我们创建了编译时创建的类型，使代码更大
        // Turbofish 使之显式化
        let i32_bag = BagOfHolding::<i32> { item: 23 };

        let bool_bag = BagOfHolding::<bool> { item: true };

        let float_bag = BagOfHolding { item: 23.4 };

        let bag_in_bag = BagOfHolding {
            item: BagOfHolding { item: 23 },
        };
        println!("i32 bag : {:?}", i32_bag);
        println!("bool bag : {:?}", bool_bag);
        println!("float bag: {:?}", float_bag);
        println!("bag in bag: {:?}", bag_in_bag);
    }

    #[test]
    fn test_generic_type() {
        generic_type();
    }

    /// # 表示空
    ///
    /// 在其他语言中，关键字 `null` 用于表示没有值。它给编程语言带来了困难，因为它使我们的程序在与变量字段交互时可能失败。
    ///
    /// Rust 没有 `null`，但这并不代表我们不知道表示空的重要性！我们可以使用一个我们已经了解过的工具来简单地表示这一点。
    ///
    /// 因为缺少 `null` 值，这种为一个或多个替代值提供 `None` 替代表示的模式非常常见， 泛型有助于解决这一难题。
    pub fn represent_empty() {
        #[allow(dead_code)]
        enum Item {
            Inventory(String),
            // None represents the absence of an empty
            None,
        }

        #[allow(dead_code)]
        struct BagOfHolding {
            item: Item,
        }
    }

    #[test]
    fn test_represent_empty() {
        represent_empty();
    }

    /// Option
    /// Rust 有一个内置的泛型枚举叫做 `Option`，它可以让我们不使用 `null` 就可以表示可以为空的值。
    ///
    /// ```rust
    /// enum Option<T> {
    ///     None,
    ///     Some(T),
    /// }
    /// ```
    /// 这个枚举很常见，使用关键字 Some 和 None 可以在任何地方创建其实例.
    pub fn option_type() {
        pub struct BagOfHolding<T> {
            item: Option<T>,
        }

        // 注意：一个放 i32 的 bag，里面什么都没有！
        // 我们必须注明类型，否则 Rust 不知道 bag 的类型
        let i32_bag = BagOfHolding::<i32> { item: None };

        if i32_bag.item.is_none() {
            println!("there's nothing in the bag!");
        } else {
            println!("there's something in the bag!");
        }

        let i32_bag = BagOfHolding::<i32> { item: Some(23) };

        if i32_bag.item.is_some() {
            println!("there's something in the bag!");
        } else {
            println!("there's nothing in the bag!");
        }

        match i32_bag.item {
            Some(v) => {
                println!("found {} in bag", v);
            }
            None => {
                println!("Found nohting!");
            }
        }
    }

    #[test]
    fn test_option_type() {
        option_type();
    }

    /// # Result
    /// Rust 有一个内置的泛型枚举叫做 `Result`，它可以让我们返回一个可能包含错误的值。 这是编程语言进行错误处理的惯用方法。
    ///
    /// ```rust
    /// enum Result<T, E> {
    ///     Ok(T),
    ///     Err(E),
    /// }
    /// ```
    /// 注意我们的泛型有多个用逗号分隔的参数化的类型。
    ///
    /// 这个枚举很常见，使用关键字 `Ok` 和 `Err` 可以在任何地方创建其实例。
    pub fn result_type() {
        fn do_something_that_fail(i: i32) -> Result<f32, String> {
            if i == 42 {
                Ok(23.0)
            } else {
                Err(String::from("this is not the right number"))
            }
        }

        let result = do_something_that_fail(23);

        match result {
            Ok(v) => println!("found: {}", v),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_result_type() {
        result_type();
    }

    /// # 可失败的主函数
    ///
    /// `main` 函数有可以返回 `Result` 的能力！
    pub fn can_fail_function() -> Result<(), String> {
        fn do_something_that_fail(i: i32) -> Result<f32, String> {
            if i == 42 {
                Ok(23.0)
            } else {
                Err(String::from("this is not the right number"))
            }
        }

        let result = do_something_that_fail(23);

        match result {
            Ok(v) => {
                println!("found: {}", v);
            }
            Err(_e) => return Err(String::from("something went wrong in main!")),
        }

        // Notice we use a unit value inside a Result Ok
        // to represent everything is fine
        Ok(())
    }

    #[test]
    fn test_can_fail_function() {
        let result = can_fail_function();
        println!("Result {:?}", result);
    }

    /// # 优雅地错误处理
    ///
    /// Result 如此常见以至于 Rust 有个强大的操作符 `?` 来与之配合。 以下两个表达式是等价的：
    ///```none
    /// fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    ///     if i == 42 {
    ///         Ok(13.0)
    ///     } else {
    ///         Err(String::from("this is not the right number"))
    ///     }
    /// }
    /// let ret = do_something_that_might_fail(23)?;
    ///
    /// match do_something_that_might_fail(23) {
    ///     Ok(v) => todo!(),
    ///     Err(e) => todo!(),
    /// }
    ///```
    pub fn handle_error() -> Result<(), String> {
        fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
            if i == 42 {
                Ok(13.0)
            } else {
                Err(String::from("this is not the right number"))
            }
        }

        let v = do_something_that_might_fail(23)?;
        println!("found : {}", v);

        Ok(())
    }

    #[test]
    fn test_handle_error() {
        let result = handle_error();
        println!("Result: {:?}", result);
    }

    /// # 丑陋的 Option/Result 处理
    ///
    /// 当你只是试图快速地写一些代码时，`Option/Result` 对付起来可能比较无聊。
    /// `Option` 和 `Result` 都有一个名为 `unwrap` 的函数：这个函数可以简单粗暴地获取其中的值。 `unwrap` 会：
    ///
    /// 获取 `Option/Result` 内部的值
    /// 如果枚举的类型是 `None/Err`， 则会 `panic!`
    /// 这两段代码是等价的：
    ///
    /// ```none
    /// my_option.unwrap();
    /// match my_option {
    ///     Some(v) => v,
    ///     None => panic!("some error message generated by Rust!"),
    /// }
    /// ```
    /// 类似的：
    ///
    /// ```none
    /// my_result.unwrap();
    /// match my_result {
    ///     Ok(v) => v,
    ///     Err(e) => panic!("some error message generated by Rust!"),
    /// }
    /// ```
    /// 不过啊，做个好 Rustacean，正确地使用 match！
    pub fn ugly_handle_option_or_result() -> Result<(), String> {
        fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
            if i == 42 {
                Ok(13.0)
            } else {
                Err(String::from("this is not the right number"))
            }
        }

        // 简洁但假设性强，而且很快就会变得丑陋
        let v = do_something_that_might_fail(42).unwrap();
        println!("Found {}", v);

        // painc!
        // let v = do_something_that_might_fail(2).unwrap();
        let v = do_something_that_might_fail(2);
        println!("found {:?}", v);

        Ok(())
    }

    #[test]
    fn test_ugly_handle_option_or_result() {
        let result = ugly_handle_option_or_result();
        println!("Result {:?}", result);
    }

    /// # Vectors
    ///
    /// 一些经常使用的泛型是集合类型。一个 `vector` 是可变长度的元素集合，以 `Vec` 结构表示。
    ///
    /// 比起手动构建，宏 `vec!` 让我们可以轻松地创建 `vector`。
    ///
    /// `Vec` 有一个形如 `iter()` 的方法可以为一个 `vector` 创建迭代器，这允许我们可以轻松地将 `vector` 用到 `for` 循环中去。
    ///
    /// 内存细节：
    ///
    /// - `Vec` 是一个结构体，但是内部其实保存了在堆上固定长度数据的引用。
    /// - 一个 `vector` 开始有默认大小容量，当更多的元素被添加进来后，
    /// 它会重新在堆上分配一个新的并具有更大容量的定长列表。（类似 C++ 的 vector）
    ///
    /// ## Code
    ///```rust
    /// let mut i32_vec = Vec::<i32>::new();
    /// i32_vec.push(1);
    /// i32_vec.push(2);
    /// i32_vec.push(3);
    ///
    /// let mut float_vec = Vec::new();
    /// float_vec.push(1.2);
    /// float_vec.push(1.3);
    /// float_vec.push(3.3);
    ///
    /// let string_vec = vec![String::from("hello, world!")];
    /// for word in string_vec.iter() {
    ///     println!("{}", word);
    /// }
    /// ```
    pub fn vector_type() {
        let mut i32_vec = Vec::<i32>::new();
        i32_vec.push(1);
        i32_vec.push(2);
        i32_vec.push(3);

        let mut float_vec = Vec::new();
        float_vec.push(1.2);
        float_vec.push(1.3);
        float_vec.push(3.3);

        let string_vec = vec![String::from("hello, world!")];
        for word in string_vec.iter() {
            println!("{}", word);
        }
    }

    #[test]
    fn test_vector_type() {
        vector_type();
    }

    /// # 第四章 - 总结
    ///
    /// 在这一章中，我们了解了泛型给我们带来的强大功能！如果你还不完全知道该如何使用这一切，
    /// 别担心，仅仅是了解这些将会在代码中反复出现的中心思想就足够了。我们的功能在日趋强大！
    /// 在下一章中，我们将讨论 Rust 中的一个重要概念：数据所有权。
    pub fn chapter4_summary() {
        println!(
            "
第四章 - 总结
在这一章中，我们了解了泛型给我们带来的强大功能！如果你还不完全知道该如何使用这一切，
别担心，仅仅是了解这些将会在代码中反复出现的中心思想就足够了。
我们的功能在日趋强大！ 在下一章中，我们将讨论 Rust 中的一个重要概念：数据所有权。
        "
        );
    }
}
