/// # 第一章 - 基础知识
/// 在第一章中，我们将探讨函数、变量和最基本的类型等基础知识。欢迎入坑！
///
/// 并且~以防你好奇这个跟你说话的螃蟹是谁，我叫 Ferris，一个非官方的 Rust 语言吉祥物。 很高兴能认识你！
///
/// ## Rust 练习场
/// 本教程使用的是来自 [Rust 练习场](https://play.rust-lang.org/) 的交互代码工具。
///
/// 这是一个玩转 Rust 并且向别人展示你的创造和挑战的最好工具。
pub mod content {
    use std::fmt::{Debug, Display};
    use std::ops::Add;

    /// # 变量
    ///
    /// 变量使用 `let` 关键字来声明。
    ///
    /// 在赋值时，`Rust` 能够在 99% 的情况下自动推断其类型。如果不能，你也可以手动将类型添加到变量声明中。
    ///
    /// 你也许注意到了，我们可以对同一个变量名进行多次赋值。这就是所谓的变量隐藏，可以更改变量类型以实现对该变量名的后续使用。
    ///
    /// 变量名总是遵循 蛇形命名法 (snake_case)。
    ///
    /// ## Code
    /// ```rust
    ///  let x = 13;
    ///  println!("i32 x: {}", x);
    ///
    ///  // rust 也可以显式声明类型
    ///  let x: f64 = std::f64::consts::PI;
    ///  println!("f64 x: {}", x);
    ///
    ///  // rust 也支持先声明后初始化，但是很少使用
    ///  let x;
    ///  x = 0;
    ///  println!("u32 x: {}", x);
    /// ```
    pub fn variables() {
        // rust 推断出x的类型
        let x = 13;
        println!("i32 x: {}", x);

        // rust 也可以显式声明类型
        let x: f64 = std::f64::consts::PI;
        println!("f64 x: {}", x);

        // rust 也支持先声明后初始化，但是很少使用
        let x;
        x = 0;
        println!("u32 x: {}", x);
    }

    #[test]
    fn test_variables() {
        variables();
    }

    /// # 修改变量
    ///
    /// Rust 非常关心哪些变量是可修改的。值分为两种类型：
    ///
    /// - 可变的 - 编译器允许对变量进行读取和写入。
    /// - 不可变的 - 编译器只允许对变量进行读取。
    ///
    /// 可变值用 `mut` 关键字表示。
    ///
    /// 关于这个概念，我们之后还会有更多的内容，但是眼下请谨记这个关键字即可。
    ///
    /// ## Code
    /// ```rust
    ///  let mut x = 42;
    ///  println!("mutable variables before: {}", x);
    ///  x = 13;
    ///  println!("mutable variables after: {}", x);
    /// ```
    pub fn mutable_variables() {
        let mut x = 42;
        println!("mutable variables before: {}", x);
        x = 13;
        println!("mutable variables after: {}", x);
    }

    #[test]
    fn test_mutable_variables() {
        mutable_variables();
    }

    /// # 基本类型
    ///
    /// Rust 有多种常见的类型：
    ///
    /// - 布尔型 - `bool` 表示 `true` 或 `false`
    /// - 无符号整型- `u8 u32 u64 u128` 表示正整数
    /// - 有符号整型 - `i8 i32 i64 i128` 表示正负整数
    /// - 指针大小的整数 - `usize isize` 表示内存中内容的索引和大小
    /// - 浮点数 - `f32 f64`
    /// - 元组`（tuple）` - `(value, value, ...)` 用于在栈上传递固定序列的值
    /// - 数组 - 在编译时已知的具有固定长度的相同元素的集合
    /// - 切片`（slice）` - 在运行时已知长度的相同元素的集合
    /// - `str(string slice)` - 在运行时已知长度的文本
    ///
    /// 文本可能比你在其他语言中学到的更复杂，因为 Rust 是一种系统编程语言，它关心的是你可能不太习惯的内存问题。 我们之后将详细讨论这个问题。
    ///
    /// 另外，你也可以通过将类型附加到数字的末尾来明确指定数字类型（如 `13u32` 和 `2u8`）.
    ///
    /// # Code
    ///
    /// ```rust
    /// let x = 12; // default type is i32.
    /// let a = 12u8;
    /// let b = 4.3; // default type is f64.
    /// let c = 4.3f32;
    /// let bv = true;
    /// let t = (13, false);
    /// let sentence = "hello world";
    /// println!("i32 x = {}", x);
    /// println!("u8 a = {}", a);
    /// println!("f64 b = {}", b);
    /// println!("f32 c = {}", c);
    /// println!("bool bv = {}", bv);
    /// println!("tuple t = {:?}", t);
    /// println!("&str sentence = {}", sentence);
    /// ```
    pub fn basic_type() {
        let x = 12; // default type is i32.
        let a = 12u8;
        let b = 4.3; // default type is f64.
        let c = 4.3f32;
        let bv = true;
        let t = (13, false);
        let sentence = "hello world";
        println!("i32 x = {}", x);
        println!("u8 a = {}", a);
        println!("f64 b = {}", b);
        println!("f32 c = {}", c);
        println!("bool bv = {}", bv);
        println!("tuple t = {:?}", t);
        println!("&str sentence = {}", sentence);
    }

    #[test]
    fn test_basic_type() {
        basic_type();
    }

    /// # 基本类型转换
    ///
    /// 当涉及到数字类型时，Rust 要求明确。一个人不能想当然地把“`u8`”用在“`u32`”上而不出错。
    ///
    /// 幸运的是，使用 `as` 关键字，`Rust` 使数字类型转换非常容易。
    ///
    /// ## Code
    /// ```rust
    /// let a = 13u8;
    /// let b = 7u32;
    /// let c = a as u32 + b;
    /// println!("u8_a({}) + u32_b({}) = u32_c({})", a, b, c);
    /// let t = true;
    /// println!("true as u8 is {}", t as u8);
    /// ```
    pub fn basic_type_convert() {
        let a = 13u8;
        let b = 7u32;
        let c = a as u32 + b;
        println!("u8_a({}) + u32_b({}) = u32_c({})", a, b, c);
        let t = true;
        println!("true as u8 is {}", t as u8);
    }

    #[test]
    fn test_basic_type_convert() {
        basic_type_convert();
    }

    /// # 常量
    ///
    /// 常量允许我们高效地指定一个在代码中会被多次使用的公共值。
    /// 不同于像变量一样在使用的时候会被复制，常量会在编译期间直接用它们的值来替换变量的文本标识符。
    ///
    /// 不同于变量，常量必须始终具有显式的类型。
    ///
    /// 常量名总是遵循 全大写蛇形命名法（SCREAMING_SNAKE_CASE）
    ///
    /// ## Code
    /// ```rust
    /// const PI: f64 = std::f64::consts::PI;
    /// println!(
    ///  "To make an apple {} from scratch, you must first create a universe.",
    ///  PI
    /// );
    /// ```
    pub fn constant_type() {
        const PI: f64 = std::f64::consts::PI;

        println!(
            "To make an apple {} from scratch, you must first create a universe.",
            PI
        );
    }

    #[test]
    fn test_constant_type() {
        constant_type();
    }

    /// # 数组
    ///
    /// 数组是所有相同类型数据元素的固定长度集合。
    ///
    /// 一个数组的数据类型是 `[T;N]`，其中 `T` 是元素的类型，`N` 是编译时已知的固定长度。
    ///
    /// 可以使用 `[x]` 运算符提取单个元素，其中 `x` 是所需元素的 `usize` 索引（从 `0` 开始）。
    ///
    /// # Code
    /// ```
    ///  let nums: [i32; 3] = [1, 2, 3];
    ///  println!("nums is {:?}", nums);
    ///  println!("nums index 1 is {}", nums[1]);
    /// ```
    pub fn array_type() {
        let nums: [i32; 3] = [1, 2, 3];
        println!("nums is {:?}", nums);
        println!("nums index 1 is {}", nums[1]);
    }

    #[test]
    fn test_array_type() {
        array_type();
    }

    /// # 函数
    ///
    /// 函数可以有 0 个或者多个参数。
    ///
    /// 在这个例子中，add 接受类型为 `i32`（32 位长度的整数）的两个参数。
    ///
    /// 函数名总是遵循 蛇形命名法 (snake_case)
    ///
    /// ## Code
    /// ```rust
    /// use std::fmt::{Debug, Display};
    /// use std::ops::Add;
    /// fn add<T: Copy + Add<Output = T> + Display + Debug>(x: T, y: T) -> T {
    ///     x + y
    /// }
    ///
    /// println!("1 + 2 = {}", add(1, 2));
    /// ```
    pub fn function_type() {
        fn add<T: Copy + Add<Output = T> + Display + Debug>(x: T, y: T) -> T {
            x + y
        }

        println!("1 + 2 = {}", add(1, 2));
    }

    #[test]
    fn test_function_type() {
        function_type();
    }

    /// # 多个返回值
    ///
    /// 函数可以通过元组来返回多个值。
    ///
    /// 元组元素可以通过他们的索引来获取。
    ///
    /// Rust 允许我们将后续会看到的各种形式的解构，也允许我们以符合逻辑的方式提取数据结构的子片段。敬请期待后面的内容！
    ///
    /// ## Code
    /// ```rust
    /// fn swap<T>(x: T, y: T) -> (T, T) {
    ///  (y, x)
    ///  }
    ///
    ///  println!("swap before is (123, 321)");
    ///  // result one tuple
    ///  let result = swap(123, 321);
    ///  println!("swap after is {:?}", result);
    ///
    ///  // decontruct tuple to two variables
    ///  let (a, b) = swap(result.0, result.1);
    ///  println!("a = {}, b = {}", a, b);
    /// ```
    pub fn function_mutli_return_value() {
        fn swap<T>(x: T, y: T) -> (T, T) {
            (y, x)
        }

        println!("swap before is (123, 321)");
        // result one tuple
        let result = swap(123, 321);
        println!("swap after is {:?}", result);

        // decontruct tuple to two variables
        let (a, b) = swap(result.0, result.1);
        println!("a = {}, b = {}", a, b);
    }

    #[test]
    fn test_function_mutli_return_value() {
        function_mutli_return_value();
    }

    /// # 返回空值
    ///
    /// 如果没有为函数指定返回类型，它将返回一个空的元组，也称为单元。
    ///
    /// 一个空的元组用 `()` 表示。
    ///
    /// 直接使用 `()` 的情况相当不常见。但它经常会出现（比如作为函数返回值），所以了解其来龙去脉非常重要。
    ///
    /// ## Code
    /// ```rust
    /// fn make_nothing() -> () {
    ///     return ();
    /// }
    ///
    /// // 返回类型隐含为 ()
    /// fn make_nothing2() {
    ///     // 如果没有指定返回值，这个函数将会返回 ()
    /// }
    ///
    /// let a = make_nothing();
    /// let b = make_nothing2();
    ///
    /// // 打印a和b的debug字符串，因为很难去打印空
    /// println!("The value of a: {:?}",a);
    /// println!("The value of b: {:?}",b);
    /// ```
    pub fn function_return_unit() {
        fn make_nothing() -> () {
            return ();
        }

        // 返回类型隐含为 ()
        fn make_nothing2() {
            // 如果没有指定返回值，这个函数将会返回 ()
        }

        let a = make_nothing();
        let b = make_nothing2();

        // 打印a和b的debug字符串，因为很难去打印空
        println!("The value of a: {:?}", a);
        println!("The value of b: {:?}", b);
    }

    #[test]
    fn test_function_return_unit() {
        function_return_unit();
    }

    /// # 第一章总结
    /// 到目前为止一切都进展顺利！ Rust 的基础知识还不赖，对吧？
    /// 我们一起窥探了 Rust 编译器是如何思考的。 作为一种系统编程语言，
    /// 它非常关心内存中值的大小，是否可以修改内容，并确保数值符合你的预期。
    /// 接下来，我们将看一些老朋友：`if` 判断和 `for` 循环。
    ///
    /// 其他教学资源（英文）：
    ///
    /// - [Youtube 视频：Rust Cast - 深入了解 Rust 的基础数字类型 types](https://www.youtube.com/watch?v=n5TRBkbystY)
    ///
    /// - [网页：Rust 之书 2018 - 基本数据的深层描述 types](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch03-02-data-types.html)
    ///
    /// - [网页：Rust Cheat Sheet - 数据类型](https://cheats.rs/#basic-types)
    pub fn chapter1_summary() {
        println!("Chapter1 summary!");
        println!("
# 第一章总结
到目前为止一切都进展顺利！ Rust 的基础知识还不赖，对吧？ 我们一起窥探了 Rust 编译器是如何思考的。 作为一种系统编程语言，它非常关心内存中值的大小，是否可以修改内容，并确保数值符合你的预期。 接下来，我们将看一些老朋友：if 判断和 for 循环。

其他教学资源（英文）：

- [Youtube 视频：Rust Cast - 深入了解 Rust 的基础数字类型 types](https://www.youtube.com/watch?v=n5TRBkbystY)

- [网页：Rust 之书 2018 - 基本数据的深层描述 types](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch03-02-data-types.html)

- [网页：Rust Cheat Sheet - 数据类型](https://cheats.rs/#basic-types)");
    }

    #[test]
    fn test_chapter_summary() {
        chapter1_summary();
    }
}
