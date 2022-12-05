
pub fn add(i: i32, j: i32) -> i32 {
    i + j
}

#[test] 
fn test_add() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e);
}

#[test]
fn test_data_operators() {
    let twenty = 20;
    let twenty_one = 21;
    let twenty_two = 22i32;
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_two = [
        42.0, 42f32, 42.0_f32,
    ];
    println!("{:0?}", forty_two[0]);
}

#[test]
fn test_display_binary_octal_hexadecimal() {
    let three = 0b11;

    let thirty = 0o36;

    let three_hundred = 0x12c;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

#[test]
fn test_compare_number() {
    let a: i32 = 10;
    let b: u16 = 100;
    if a < b.into() { // b as i32 is also ok 
        println!("The is less than on hundred.");
    }
}

#[test]
fn test_float_type() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("  0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!(" 0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!(" 0.3: {:x}", (xyz.2).to_bits());
    println!();

    let f64_absolete_diff = (xyz.2 - (xyz.0 + xyz.1)).abs();
    assert!(f64_absolete_diff <= f64::EPSILON);


    assert_eq!(abc.0 + abc.1, abc.2);
    // assert_eq!(xyz.0 + xyz.1, xyz.2);
//     thread 'chapter2::test_float_type' panicked at 'assertion failed: `(left == right)`
//   left: `0.30000000000000004`,
//  right: `0.3`', rust-in-action/src/chapter2.rs:70:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// test chapter2::test_float_type ... FAILED

    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;

    let absolute_difference = (desired - result).abs();

    assert!(absolute_difference <= f32::EPSILON);

    // let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);
//     thread 'chapter2::test_float_type' panicked at 'assertion failed: `(left == right)`
//   left: `NaN`,
//  right: `NaN`', rust-in-action/src/chapter2.rs:89:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// test chapter2::test_float_type ... FAILED

    let x: f32 = 1.0 / 0.0;
    // x us finite
    assert!(!x.is_finite());

}


#[test]
fn test_complex_number() {
    use num::complex::Complex;

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.22);

    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}

#[test]
fn test_if_expression() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    let n = 123456;
    // let description = if is_even(n) {
    //     "even"
    // } else {
    //     "odd"
    // };   
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };

    // another if expression write ways

    println!("{} is {}", n, description);
}

#[test]
fn test_match() {
    // let needle = 42;

    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}

#[test]
fn test_find_number() {
    let needle = 0o204;

    let haystack = [1, 1, 2, 5, 52, 132, 877, 4140, 21147];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}