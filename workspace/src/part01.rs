// Rust-101, Part 01: Expressions, Inherent methods
// ================================================

// For Rust to compile this file, make sure to enable the corresponding line
// in `main.rs` before going on.

// ## Expression-based programming
fn sqr(i: i32) -> i32 {
    i * i
}

// Conditionals are also just expressions. This is comparable to the ternary `? :` operator
// from languages like C.
fn abs(i: i32) -> i32 {
    if i >= 0 {
        i
    } else {
        -i
    }
}

enum NumberOrNothing {
    Number(i32),
    Nothing,
}
use self::NumberOrNothing::{Nothing, Number};
fn number_or_default(n: NumberOrNothing, default: i32) -> i32 {
    match n {
        Nothing => default,
        Number(n) => n,
    }
}

// It is even the case that blocks are expressions, evaluating to the last expression they contain.
fn compute_stuff(x: i32) -> i32 {
    let y = {
        let z = x * x;
        z + 14
    };
    y * y
}

// Let us now refactor `vec_min`.
fn vec_min(v: Vec<i32>) -> NumberOrNothing {
    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }

    let mut min = Nothing;
    for e in v {
        match min {
            NumberOrNothing::Nothing => {
                min = NumberOrNothing::Number(e);
            }
            NumberOrNothing::Number(value) => {
                min = NumberOrNothing::Number(min_i32(e, value));
            }
        }
    }
    min
}

// Now that's already much shorter! Make sure you can go over the code above and actually understand
// every step of what's going on.

// ## Inherent implementations
impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        };
    }
}

// With our refactored functions and methods, `main` now looks as follows:
fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 2, 9, 27]
}
pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec.clone());
    min.print();
    let sum = vec_sum(vec.clone());
    println!("Vec Sum: {}", sum);
    vec_print(vec.clone());
}
// You will have to replace `part00` by `part01` in the `main` function in
// `main.rs` to run this code.

// **Exercise 01.1**: Write a function `vec_sum` that computes the sum of all values of a `Vec<i32>`.

fn vec_sum(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, x| acc + x)
}

// **Exercise 01.2**: Write a function `vec_print` that takes a vector and prints all its elements.
fn vec_print(nums: Vec<i32>) {
    let result = nums
        .iter()
        .fold(String::new(), |acc, &x| format!("{acc}, {x}"));
    println!("{}", result);
}
