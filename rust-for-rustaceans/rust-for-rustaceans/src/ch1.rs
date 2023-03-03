#[test]
fn test_stack_and_heap_value() {
    let x = 42; // 42是值
    let y = 43; // 43是值
                // While var1 and var2 store the same value initially, they store separate,
                // independent copies of that value; when we change the value stored in
                // var2 1, the value in var1 does not change.
    let var1 = &x; // var1 is pointers
    let mut var2 = &x; // var2 is pointers
    var2 = &y;
    println!("x = {x}, y = {y}");
    println!("var1 = {var1}, var2 = {var2}");
}

#[test]
fn test_flow() {
    let mut x;
    // this access would be illegal, nowhere to draw the flow from:
    // 这条通道将是非法的，没有地方可以引流：
    // 这里是非法的，这里的x在assert_eq中用的时候x还没有初始化，所以他是不存在的。
    // assert_eq!(x, 42); // error
    x = 42; // ^1
            // this is okay, can draw a flow from the value assigned above:
            // 这是可以的，可以从上面分配的值中画出一个流程。
    let y = &x; // ^2
                // this establishes a scond, mutable flow from x:
                // 这就建立了一个来自x的第二个可变的流。
                // x = 43; // ^3 error 根据借用规则 这里也是不允许的
                // this continus the flow from y, wich in turn draws from x.
                // but that flow conflicts with the assignment to x!
                // 这就继续了来自y的流量，而y又从x中提取。但这一流动与分配给X的任务相冲突!
    assert_eq!(*y, 42); // ^4
}

#[test]
fn test_ownership() {
    let x1 = 42;
    let y1 = Box::new(84);
    {
        // startsa new scope
        let z = (x1, y1); // 1
                          // z goes out of scope, and is dropes:
                          // it in turn drops the values from x1 and y1
    } // 2
      // x1's value is Copy, so it was not moved into z
    let x2 = x1; // 3
                 // y1's value is not Copy, so it was moved into z
                 // let y2 = y1;
}

#[test]
fn test_catche() {
    fn cache(input: &i32, sum: &mut i32) {
        *sum = *input + *input;
        assert_eq!(*sum, 2 * *input);
    }

    cache(&2, &mut 4);
}

#[test]
fn test_nolias() {
    fn nolias(input: &i32, output: &mut i32) {
        if *input == 1 {
            *output = 2;
        }
        if *input != 1 {
            *output = 3;
        }
    }

    fn nolias_2(input: &i32, output: &mut i32) {
        if *input == 1 {
            *output = 2;
        } else {
            *output = 3;
        }
    }

    let mut result = 0;
    nolias(&1, &mut result);

    println!("result: {:?}", result);

    nolias_2(&2, &mut result);
    println!("result: {:?}", result);
}

#[test]
fn test_mut_reference() {
    let x = 42;
    let mut y = &x; // y is of type &i32
    let z = &mut y; // z is of type &mut &i32
}

#[test]
fn test_() {
    fn replace_with_84(s: &mut Box<i32>) {
        // this is no okay, as *s would be empty:
        // let was = *s; // 1
        // but this is:
        let was = std::mem::take(s); // 2
                                     // so is this:
        *s = was; // 3
                  // we can exchange values behind &mut:
        let mut r = Box::new(84);
        std::mem::swap(s, &mut r); // 4
        assert_ne!(*r, 84);
        // below two row, I added.
        assert_eq!(*s, Box::new(84));
        assert_eq!(*r, 42);
    }
    let mut s = Box::new(42);
    replace_with_84(&mut s);
    assert_eq!(*s, 84);
    //   5:
}

#[test]
fn test_lifetime() {
    let mut x = Box::new(42);
    let r = &x; // 'a
                // if rand() > 0.5 {
                //     *x = 84;
                // } else {
                //     println!("{}", r); //'a
                // }
}

#[test]
// todo need to understand
fn test_life_time1() {
    let mut x = Box::new(42);
    let mut z = &x; // 'a
    for i in 0..100 {
        println!("{}", z); //'a
        x = Box::new(i);
        z = &x; //'a
    }
    println!("{}", z); // 'a
}
