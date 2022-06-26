/// # 第三章 - 基本数据结构类型
/// 现在是我们探索基本类型之外内容的时候了！
/// 在本章中，我们将查看 Rust 中最原始的数据结构， 并密切关注它们在内存中的表示。
/// 我想你们会喜欢上 Rust 很少隐藏事物运作原理这一点的。
pub mod content {
    /// # 结构体
    /// 一个 struct 就是一些字段的集合。
    ///
    /// 字段是一个与数据结构相关联的数据值。它的值可以是基本类型或结构体类型。
    ///
    /// 它的定义就像给编译器的蓝图，告诉编译器如何在内存中布局彼此相邻的字段。
    ///
    /// ## Code
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct SeaCreature {
    ///     animal_type: String,
    ///     name: String,
    ///     arms: i32,
    ///     legs: i32,
    ///     weapon: String,
    /// }
    /// ```
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct SeaCreature {
        animal_type: String,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String,
    }

    /// # 方法调用
    /// 与函数（function）不同，方法（method）是与特定数据类型关联的函数。
    ///
    /// - 静态方法 — 属于某个类型，调用时使用 `::` 运算符。
    ///
    /// - 实例方法 — 属于某个类型的实例，调用时使用 `.` 运算符。
    ///
    /// 我们将在后续章节中更多地讨论如何创建自己的方法。
    pub fn method_call() {
        // call static function
        let s = String::from("Hello, world!");
        // call instance method
        println!("{} is {} characters long.", s, s.len());
    }

    #[test]
    fn test_method_call() {
        method_call();
    }

    /// # 内存
    /// Rust 程序有 3 个存放数据的内存区域：
    ///
    /// - 数据内存 - 对于固定大小和静态（即在整个程序生命周期中都存在）的数据。
    /// 考虑一下程序中的文本（例如 “Hello World”），该文本的字节只能读取，因此它们位于该区域中。
    /// 编译器对这类数据做了很多优化，由于位置已知且固定，因此通常认为编译器使用起来非常快。
    /// - 栈内存 - 对于在函数中声明为变量的数据。
    /// 在函数调用期间，内存的位置不会改变，因为编译器可以优化代码，
    /// 所以栈数据使用起来比较快。
    /// - 堆内存 - 对于在程序运行时创建的数据。
    /// 此区域中的数据可以添加、移动、删除、调整大小等。
    /// 由于它的动态特性，通常认为它使用起来比较慢， 但是它允许更多创造性的内存使用。
    /// 当数据添加到该区域时，我们称其为分配。 从本区域中删除 数据后，我们将其称为释放。
    ///
    /// # 在内存中创建数据
    /// 当我们在代码中实例化一个结构体时，我们的程序会在内存中并排创建关联的字段数据。
    ///
    /// 当我们通过制定所有字段值的方式来实例化时：
    ///
    /// `结构体名 { ... }`.
    ///
    /// 结构体字段可以通过 `.` 运算符来获取。
    ///
    /// 我们例子的内存详情：
    ///
    /// - 引号内的文本是只读数据（例如“ferris”），因此它位于数据内存区。
    /// - 函数调用 String::from 创建一个结构体 String，该结构体与 SeaCreature 的字段并排放置在栈中。
    /// 字符串容器通过如下步骤表示可更改的文本：
    ///     - 在堆上创建可修改文本的内存。
    ///     - 将堆中存储对象的内存位置的引用存储在 String 结构体中（在以后的课程中会详细介绍）。
    /// 最后，我们的两个朋友 Ferris 和 Sarah 有在程序中总是固定的位置的数据结构，所以它们被放在栈上。
    ///
    /// ## Code
    ///
    /// ```rust
    /// #[allow(dead_code)]
    /// #[derive(Debug)]
    /// pub struct SeaCreature {
    ///     animal_type: String,
    ///     name: String,
    ///     arms: i32,
    ///     legs: i32,
    ///     weapon: String,
    /// }
    ///
    /// let ferris = SeaCreature {
    ///     animal_type: String::from("pang xie"),
    ///     name: String::from("Ferris"),
    ///     arms: 2,
    ///     legs: 4,
    ///     weapon: String::from("da qian zi"),
    /// };
    ///
    /// let sarah = SeaCreature {
    ///     animal_type: String::from("zhang yu"),
    ///     name: String::from("Sarah"),
    ///     arms: 8,
    ///     legs: 0,
    ///     weapon: String::from("None"),
    /// };
    ///
    /// println!("ferris type : {:#?}", ferris);
    /// println!("sarah type: {:#?}",sarah);
    /// ```
    pub fn create_sea_creature_type() {
        let ferris = SeaCreature {
            animal_type: String::from("pang xie"),
            name: String::from("Ferris"),
            arms: 2,
            legs: 4,
            weapon: String::from("da qian zi"),
        };

        let sarah = SeaCreature {
            animal_type: String::from("zhang yu"),
            name: String::from("Sarah"),
            arms: 8,
            legs: 0,
            weapon: String::from("None"),
        };

        println!("ferris type : {:#?}", ferris);
        println!("sarah type: {:#?}", sarah);
    }

    #[test]
    fn test_create_sea_creature_type() {
        create_sea_creature_type();
    }

    /// # 类元组结构体
    ///
    /// 简洁起见，你可以创建像元组一样被使用的结构体。
    ///
    /// ## Code
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct  Location(i32,i32);
    ///```
    #[derive(Debug)]
    pub struct Location(i32, i32);

    #[test]
    fn test_location_type() {
        let loc = Location(32, 23);
        println!("Location type : {:#?}", loc);
    }

    /// # 类单元结构体
    ///
    /// 结构体也可以没有任何字段。
    ///
    /// 就像第一章提到的，一个 unit 是空元组 () 的别称。这就是为什么，此类结构体被称为 类单元。
    ///
    /// 这种类型的结构体很少用到。
    ///
    /// ## Code
    /// ```rust
    /// #[derive(Debug)]
    /// pub struct  Marker;
    /// ```
    #[derive(Debug)]
    pub struct Marker;

    #[test]
    fn test_class_unit_struct_type() {
        let m = Marker;
        println!("Marker: {:?}", m);
    }

    /// # 枚举
    ///
    /// 枚举允许你使用 `enum` 关键字创建一个新类型，该类型的值可以包含几个带标记的元素。
    ///
    /// `match` 有助于确保对所有可能的枚举值进行彻底的处理，使其成为确保高质量代码的强大工具。
    ///
    /// ## code
    ///```rust
    /// #[derive(Debug)]
    /// pub enum Species {
    ///     Crab,
    ///     Octopus,
    ///     Fish,
    ///     Clam
    /// }
    /// ```
    #[derive(Debug)]
    pub enum Species {
        Crab,
        Octopus,
        Fish,
        Clam,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct SeaCreatures {
        species: Species,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String,
    }

    #[test]
    fn test_enums_type() {
        let ferris = SeaCreatures {
            species: Species::Crab,
            name: String::from("Ferris"),
            arms: 2,
            legs: 4,
            weapon: String::from("claw"),
        };

        match ferris.species {
            Species::Crab => {
                println!("{} is a crab", ferris.name);
            }
            Species::Octopus => {
                println!("{} is a octopus", ferris.name);
            }
            Species::Fish => {
                println!("{} is fish", ferris.name);
            }
            Species::Clam => {
                println!("{} is a clam", ferris.name);
            }
        }
    }

    pub enum PositionType {
        Acidic,
        Painful,
        Lethal,
    }

    pub enum Size {
        Big,
        Small,
    }

    /// # 带数据的枚举
    ///
    /// `enum` 的元素可以有一个或多个数据类型，从而使其表现得像 C 语言中的联合。
    ///
    /// 当使用 `match` 对一个 `enum` 进行模式匹配时，可以将变量名称绑定到每个数据值。
    ///
    /// `enum` 的内存细节：
    /// - 枚举数据的内存大小等于它最大元素的大小。此举是为了让所有可能的值都能存入相同的内存空间。
    /// - 除了元素数据类型（如果有）之外，每个元素还有一个数字值，用于表示它是哪个标签。
    ///
    /// 其他细节：
    /// - Rust 的 `enum` 也被称为标签联合 （tagged-union）
    /// - 把类型组合成一种新的类型，这就是人们所说的 Rust 具有 代数类型 的含义。
    ///
    /// ## code
    /// ```rust
    /// pub enum Weapon {
    ///  Claw(i32, Size),
    ///  Position(PositionType),
    ///  None,
    /// }
    /// pub enum  PositionType {
    ///     Acidic,
    ///     Painful,
    ///     Lethal,
    /// }
    ///
    /// pub enum Size {
    ///     Big,
    ///     Small
    /// }
    /// ```
    #[allow(dead_code)]
    pub enum Weapon {
        Claw(i32, Size),
        Position(PositionType),
        None,
    }

    #[allow(dead_code)]
    pub struct SeaCreature3 {
        species: Species,
        name: String,
        arms: i32,
        legs: i32,
        weapon: Weapon,
    }

    #[test]
    fn test_with_data_enum_type() {
        let ferris = SeaCreature3 {
            species: Species::Crab,
            name: String::from("Ferris"),
            arms: 2,
            legs: 4,
            weapon: Weapon::Claw(2, Size::Small),
        };

        match ferris.species {
            Species::Crab => match ferris.weapon {
                Weapon::Claw(num_clawns, size) => {
                    let size_description = match size {
                        Size::Big => "big",
                        Size::Small => "small",
                    };
                    println!(
                        "ferris is a crab with {} {} claws",
                        num_clawns, size_description
                    );
                }
                _ => println!("ferris is a crab with  some other weapon!"),
            },
            _ => println!("ferris is some other animal"),
        }
    }

    /// # 第三章 - 总结
    ///
    /// 太好了！现在我们有了一些用代码来展示我们想法最基本的工具。
    /// 希望现在我们能看到 Rust 的基本操作是如何与它的类型和谐一致地工作的。
    /// 接下来我们将讨论一个概念，它为我们的数据类型提供了更大的灵活性：泛型。
    pub fn chapter3_summary() {
        println!(
            "
第三章 - 总结
太好了！现在我们有了一些用代码来展示我们想法最基本的工具。
希望现在我们能看到 Rust 的基本操作是如何与它的类型和谐一致地工作的。
接下来我们将讨论一个概念，它为我们的数据类型提供了更大的灵活性：泛型。
        "
        );
    }
}
