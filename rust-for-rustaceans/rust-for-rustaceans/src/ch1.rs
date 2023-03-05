#[test]
fn test_stack_and_heap_value() {
    let x = 42; // 42是value
    let y = 43; // 43是value
    println!("x = {x}, y = {y}");
    // 这里var1, var2 保存的是x的reference
    // var2也是保存的x的reference，并且var2也是可以改变的。
    let var1 = &x; 
    let mut var2 = &x;
    // 这里将var2的值改变成了y的reference
    var2 = &y;
    println!("var1 = {var1}, var2 = {var2}");
}

#[test]
fn test_flow() {
    let mut x;
    // this access would be illegal, nowhere to draw the flow from:
    // assert_eq!(x, 42); // error
    x = 42; // ^1
    // this is okay, can draw a flow from the value assigned above:
    let y = &x; // ^2
    // this establishes a scond, mutable flow from x:
    x = 43; // ^3 error 根据借用规则 这里也是不允许的
    // this continus the flow from y, wich in turn draws from x.
    // but that flow conflicts with the assignment to x!
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
