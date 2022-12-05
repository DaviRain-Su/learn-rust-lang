fn main() {
    let mut letters = vec!["a", "b", "c"];

    // for letter in letters {
    //     println!("{}", letter);
    //     letters.push(letter.clone());
    // }
}

// 就会出现编译失败的情况，因为Rust不允许在该迭代块中修改letters

// error[E0382]: borrow of moved value: `letters`
//    --> rust-in-action/examples/iterator_error.rs:6:9
//     |
// 2   |     let mut letters = vec!["a", "b", "c"];
//     |         ----------- move occurs because `letters` has type `Vec<&str>`, which does not implement the `Copy` trait
// 3   |
// 4   |     for letter in letters {
//     |                   ------- `letters` moved due to this implicit call to `.into_iter()`
// 5   |         println!("{}", letter);
// 6   |         letters.push(letter.clone());
//     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
//     |
// note: this function takes ownership of the receiver `self`, which moves `letters`
//    --> /Users/davirain/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:262:18
//     |
// 262 |     fn into_iter(self) -> Self::IntoIter;
//     |                  ^^^^
// help: consider iterating over a slice of the `Vec<&str>`'s content to avoid moving into the `for` loop
//     |
// 4   |     for letter in &letters {
//     |                   +

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `rust-in-action` due to previous error