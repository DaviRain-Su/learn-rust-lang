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
        assert_eq!(*sum, 2* *input);
    }

    cache(&2, &mut 4);
}