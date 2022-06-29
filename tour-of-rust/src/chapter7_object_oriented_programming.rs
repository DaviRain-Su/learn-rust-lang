/// # 第七章 - 面向对象编程
/// 用函数表达思想是一种成熟的行为和数据表示方式（C 语言已经这么做了几十年了！）。
/// 从历史上看，计算机科学领域已经找到了其他的数据表达聚合和抽象的方式。
/// 你可能熟知面向对象编程（OOP）就是这样一种方式。在本章中，我们将探索函数之外的 Rust 编程语言。
///
/// ## 什么是 OOP？
/// 面向对象编程大致是指具有如下一些标志性特征的编程语言：
///
/// 封装——将数据和函数关联到单一类型的概念单元中，称为对象。
/// 抽象——将数据和函数成员隐藏起来，以隐藏对象的实现细节。
/// 多态——从不同的功能角度与对象进行交互的能力。
/// 继承——从其他对象继承数据和行为的能力。
///
/// ## Rust 不是 OOP
/// Rust 缺乏任何有意义的数据和行为的继承。
///
/// 结构体不能从父结构继承字段。
/// 结构体不能从父结构继承函数。
/// 尽管如此，Rust 实现了许多编程语言的特性，所以你可能不会在意这个缺失。
pub mod content {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct SeaCreature {
        pub name: String,
        noise: String,
    }

    impl SeaCreature {
        fn get_sound(&self) -> &str {
            &self.noise
        }
    }

    /// # 使用方法进行封装
    /// Rust 支持对象的概念。“对象”是一个与一些函数（也称为方法）相关联的结构体。
    ///
    /// 任何方法的第一个参数必须是与方法调用相关联的实例的引用。(例如 instanceOfObj.foo())。Rust 使用：
    ///
    /// - &self —— 对实例的不可变引用。
    /// - &mut self —— 对实例的可变引用。
    /// 方法是在一个有 impl 关键字的实现块中定义的：
    ///
    /// `impl MyStruct {
    ///     ...
    ///     fn foo(&self) {
    ///         ...
    ///     }
    /// }`
    pub fn method_example() {
        let creature = SeaCreature {
            name: String::from("Ferries"),
            noise: String::from("blub"),
        };
        println!("{}", creature.get_sound());
    }

    #[test]
    fn test_method_example() {
        method_example();
    }

    /// # 抽象与选择性暴露
    /// Rust 可以隐藏对象的内部实现细节。
    ///
    /// 默认情况下，字段和方法只有它们所属的模块才可访问。
    ///
    /// pub 关键字可以将字段和方法暴露给模块外的访问者。
    pub fn abstract_or_public_example() {
        let creature = SeaCreature {
            name: String::from("Ferris"),
            noise: String::from("blub"),
        };
        println!("{:?}", creature);
    }

    #[test]
    fn test_abstract_or_public_example() {
        abstract_or_public_example();
    }

    /// # 使用 Trait 实现多态
    /// Rust 支持多态的特性。Trait 允许我们将一组方法与结构类型关联起来。
    ///
    /// 我们首先在 Trait 里面定义函数签名：
    /// `
    /// trait MyTrait {
    ///     fn foo(&self);
    ///     ...
    /// }`
    /// 当一个结构体实现一个 trait 时，它便建立了一个契约，允许我们通过 trait 类型与结构体进行间接交互（例如 &dyn MyTrait），而不必知道其真实的类型。
    ///
    /// 结构体实现 Trait 方法是在实现块中定义要实现的方法：
    /// `
    /// impl MyTrait for MyStruct {
    ///     fn foo(&self) {
    ///         ...
    ///     }
    ///     ...
    /// }
    /// `
    pub fn use_trait_impl_inference() {
        trait NoiseMaker {
            fn make_noise(&self);
        }

        impl NoiseMaker for SeaCreature {
            fn make_noise(&self) {
                println!("{}", self.get_sound());
            }
        }

        let creature = SeaCreature {
            name: String::from("Ferris"),
            noise: String::from("blub"),
        };
        creature.make_noise();
    }

    #[test]
    fn test_use_trait_impl_inference() {
        use_trait_impl_inference();
    }

    /// Trait 自带方法
    /// Trait 可以有已实现的方法。
    ///
    /// 这些函数并不能直接访问结构体的内部字段，但它可以在许多 trait 实现者之间共享行为。
    pub fn default_trait_method_example() {
        trait NoiseMaker {
            fn make_noise(&self);

            fn make_alot_of_noise(&self) {
                self.make_noise();
                self.make_noise();
                self.make_noise();
            }
        }

        impl NoiseMaker for SeaCreature {
            fn make_noise(&self) {
                println!("{}", self.get_sound());
            }
        }

        let creature = SeaCreature {
            name: String::from("Ferris"),
            noise: String::from("blub"),
        };
        creature.make_alot_of_noise();
    }

    #[test]
    fn test_default_trait_method_example() {
        default_trait_method_example();
    }

    /// Trait 继承
    /// Traits 可以从其他 trait 继承方法。
    pub fn trait_inherited_example() {
        trait NoiseMaker {
            fn make_noise(&self);
        }

        trait LoudNoiseMaker: NoiseMaker {
            fn make_alot_of_noise(&self) {
                self.make_noise();
                self.make_noise();
                self.make_noise();
            }
        }

        impl NoiseMaker for SeaCreature {
            fn make_noise(&self) {
                println!("{}", self.get_sound());
            }
        }

        impl LoudNoiseMaker for SeaCreature {}

        let creature = SeaCreature {
            name: String::from("Ferris"),
            noise: String::from("blub"),
        };
        creature.make_alot_of_noise();
    }

    #[test]
    fn test_trait_inherited_example() {
        trait_inherited_example();
    }

    /// 动态调度和静态调度
    /// 方法的执行有两种方式：
    ///
    /// - 静态调度——当实例类型已知时，我们直接知道要调用什么函数。
    /// - 动态调度——当实例类型未知时，我们必须想方法来调用正确的函数。
    ///
    /// Trait 类型 &dyn MyTrait 给我们提供了使用动态调度间接处理对象实例的能力。
    ///
    /// 当使用动态调度时，Rust 会鼓励你在你的 trait 类型前加上dyn，以便其他人知道你在做什么。
    ///
    /// 内存细节：
    ///
    /// 动态调度的速度稍慢，因为要追寻指针以找到真正的函数调用。
    pub fn dynamic_and_static_call() {
        trait NoiseMaker {
            fn make_noise(&self);
        }

        impl NoiseMaker for SeaCreature {
            fn make_noise(&self) {
                println!("{}", self.get_sound());
            }
        }

        fn static_make_noise(noise_maker: &SeaCreature) {
            // 我们知道真实类型
            noise_maker.make_noise();
        }

        fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
            // 我们不知道真实类型
            noise_maker.make_noise();
        }

        let creature = SeaCreature {
            name: String::from("Ferris"),
            noise: String::from("blub"),
        };

        static_make_noise(&creature);
        dynamic_make_noise(&creature);
    }

    #[test]
    fn test_dynamic_and_static_call() {
        dynamic_and_static_call();
    }

    /// # Trait 对象
    /// 当我们将一个对象的实例传递给类型为 &dyn MyTrait 的参数时，我们传递的是所谓的 trait 对象。
    ///
    /// Trait 对象允许我们间接调用一个实例的正确方法。一个 trait 对象对应一个结构。 它保存着我们实例的指针，并保有一个指向我们实例方法的函数指针列表。
    ///
    /// 内存细节：
    ///
    /// 这个函数列表在 C++ 中被称为 vtable。
    ///
    /// # 处理未知大小的数据
    /// 当我们想把 Trait 存储在另一个结构中时，它们亦带来了一个有趣的挑战。 Trait 混淆了原始结构，因此它也混淆了原来的结构体的大小。在 Rust 中，在结构体中存储未知大小的值有两种处理方式。
    ///
    /// 泛型（generics）——使用参数化类型创建已知类型的结构/函数，因此大小变成已知的。
    /// 间接存储（indirection）——将实例放在堆上，给我们提供了一个间接的层次，让我们不必担心实际类型的大小，只需存储一个指向它的指针。不过还有其他方法！
    pub fn trait_object() {
        println!("trait  object!");
    }

    /// # 泛型函数
    /// Rust中的泛型与 Trait 是相辅相成的。 当我们描述一个参数化类型 T 时，我们可以通过列出参数必须实现的 Trait 来限制哪些类型可以作为参数使用。
    ///
    /// 在以下例子中，类型 T 必须实现 Foo 这个 Trait：
    /// `
    /// fn my_function<T>(foo: T)
    ///     where
    ///         T:Foo
    /// {
    ///     ...
    /// }`
    /// 通过使用泛型，我们在编译时创建静态类型的函数，这些函数有已知的类型和大小，允许我们对其执行静态调度，并存储为有已知大小的值。
    ///
    ///
    /// 泛型函数简写
    /// Rust 为由 Trait 限制的泛型函数提供了简写形式：
    ///
    /// fn my_function(foo: impl Foo) {
    ///     ...
    /// }
    /// 这段代码等价于：
    ///
    /// fn my_function<T>(foo: T)
    ///     where
    ///         T:Foo
    /// {
    ///     ...
    /// }
    pub fn generics_function() {
        trait NoiseMaker {
            fn make_noise(&self);
        }

        impl NoiseMaker for SeaCreature {
            fn make_noise(&self) {
                println!("{}", self.get_sound());
            }
        }

        fn generic_make_noise<T>(creature: &T)
        where
            T: NoiseMaker,
        {
            // 我们在编译期就已经知道其真实类型
            creature.make_noise();
        }

        fn generic_make_noise_impl(creature: &impl NoiseMaker) {
            creature.make_noise();
        }

        let creature = SeaCreature {
            name: String::from("Ferris"),
            noise: String::from("blub"),
        };

        generic_make_noise(&creature);
        generic_make_noise_impl(&creature);
    }

    #[test]
    fn test_generics_function() {
        generics_function();
    }

    /// Box
    /// Box 是一个允许我们将数据从栈上移到堆上的数据结构。
    ///
    /// Box 是一个被称为智能指针的结构，它持有指向我们在堆上的数据的指针。
    ///
    /// 由于 Box 是一个已知大小的结构体（因为它只是持有一个指针）， 因此它经常被用在一个必须知道其字段大小的结构体中存储对某个目标的引用。
    ///
    /// Box 非常常见，它几乎可以被用在任何地方：
    ///
    /// Box::new(Foo { ... })
    pub fn box_example() {
        trait NoiseMaker {
            fn make_noise(&self);
        }

        impl NoiseMaker for SeaCreature {
            fn make_noise(&self) {
                println!("{}", self.get_sound());
            }
        }

        struct Ocean {
            animals: Vec<Box<dyn NoiseMaker>>,
        }

        let creature = SeaCreature {
            name: String::from("Ferris"),
            noise: String::from("blub"),
        };

        let sarah = SeaCreature {
            name: String::from("Sarah"),
            noise: String::from("Sarah"),
        };

        let ocean = Ocean {
            animals: vec![Box::new(creature), Box::new(sarah)],
        };

        for a in ocean.animals.iter() {
            a.make_noise();
        }
    }

    #[test]
    fn test_box_example() {
        box_example();
    }

    /// # 重温泛型结构体
    ///
    /// 泛型结构体也可以通过 Trait 来约束其参数化类型：
    /// `
    /// struct MyStruct<T>
    ///     where
    ///         T: MyTrait
    /// {
    ///     foo: T
    ///     ...
    /// }`
    /// 泛型结构体在它的实现块中有其参数化的类型：
    /// `
    /// impl<T> MyStruct<T> {
    ///     ...
    /// }`
    pub fn generic_struct() {
        println!("generic_struct!");
    }

    /// 第七章 - 总结
    /// 现在我们手头有了更多可以清晰地表达我们的想法的语言功能！ Rust 的抽象可能很简单，但它们强大到足以让我们写代码写得很愉快。 在本章中，我们通过 Box 简单瞥见了智能指针。在下一章中，我们将了解智能指针如何帮助我们处理其他特定的内存情况。
    ///
    /// 其他资源（英文）：
    ///
    /// 视频 - [Object-oriented Programming in 7 minutes](https://www.youtube.com/watch?v=pTB0EiLXUC8)
    /// 文章 - ["The faster you unlearn OOP, the better for you and your software"](https://dpc.pw/the-faster-you-unlearn-oop-the-better-for-you-and-your-software)
    pub fn chapter1_summary() {
        println!(
            r##"第七章 - 总结
现在我们手头有了更多可以清晰地表达我们的想法的语言功能！ Rust 的抽象可能很简单，但它们强大到足以让我们写代码写得很愉快。 在本章中，我们通过 Box 简单瞥见了智能指针。在下一章中，我们将了解智能指针如何帮助我们处理其他特定的内存情况。

其他资源（英文）：

视频 - Object-oriented Programming in 7 minutes
文章 -    "The faster you unlearn OOP, the better for you and your software""##
        );
    }
}
