/// # 第六章 - 文本
///
/// 了解完了 Rust 内存管理的机制，是时候更为详尽地了解文本相关的内容了。
/// Rust 非常关注文本国际化与字节层级的问题，这意味着 Rust 有许多实用的工具来解决这些问题。
/// 虽然你可能对其它语言中的这些问题并不熟悉。
pub mod content {
    /// # 字符串常量（String Literals）
    ///
    /// 字符串常量（String Literals）采用 Unicode 编码（注：下文提及的 utf-8 为 Unicode 的一部分）。
    ///
    /// 字符串常量的类型为 &'static str：
    ///
    /// & 意味着该变量为对内存中数据的引用，没有使用 &mut 代表编译器将不会允许对该变量的修改
    /// 'static 意味着字符串数据将会一直保存到程序结束（它不会在程序运行期间被释放（drop））
    /// str 意味着该变量总是指向一串合法的 utf-8 字节序列。
    /// 内存细节：
    ///
    /// Rust 编译器可能会将字符串储存在程序内存的数据段中。
    pub fn literal_str() {
        let a: &'static str = "Hello, world!";

        println!("literal_str = {}, and len = {}", a, a.len());
    }

    #[test]
    fn test_literal_str() {
        literal_str();
    }
    /// # 什么是 utf-8
    ///
    /// 随着在计算机上使用的语言增加，需要一个能比 ASCII 编码（1 字节表示 1 个字符，总共可表示 256 个字符）表示更多字符的编码来容纳其它语言的字符。
    ///
    /// utf-8 编码这时被引入来解决这个问题，它用 1-4 个字节来表示 1 个字符，这使得可以表示的字符数大大增加。
    ///
    /// 使用可变长度的字节来表示字符有一个优点，就是常见的 ASCII 编码字符在 utf-8 编码中无需使用更多的字节（也是 1 字节表示 1 个字符）。
    ///
    /// 但是这样做也有缺点，在 utf-8 文本中通过索引来匹配字符（例：my_text[3] 获取 my_text 的第 4 个字符）将不能像以前的编码标准那么快（以前编码标准花费 O(1) 常数时间）。 这是因为前面的字符具有可变的对应字节，从而无法直接确定第 4 个字符在字节序列中的起始字节。
    ///
    /// 我们需要遍历 utf-8 的字节序列才可以得到对应 Unicode 字符的起始位置（这将花费 O(n) 线性时间）。
    ///
    /// Ferris：“我只是为 utf-8 编码有表示我水中好友的表情符号感到高兴。“
    ///
    /// 🐠🐙🐟🐬🐋
    /// ## 转义字符
    /// 有些字符难以使用可视字符表示，这时可通过转义字符来表示这些字符。
    ///
    /// Rust 支持类 C 语言中的常见转义字符；
    ///
    /// \n - 换行符
    /// \r - 回车符（回到本行起始位置）
    /// \t - 水平制表符（即键盘 Tab 键）
    /// \\ - 代表单个反斜杠 \
    /// \0 - 空字符（null）
    /// \' - 代表单引号 '
    /// 完整的转义字符表在这。
    ///
    /// ## 多行字符串常量
    /// Rust 中字符串默认支持分行。
    ///
    /// 使用 \ 可以使多行字符串不换行。
    ///
    /// ## 原始字符串常量
    ///
    /// 原始字符串支持写入原始的文本而无需为特殊字符转义，
    /// 因而不会导致可读性下降（如双引号与反斜杠无需写为 \" 和 \\），只需以 r#" 开头，以 "# 结尾
    ///
    /// ## 文件中的字符串常量
    ///
    /// 如果你需要使用大量文本，可以尝试用宏 include_str! 来从本地文件中导入文本到程序中：
    ///
    /// `let hello_html = include_str!("hello.html");`
    ///
    pub fn utf8_example() {
        let a: &'static str = "Ferris 说：\t\"你好\"";
        println!("{}", a);
        println!("\n \r \t \\ \0 \'");

        let haiku: &'static str = "
        我写下，擦掉，
        再写，再擦，
        然后一朵罂粟花开了。
        - 葛饰北斋";
        println!("{}", haiku);

        println!(
            "你好 \
    世界"
        ); // 注意11行 世 字前面的空格会被忽略

        let a: &'static str = r#"
        <div class="advice">
            原始字符串在一些情景下非常有用。
        </div>
        "#;
        println!("{}", a);
    }

    #[test]
    fn test_utf8_example() {
        utf8_example();
    }

    /// 字符串片段（String Slice）
    /// 字符串片段是对内存中字节序列的引用，而且这段字节序列必须是合法的 utf-8 字节序列。
    ///
    /// str 片段的字符串片段（子片段），也必须是合法的 utf-8 字节序列。
    ///
    /// &str 的常用方法：
    ///
    /// len 获取字符串常量的字节长度（不是字符长度）。
    /// starts_with/ends_with 用于基础测试。
    /// is_empty 长度为 0 时返回 true。
    /// find 返回 Option<usize>，其中的 usize 为匹配到的第一个对应文本的索引值。
    pub fn string_slice_example() {
        let a = "你好 🦀";
        println!("{}", a.len());
        let first_word = &a[0..6];
        let second_word = &a[7..11];
        // let half_crab = &a[7..9]; 报错
        // Rust 不接受无效 unicode 字符构成的片段
        println!("{} {}", first_word, second_word);
        println!("a is empty = {}", a.is_empty());
    }

    #[test]
    fn test_string_slice_example() {
        string_slice_example();
    }

    /// # Char
    /// 为了解决使用 Unicode 带来的麻烦，Rust 提供了将 utf-8 字节序列转化为类型 char 的 vector 的方法。
    ///
    /// 每个 char 长度都为 4 字节（可提高字符查找的效率）。
    pub fn char_example() {
        // 收集字符并转换为类型为 char 的 vector
        let chars = "你好 🦀".chars().collect::<Vec<char>>();
        println!("{}", chars.len()); // 结果应为 4
                                     // 由于 char 为 4 字节长，我们可以将其转化为 u32
        println!("{}", chars[3] as u32);
    }

    #[test]
    fn test_char_example() {
        char_example();
    }

    /// # 字符串（String）
    /// 字符串String 是一个结构体，其持有以堆（heap）的形式在内存中存储的 utf-8 字节序列。
    ///
    /// 由于它以堆的形式来存储，字符串可以延长、修改等等。这些都是字符串常量（string literals）无法执行的操作。
    ///
    /// 常用方法：
    ///
    /// push_str 用于在字符串的结尾添加字符串常量（&str）。
    /// replace 用于将一段字符串替换为其它的。
    /// to_lowercase/to_uppercase 用于大小写转换。
    /// trim 用于去除字符串前后的空格。
    /// 如果字符串String 被释放（drop）了，其对应的堆内存片段也将被释放。
    ///
    /// 字符串String 可以使用 + 运算符来在其结尾处连接一个 &str 并将其自身返回。但这个方法可能并不像你想象中的那么人性化。
    pub fn string_example() {
        let mut helloworld = String::from("你好");
        helloworld.push_str(" 世界");
        helloworld = helloworld + "!";
        println!("{}", helloworld.to_uppercase());

        println!("{}", "hello, world!".to_uppercase());
    }

    #[test]
    fn test_string_example() {
        string_example();
    }

    /// # 将文本作为函数的参数
    ///
    /// 字符串常量（String literals）和字符串（String）
    /// 一般以字符串片段（string slice）的形式传递给函数。这给许多场景提供了充足的灵活性，因为所有权并未被传递。
    pub fn transfer_str_to_function() {
        fn say_it_loud(msg: &str) {
            println!("{} !!", msg.to_string().to_uppercase());
        }

        // say_it_loud can borrow &'static str as a &str
        say_it_loud("你好");
        // say_it_loud can also borrow String as a &str
        say_it_loud(&String::from("再见"));
    }

    #[test]
    fn test_transfer_str_to_function() {
        transfer_str_to_function();
    }

    /// # 字符串构建
    /// concat 和 join 可以以简洁而有效的方式构建字符串。
    pub fn concat_and_join_contruct_string() {
        let helloworld = ["你好", " ", "世界", "！"].concat();
        let abc = ["a", "b", "c"].join(",");
        println!("{}", helloworld);
        println!("{}", abc);
    }

    #[test]
    fn test_concat_and_join_contruct_string() {
        concat_and_join_contruct_string();
    }

    /// 字符串格式化
    /// 宏 format! 可用于创建一个使用占位符的参数化字符串。（例：{}）
    ///
    /// format! 和 println! 生成的参数化字符串相同，只是 format! 将其返回而 println! 将其打印出来。
    ///
    /// 这个函数涉及的内容太过广泛，因而不可能在 Rust 语言之旅 中详细介绍， 如需了解完整的内容可看这里。
    pub fn string_format_example() {
        let a = 42;
        let f = format!("生活诀窍: {}", a);
        println!("{}", f);
    }

    #[test]
    fn test_string_format_example() {
        string_format_example();
    }

    /// # 字符串转换
    ///
    /// 许多类型都可以通过 to_string 转换为字符串。
    ///
    /// 而泛型函数 parse 则可将字符串或是字符串常量转换为其它类型，该函数会返回 Result 因为转换有可能失败。
    pub fn string_convert_example() -> Result<(), std::num::ParseIntError> {
        let a = 42;
        let a_string = a.to_string();
        let b = a_string.parse::<i32>()?;
        println!("{} {}", a, b);
        Ok(())
    }

    #[test]
    fn test_string_convert_example() {
        let ret = string_convert_example();
        println!("{:?}", ret);
    }

    /// 第六章 - 总结
    /// 现在你已经懂得 Rust 中文本的基础了！正如你所见，Unicode 编码的应用使文本相关操作有些棘手，但标准库中丰富的功能弥补了这一缺陷。
    ///
    /// 到目前为止，我们主要是从程序化范式的角度来看待 Rust（即只是函数和数据），但现在是时候让我们来了解一些 Rust 的面向对象范式的特性和能力了。
    pub fn chapter6_summary() {
        println!("
第六章 - 总结
现在你已经懂得 Rust 中文本的基础了！正如你所见，Unicode 编码的应用使文本相关操作有些棘手，但标准库中丰富的功能弥补了这一缺陷。

到目前为止，我们主要是从程序化范式的角度来看待 Rust（即只是函数和数据），但现在是时候让我们来了解一些 Rust 的面向对象范式的特性和能力了。
     ");
    }
}
