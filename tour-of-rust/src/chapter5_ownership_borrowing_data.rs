/// # 第五章 - 所有权和数据借用
///
/// 相较于其他编程语言，Rust 具有一套独特的内存管理范例。
/// 为了不让您被概念性的东西淹没，我们将一一展示这些编译器的行为和验证方式。
/// 有一点很重要：所有这些规则的终极目的不是为了为难您，而是为了更好地降低代码的出错率！
pub mod content {

    /// Foo Type
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Foo {
        pub x: i32,
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Drop Foo");
        }
    }

    /// # 所有权
    /// 实例化一个类型并且将其绑定到变量名上将会创建一些内存资源，
    /// 而这些内存资源将会在其整个生命周期中被 Rust 编译器检验。 被绑定的变量即为该资源的所有者。
    pub fn what_is_ownership() {
        // 我们实例化这个结构体并将其绑定到具体的变量上
        // 来创建内存资源
        let foo = Foo { x: 42 };
        // foo 即为该资源的所有者
        println!("foo = {:?}", foo);
    }

    #[test]
    fn test_what_is_ownership() {
        what_is_ownership();
    }

    /// # 基于域的资源管理
    ///
    /// Rust 将使用资源最后被使用的位置或者一个函数域的结束来作为资源被析构和释放的地方。 此处析构和释放的概念被称之为 drop（丢弃）。
    ///
    /// 内存细节：
    /// - Rust 没有垃圾回收机制。
    /// - 在 C++ 中，这被也称为“资源获取即初始化“（RAII）
    pub fn base_block_resource_manager() {
        let foo_a = Foo { x: 32 };
        let foo_b = Foo { x: 12 };

        println!("foo a : {:?}", foo_a);
        // foo_a 将在这里被 dropped 因为其在这之后再也没有被使用
        println!("foo b : {:?}", foo_b);
        // foo_b 将在这里被 dropped 因为这是函数域的结尾
    }

    #[test]
    fn test_base_block_resource_manager() {
        base_block_resource_manager();
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Bar {
        x: i32,
    }

    impl Drop for Bar {
        fn drop(&mut self) {
            println!("Drop Bar");
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Boo {
        bar: Bar,
    }

    impl Drop for Boo {
        fn drop(&mut self) {
            println!("Drop Boo");
        }
    }

    /// # 释放是分级进行的
    ///
    /// 删除一个结构体时，结构体本身会先被释放，紧接着才分别释放相应的子结构体并以此类推。
    ///
    /// 内存细节：
    /// - Rust 通过自动释放内存来帮助确保减少内存泄漏。
    /// - 每个内存资源仅会被释放一次
    pub fn drop_access() {
        let boo = Boo { bar: Bar { x: 42 } };
        println!("boo = {:?}", boo);
        // boo 首先被 dropped 释放
        // 紧接着是 boo.bar
    }

    #[test]
    fn test_drop_access() {
        drop_access();
    }

    /// # 移交所有权
    ///
    /// 将所有者作为参数传递给函数时，其所有权将移交至该函数的参数。 在一次移动后，原函数中的变量将无法再被使用。
    ///
    /// 内存细节:
    /// - 在移动期间，所有者的堆栈值将会被复制到函数调用的参数堆栈中
    pub fn transfer_ownership() {
        fn do_something(f: Foo) {
            println!("{:?}", f);
            // f here will be drop and release.
        }

        let foo = Foo { x: 32 };
        // foo transfer to do_something
        do_something(foo);
        // foo never to used.
    }

    #[test]
    fn test_transfer_ownership() {
        transfer_ownership();
    }

    /// # 归还所有权
    ///
    /// 所有权也可以从一个函数中被归还。
    pub fn transfer_ownership_to_self() {
        fn do_something() -> Foo {
            Foo { x: 3 }
        }
        // ownership will be remove to

        let _foo = do_something();

        // foo is ownership
        // foo will be dropped
    }

    #[test]
    fn test_transfer_ownership_to_self() {
        transfer_ownership_to_self();
    }

    /// # 使用引用借用所有权
    ///
    /// 引用允许我们通过 `&` 操作符来借用对一个资源的访问权限。 引用也会如同其他资源一样被释放。
    pub fn use_ref_borrow_ownership() {
        let foo = Foo { x: 32 };
        let f = &foo;
        println!("foo = {:?}", f);
        // f will be dropped
        // foo will be dropped
    }

    #[test]
    fn test_use_ref_borrow_ownership() {
        use_ref_borrow_ownership();
    }

    /// # 通过引用借用可变所有权
    ///
    /// 我们也可以使用 `&mut` 操作符来借用对一个资源的可变访问权限。 在发生了可变借用后，一个资源的所有者便不可以再次被借用或者修改。
    ///
    /// 内存细节：
    ///
    /// Rust 之所以要避免同时存在两种可以改变所拥有变量值的方式，是因为此举可能会导致潜在的数据争用（data race）。
    pub fn use_mut_ref_borrow_ownership() {
        fn do_something(f: Foo) {
            println!("{:?}", f);
            // f will be dropped.
        }

        let mut foo = Foo { x: 32 };
        let f = &mut foo;

        // 会报错:
        // do_something(foo);
        // 因为 foo 已经被可变借用而无法取得其所有权

        // 会报错: foo.x = 13;
        // 因为 foo 已经被可变借用而无法被修改

        f.x = 13;
        // f 会因为此后不再被使用而被 dropped 释放

        println!("{}", foo.x);

        // 现在修改可以正常进行因为其所有可变引用已经被 dropped 释放
        foo.x = 7;

        // 移动 foo 的所有权到一个函数中
        do_something(foo);
    }

    #[test]
    fn test_use_mut_ref_borrow_ownership() {
        use_mut_ref_borrow_ownership();
    }

    /// 解引用
    ///
    /// 使用 `&mut` 引用时, 你可以通过 `*` 操作符来修改其指向的值。
    /// 你也可以使用 `*` 操作符来对所拥有的值进行拷贝（前提是该值可以被拷贝——我们将会在后续章节中讨论可拷贝类型）
    pub fn deref_value() {
        let mut foo = 23;
        let f = &mut foo;
        let bar = *f; // 取得所有者值的拷贝
        *f = 12; // 设置引用所有者的值
        println!("bar = {}", bar);
        println!("foo = {}", foo);
    }

    #[test]
    fn test_deref_value() {
        deref_value();
    }

    /// # 传递借用的数据
    /// Rust 对于引用的规则也许最好用以下的方式总结：
    ///
    /// - Rust 只允许同时存在一个可变引用或者多个不可变引用，不允许可变引用和不可变引用同时存在。
    /// - 一个引用永远也不会比它的所有者存活得更久。
    ///
    /// 而在函数间进行引用的传递时，以上这些通常都不会成为问题。
    ///
    /// 内存细节：
    ///
    /// - 上面的第一条规则避免了数据争用的出现。什么是数据争用？
    /// 在对数据进行读取的时候，数据争用可能会因为同时存在对数据的写入而产生不同步。这一点往往会出现在多线程编程中。
    /// - 而第二条引用规则则避免了通过引用而错误的访问到不存在的数据（在 C 语言中被称之为悬垂指针）。
    pub fn transfer_borrow_value() {
        fn do_something(f: &mut Foo) {
            f.x += 1;
            // 可变引用 f 在这里被 dropped 释放
        }

        let mut foo = Foo { x: 323 };
        do_something(&mut foo);
        println!("Foo : {:?}", foo);
        // 因为所有的可变引用都在 do_something 函数内部被释放了
        // 此时我们便可以再创建一个
        do_something(&mut foo);
        // foo 在这里被 dropped 释放

        println!("Foo : {:?}", foo);
    }

    #[test]
    fn test_transfer_borrow_value() {
        transfer_borrow_value();
    }

    /// # 引用的引用
    ///
    /// 引用甚至也可以用在其他引用上。
    pub fn ref_and_ref_example() {
        fn do_something(a: &Foo) -> &i32 {
            &a.x
        }

        let mut foo = Foo { x: 32 };
        let x = &mut foo.x;
        *x = 12;
        // x 在这里被 dropped 释放从而允许我们再创建一个不可变引用
        let y = do_something(&foo);
        println!("y = {}", y);
        // y 在这里被 dropped 释放
        // foo 在这里被 dropped 释放
    }

    #[test]
    fn test_ref_and_ref_example() {
        ref_and_ref_example();
    }

    /// # 显式生命周期
    ///
    /// 尽管 Rust 不总是在代码中将它展示出来，
    /// 但编译器会理解每一个变量的生命周期并进行验证以确保一个引用不会有长于其所有者的存在时间。
    /// 同时，函数可以通过使用一些符号来参数化函数签名，以帮助界定哪些参数和返回值共享同一生命周期。
    /// 生命周期注解总是以 `'` 开头，例如 `'a，'b 以及 'c`。
    pub fn declare_life_time() {
        fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
            return &foo.x;
        }

        let mut foo = Foo { x: 23 };
        let x = &mut foo.x;
        *x = 12;
        // x 在这里被 dropped 释放从而允许我们再创建一个不可变引用
        let y = do_something(&foo);
        println!("y = {}", y);
        // y 在这里被 dropped 释放
        // foo 在这里被 dropped 释放
    }

    #[test]
    fn test_declare_life_time() {
        declare_life_time();
    }

    /// # 多个生命周期
    ///
    /// 生命周期注解可以通过区分函数签名中不同部分的生命周期，来允许我们显式地明确某些编译器靠自己无法解决的场景。
    pub fn multi_life_time_example() {
        fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
            println!("foo_a = {}", foo_a.x);
            println!("foo_b = {}", foo_b.x);

            return &foo_b.x;
        }

        let foo_a = Foo { x: 323 };
        let foo_b = Foo { x: 23 };
        let x = do_something(&foo_a, &foo_b);
        // foo_a 在这里被 dropped 释放因为只有 foo_b 的生命周期在此之后还在延续
        println!("x = {}", x);
        // x 在这里被 dropped 释放
        // foo_b 在这里被 dropped 释放
    }

    #[test]
    fn test_multi_life_time_example() {
        multi_life_time_example();
    }

    /// # 静态生命周期
    ///
    /// 一个静态变量是一个在编译期间即被创建并存在于整个程序始末的内存资源。他们必须被明确指定类型。
    /// 一个静态生命周期是指一段内存资源无限期地延续到程序结束。需要注意的一点是，在此定义之下，
    /// 一些静态生命周期的资源也可以在运行时被创建。
    /// 拥有静态生命周期的资源会拥有一个特殊的生命周期注解 `'static`。
    /// `'static` 资源永远也不会被 `drop` 释放。 如果静态生命周期资源包含了引用，
    /// 那么这些引用的生命周期也一定是 `'static` 的。
    /// （任何缺少了此注解的引用都不会达到同样长的存活时间）
    ///
    /// 内存细节：
    ///
    /// - 因为静态变量可以全局性地被任何人访问读取而潜在地引入数据争用，
    /// 所以修改它具有内在的危险性。我们会在稍后讨论使用全局数据的一些挑战。
    /// - Rust 允许使用 `unsafe { ... }` 代码块来进行一些无法被编译器担保的内存操作。
    /// The R̸͉̟͈͔̄͛̾̇͜U̶͓͖͋̅Ṡ̴͉͇̃̉̀T̵̻̻͔̟͉́͆Ơ̷̥̟̳̓͝N̶̨̼̹̲͛Ö̵̝͉̖̏̾̔M̶̡̠̺̠̐͜Î̷̛͓̣̃̐̏C̸̥̤̭̏͛̎͜O̶̧͚͖͔̊͗̇͠N̸͇̰̏̏̽̃（常见的中文翻译为：Rust 死灵书）在讨论时应该被严肃地看待，
    pub fn static_life_time_example() {
        static PI: f64 = 3.1415;

        // 静态变量的范围也可以被限制在一个函数内
        static mut SECRET: &'static str = "swordfish";
        unsafe {
            println!("SECRET: {}", SECRET);
        }

        // 字符串字面值拥有 'static 生命周期
        let msg: &'static str = "Hello World!";
        let p: &'static f64 = &PI;
        println!("msg = {}", msg);
        println!("PI = {}", p);

        // 你可以打破一些规则，但是必须是显式地
        unsafe {
            // 我们可以修改 SECRET 到一个字符串字面值因为其同样是 'static 的
            SECRET = "abracadabra";
            println!("SECRET = {}", SECRET);
        }
    }

    #[test]
    fn test_static_life_time_example() {
        static_life_time_example();
    }

    /// 数据类型中的生命周期
    /// 和函数相同，数据类型也可以用生命周期注解来参数化其成员。
    /// Rust 会验证引用所包含的数据结构永远也不会比引用指向的所有者存活周期更长。
    /// 我们不能在运行中拥有一个包括指向虚无的引用结构存在！
    pub fn data_struct_life_time() {
        struct Foo<'a> {
            i: &'a i32,
        }

        let x = 23;
        let foo = Foo { i: &x };

        println!("{}", foo.i);
    }

    #[test]
    fn test_data_struct_life_time() {
        data_struct_life_time();
    }

    /// # 第五章 - 总结
    /// 哇，恭喜您成功走完了本章！我知道这下可能会有很多需要吸收的东西，但是您已经在成为一名 Rustacean 的路上走得很好了。希望您能愈发清晰地认识到 Rust 是如何致力于解决系统编程中的诸多常见挑战：
    ///
    /// 无意间对资源的修改
    /// 忘记及时地释放资源
    /// 资源意外地被释放两次
    /// 在资源被释放后使用了它
    /// 由于读取数据的同时有其他人正在向资源中写入数据而引起的数据争用
    /// 在编译器无法做担保时，清晰看到代码的作用域
    /// 在下一章中，我们会研究一些 Rust 如何处理文本的相关知识。
    pub fn chapter5_summary() {
        println!("\
第五章 - 总结
哇，恭喜您成功走完了本章！我知道这下可能会有很多需要吸收的东西，但是您已经在成为一名 Rustacean 的路上走得很好了。希望您能愈发清晰地认识到 Rust 是如何致力于解决系统编程中的诸多常见挑战：

无意间对资源的修改
忘记及时地释放资源
资源意外地被释放两次
在资源被释放后使用了它
由于读取数据的同时有其他人正在向资源中写入数据而引起的数据争用
在编译器无法做担保时，清晰看到代码的作用域
在下一章中，我们会研究一些 Rust 如何处理文本的相关知识。
        ");
    }
}
