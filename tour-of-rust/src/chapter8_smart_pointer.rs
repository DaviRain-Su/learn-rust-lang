/// # 第8章 - 智能指针
///
/// 在本章中，我们将揭开智能指针的神秘面纱。
/// 让我们探索一下能够让我们与最底层内存打交道的这些数据结构。
/// Ferris 说：“读完这一章之后，即使您觉得仍然不能编写管理底层内存的代码也不用觉得不知所措。
/// 本章主要是向您介绍一些有用的工具并简要了解他们如何工作！”
///
/// ## 重温引用
///
/// 引用本质上只是表示内存中某些字节起始位置的数字。
/// 它唯一的目的就是表示特定类型的数据存在于何处。
/// 引用与数字的不同之处在于，
/// Rust 将验证引用自身的生命周期不会超过它指向的内容（否则我们在使用它时会出错！）。
pub mod content {

    /// # 指针
    ///
    /// 引用可以转换成一个更原始的类型，指针(raw pointer)。 像数字一样，
    /// 它可以不受限制地复制和传递，但是Rust 不保证它指向的内存位置的有效性。 有两种指针类型：
    ///
    /// - *const T - 指向永远不会改变的 T 类型数据的指针。
    /// - *mut T - 指向可以更改的 T 类型数据的指针。
    ///
    /// 指针可以与数字相互转换（例如usize）。
    /// 指针可以使用 unsafe 代码访问数据（稍后会详细介绍）。
    ///
    /// 内存细节：
    ///
    /// - Rust中的引用在用法上与 C 中的指针非常相似，但在如何存储和传递给其他函数上有更多的编译时间限制。
    /// - Rust中的指针类似于 C 中的指针，它表示一个可以复制或传递的数字，甚至可以转换为数字类型，
    /// 可以将其修改为数字以进行指针数学运算。
    ///
    /// ## 解引用
    ///
    /// 访问或操作 由引用（例如&i32）指向的数据的过程称为解除引用。
    /// 有两种方式通过引用来访问或操作数据：
    ///
    /// - 在变量赋值期间访问引用的数据。
    /// - 访问引用数据的字段或方法。
    ///
    /// Rust 有一些强大的运算符可以让我们做到这一点。
    ///
    /// # 运算符 *
    ///
    /// * 运算符是一种很明确的解引用的方法。
    ///
    /// ```no
    /// let a: i32 = 42;
    /// let ref_ref_ref_a: &&&i32 = &&&a;
    /// let ref_a: &i32 = **ref_ref_ref_a;
    /// let b: i32 = *ref_a;
    /// ```
    /// 内存细节:
    ///
    /// 因为 i32 是实现了 Copy 特性的原始类型，堆栈上变量 a 的字节被复制到变量 b 的字节中。
    ///
    /// ## 运算符 .
    /// .运算符用于访问引用的字段和方法，它的工作原理更加巧妙。
    /// ```no
    /// let f = Foo { value: 42 };
    /// let ref_ref_ref_f = &&&f;
    /// println!("{}", ref_ref_ref_f.value);
    /// ```
    /// 哇，为什么我们不需要在ref_ref_ref_f之前添加***？
    /// 这是因为 . 运算符会做一些列自动解引用操作。 最后一行由编译器自动转换为以下内容。
    /// ```no
    /// println!("{}", (***ref_ref_ref_f).value);
    /// ```
    pub fn pointer_example() {
        // &
        let a = 42;
        let memory_location = &a as *const i32 as usize;
        println!("Data is here {}", memory_location);

        // *
        let a: i32 = 42;
        let ref_ref_ref_a: &&&i32 = &&&a;
        let ref_a: &i32 = **ref_ref_ref_a;
        let b: i32 = *ref_a;
        println!("{}", b);

        struct Foo {
            value: i32,
        }

        let f = Foo { value: 32 };
        let ref_ref_ref_f = &&&f;
        println!("{}", ref_ref_ref_f.value);
    }

    #[test]
    fn test_pointer_example() {
        pointer_example();
    }

    /// # 智能指针
    ///
    /// 除了能够使用&运算符创建对现有类型数据的引用之外, Rust 给我们提供了能够创建称为智能指针的类引用结构。
    /// 我们可以在高层次上将引用视为一种类型，它使我们能够访问另一种类型. 智能指针的行为与普通引用不同，
    /// 因为它们基于程序员编写的内部逻辑进行操作. 作为程序员的你就是智能的一部分。
    /// 通常，智能指针实现了 `Deref`、`DerefMut` 和 `Drop` 特征，以指定当使用 `*` 和 `.` 运算符时解引用应该触发的逻辑。
    pub fn smart_pointer_example() {
        use std::ops::Deref;
        struct TattleTell<T> {
            value: T,
        }

        impl<T> Deref for TattleTell<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                println!("{} was used!", std::any::type_name::<T>());
                &self.value
            }
        }

        let foo = TattleTell {
            value: "secret message",
        };
        // dereference occurs here immediately
        // after foo is auto-referenced for the
        // function `len`
        println!("{}", foo.len());

        let bar = TattleTell {
            value: vec![1, 2, 3],
        };

        println!("{}", bar.len());
    }

    #[test]
    fn test_smart_pointer_example() {
        smart_pointer_example();
    }

    /// # 智能不安全代码
    ///
    /// 智能指针倾向于经常使用不安全的代码。如前所述，它们是与 Rust 中最低级别的内存进行交互的常用工具。
    /// 什么是不安全代码? 不安全代码的行为与普通 Rust 完全一样，除了一些 Rust 编译器无法保证的功能。
    /// 不安全代码的主要功能是解引用指针。
    /// 这意味着将原始指针指向内存中的某个位置并声明“此处存在数据结构！”
    /// 并将其转换为您可以使用的数据表示（例如将`*const u8` 转换为`u8`）。
    /// Rust 无法跟踪写入内存的每个字节的含义。
    /// 因为 Rust 不能保证在用作 指针 的任意数字上存在什么，
    /// 所以它将解引用放在一个 `unsafe { ... }` 块中。
    ///
    /// 智能指针广泛地被用来解引用指针，它们的作用得到了很好的证明。
    pub fn unsafe_smart_pointer_example() {
        let a: [u8; 4] = [86, 14, 73, 64];
        // this is a raw pointer. Getting the memory address
        // of something as a number is totally safe
        let pointer_a = &a as *const u8 as usize;
        println!("Data memory location: {}", pointer_a);
        // Turning our number into a raw pointer to a f32 is
        // also safe to do.
        let pointer_b = pointer_a as *const f32;
        let b = unsafe {
            // This is unsafe because we are telling the compiler
            // to assume our pointer is a valid f32 and
            // dereference it's value into the variable b.
            // Rust has no way to verify this assumption is true.
            *pointer_b
        };
        println!("I swear this is a pie! {}", b);
    }

    #[test]
    fn test_unsafe_small_pointer_example() {
        unsafe_smart_pointer_example();
    }

    /// # 熟悉的朋友
    ///
    /// 想一想一些我们已经见过的智能指针，例如 Vec<T> 和 String。
    ///
    /// `Vec<T>` 是一个智能指针，它只拥有一些字节的内存区域。
    /// Rust 编译器不知道这些字节中存在着什么。 智能指针解释从它管理的内存区域获取数据意味着什么，
    /// 跟踪这些字节中的数据结构开始和结束的位置，最后将指针解引用到数据结构中，
    /// 成为一个漂亮干净的可以阅读的接口供我们使用（例如`my_vec[3]`）。
    ///
    /// 类似地，`String` 跟踪字节的内存区域，并以编程方式将写入其中的内容限制为始终有效的 utf-8，
    /// 并帮助将该内存区域解引用为类型 `&str`。
    ///
    /// 这两种数据结构都使用不安全的解引用指针来完成它们的工作。
    ///
    /// 内存细节：
    ///
    /// - Rust 有一个相当于 C 的 `malloc`方法， `alloc` 和 `Layout` 来获取你自己管理的内存区域。
    pub fn smart_pointer_example_2() {
        use std::alloc::{alloc, Layout};
        use std::ops::Deref;

        struct Pie {
            secret_recipe: usize,
        }

        impl Pie {
            fn new() -> Self {
                // let's ask for 4 bytes
                let layout = Layout::from_size_align(4, 1).unwrap();

                unsafe {
                    // allocate and save the memory location as a number
                    let ptr = alloc(layout) as *mut u8;
                    // use pointer math and write a few
                    // u8 values to memory
                    ptr.add(0).write(86);
                    ptr.add(1).write(14);
                    ptr.add(2).write(73);
                    ptr.add(3).write(64);

                    Pie {
                        secret_recipe: ptr as usize,
                    }
                }
            }
        }
        impl Deref for Pie {
            type Target = f32;
            fn deref(&self) -> &f32 {
                // interpret secret_recipe pointer as a f32 raw pointer
                let pointer = self.secret_recipe as *const f32;
                // dereference it into a return value &f32
                unsafe { &*pointer }
            }
        }

        let p = Pie::new();
        // "make a pie" by dereferencing our
        // Pie struct smart pointer
        println!("{:?}", *p);
    }

    #[test]
    fn test_smart_pointer_example_2() {
        smart_pointer_example_2();
    }

    /// # 堆分配内存
    ///
    /// Box 是一个可以让我们将数据从堆栈移动到堆的智能指针。
    ///
    /// 解引用可以让我们以人类更容易理解的方式使用堆分配的数据，就好像它是原始类型一样。
    pub fn box_usage_example() {
        struct Pie;

        impl Pie {
            fn eat(&self) {
                println!("tastes better on the heap!");
            }
        }

        let heap_pie = Box::new(Pie);
        heap_pie.eat();
    }

    #[test]
    fn test_box_usage_example() {
        box_usage_example();
    }

    /// # 重温error的使用
    ///
    /// Rust可能有过多的错误表示方法，但标准库有一个通用特性 `std::error::Error` 来描述错误。
    ///
    /// 使用智能指针“`Box`”，我们可以使用类型`Box<dyn std::error::Error>`作为常见的返回错误类型，
    /// 因为它允许我们在堆上、高级别的传播错误，而不必知道特定的类型。
    ///
    /// 在 `Rust` 之旅的早期，我们了解到 `main` 可以返回一个错误。我们现在可以返回一个类型，
    /// 该类型能够描述我们程序中可能发生的几乎任何类型的错误，只要错误的数据结构实现了 `Rust` 的通用`Error`特征。
    ///
    /// `fn main() -> Result<(), Box<dyn std::error:Error>>`
    pub fn error_usage_example() {
        use std::error::Error;
        use std::fmt::Display;
        use std::fmt::Formatter;

        struct Pie;

        #[derive(Debug)]
        struct NotFreshError;

        impl Display for NotFreshError {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "This pie is not fresh!")
            }
        }
        impl Error for NotFreshError {}

        impl Pie {
            fn eat(&self) -> Result<(), Box<dyn std::error::Error>> {
                Err(Box::new(NotFreshError))
            }
        }

        let heap_pie = Box::new(Pie);
        let ret = heap_pie.eat();

        println!("ret = {:?}", ret);
    }

    #[test]
    fn test_error_usage_example() {
        error_usage_example();
    }

    /// # 引用计数
    ///
    /// Rc 是一个能将数据从栈移动到智能指针。 它允许我们克隆其他Rc智能指针，
    /// 这些指针都具有不可改变地借用放在堆上的数据的能力。
    /// 只有当最后一个智能指针被删除时，堆上的数据才会被释放。
    pub fn ref_counter_example() {
        use std::rc::Rc;
        struct Pie;

        impl Pie {
            fn eat(&self) {
                println!("tastes better on the heap!")
            }
        }
        let heap_pie = Rc::new(Pie);
        let heap_pie2 = heap_pie.clone();
        let heap_pie3 = heap_pie2.clone();

        heap_pie3.eat();
        heap_pie2.eat();
        heap_pie.eat();

        // all reference count smart pointers are dropped now
        // the heap data Pie finally deallocates
    }

    #[test]
    fn test_ref_counter_example() {
        ref_counter_example();
    }

    /// # 共享访问
    ///
    /// `RefCell` 是一个容器数据结构，通常由智能指针拥有，它接收数据并让我们借用可变或不可变引用来访问内部内容。
    /// 当您要求借用数据时，它通过在运行时强制执行 Rust 的内存安全规则来防止借用被滥用
    ///
    /// 只有一个可变引用或多个不可变引用，但不能同时有！
    ///
    /// 如果你违反了这些规则，`RefCell` 将会`panic`。
    pub fn ref_cell_usage_example() {
        use std::cell::RefCell;
        struct Pie {
            slices: u8,
        }

        impl Pie {
            fn eat(&mut self) {
                println!("tastes better on the heap!");
                self.slices -= 1;
            }
        }

        // RefCell validates memory safety at runtime
        // notice: pie_cell is not mut!
        let pie_cell = RefCell::new(Pie { slices: 8 });

        {
            // but we can borrow mutable references!
            let mut mut_ref_pie = pie_cell.borrow_mut();
            mut_ref_pie.eat();
            mut_ref_pie.eat();

            // mut_ref_pie is dropped at end of scope
        }

        // now we can borrow immutably once our mutable reference drops
        let ref_pie = pie_cell.borrow();
        println!("{} slices left", ref_pie.slices);
    }

    #[test]
    fn test_ref_cell_usage_example() {
        ref_cell_usage_example();
    }

    /// # 线程间共享
    ///
    /// Mutex 是一种容器数据结构，通常由智能指针持有，它接收数据并让我们借用对其中数据的可变和不可变引用。
    /// 这可以防止借用被滥用，因为操作系统一次只限制一个 CPU 线程访问数据，阻塞其他线程，直到原线程完成其锁定的借用。
    ///
    /// 多线程超出了 Rust 之旅的范围，但 `Mutex` 是协调多个 CPU 线程访问相同数据的基本部分。
    ///
    /// 有一个特殊的智能指针 `Arc`，它与 `Rc` 相同，除了使用线程安全的引用计数递增。 它通常用于对同一个 Mutex 进行多次引用
    pub fn mutex_usage_example() {
        use std::sync::Mutex;

        struct Pie;

        impl Pie {
            fn eat(&self) {
                println!("only I eat the pie!");
            }
        }

        let mutex_pie = Mutex::new(Pie);

        // let's borrow a locked immutable reference of pie
        // we have to unwrap the result of a lock
        // because it might fail
        let ref_pie = mutex_pie.lock().unwrap();
        ref_pie.eat();
        // locked reference drops here, and mutex protected value can be used by someone else
    }

    #[test]
    fn test_mutex_usage_example() {
        mutex_usage_example();
    }

    /// # 组合智能指针
    ///
    /// 智能指针看起来可能会存在一些限制，但是我们可以做一些非常有用的结合。
    ///
    /// - `Rc<Vec<Foo>>` - 允许克隆多个可以借用堆上不可变数据结构的相同向量的智能指针。
    ///
    /// - `Rc<RefCell<Foo>>` - 允许多个智能指针可变/不可变地借用相同的结构Foo
    ///
    /// - `Arc<Mutex<Foo>>` - 允许多个智能指针以 CPU 线程独占方式锁定临时可变/不可变借用的能力。
    ///
    /// 内存细节：
    ///
    /// - 您会注意到一个包含许多这些组合的主题。
    /// - 使用不可变数据类型（可能由多个智能指针拥有）来修改内部数据。
    /// - 这在 Rust 中被称为“内部可变性”模式。
    /// - 这种模式让我们可以在运行时以与 Rust 的编译时检查相同的安全级别来改变内存使用规则。
    pub fn compoent_smart_pointer_example() {
        use std::cell::RefCell;
        use std::rc::Rc;

        struct Pie {
            slices: u8,
        }

        impl Pie {
            fn eat_slice(&mut self, name: &str) {
                println!("{} took a slice!", name);
                self.slices -= 1;
            }
        }

        struct SeaCreature {
            name: String,
            pie: Rc<RefCell<Pie>>,
        }

        impl SeaCreature {
            fn eat(&self) {
                // use smart pointer to pie for a mutable borrow
                let mut p = self.pie.borrow_mut();
                // take a bite!
                p.eat_slice(&self.name);
            }
        }

        let pie = Rc::new(RefCell::new(Pie { slices: 8 }));
        // ferris and sarah are given clones of smart pointer to pie
        let ferris = SeaCreature {
            name: String::from("ferris"),
            pie: pie.clone(),
        };
        let sarah = SeaCreature {
            name: String::from("sarah"),
            pie: pie.clone(),
        };
        ferris.eat();
        sarah.eat();

        let p = pie.borrow();
        println!("{} slices left", p.slices);
    }

    #[test]
    fn test_compoent_smart_pointer_example() {
        compoent_smart_pointer_example();
    }

    /// # 第8章 - 总结
    ///
    /// 智能指针是 Rust编程中经常使用的，它可以让我们不必重新创建非常常见的内存使用范式。
    /// 有了它，您可以准备好应对最艰难的挑战了！ 现在我们掌握了 Rust 的基础，
    /// 让我们来谈谈如何编写更庞大的项目。 在下一章中，我们将摆脱一个文件包含所有代码的束缚。
    pub fn chapter8_summary() {
        println!("
第8章 - 总结
智能指针是 Rust编程中经常使用的，它可以让我们不必重新创建非常常见的内存使用范式。
 有了它，您可以准备好应对最艰难的挑战了！ 现在我们掌握了 Rust 的基础，让我们来谈谈如何编写更庞大的项目。
 在下一章中，我们将摆脱一个文件包含所有代码的束缚。
        ");
    }
}
