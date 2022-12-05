#[derive(Debug)]
enum Cereal {
    Barley, 
    Millet, 
    Rice,
    Rye,
    Spelt,
    Wheat
}

// 尝试去编译会触发一个错误信息，信息的大意是“试图去‘借用’一个已经‘被移动’了的值”。
fn main() {
    let mut grains = vec![];

    grains.push(Cereal::Barley);

    // drop(grains);

    println!("{:?}", grains);
}

// --> rust-in-action/examples/error1.rs:18:22
// |
// 12 |     let mut grains = vec![];
// |         ---------- move occurs because `grains` has type `Vec<Cereal>`, which does not implement the `Copy` trait
// ...
// 16 |     drop(grains);
// |          ------ value moved here
// 17 |
// 18 |     println!("{:?}", grains);
// |                      ^^^^^^ value borrowed here after move
// |
// = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
