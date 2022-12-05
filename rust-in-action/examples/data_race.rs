use std::thread;
use std::sync::{Arc, Mutex};

// 由于线程的调度是由操作系统决定的，而不是由应用程序决定的，因此根本无法知道先定义的那个线程会不会率先执行。
fn main() {
    let data = 100;
    
    // thread::spawn(|| { data = 500; });

    // thread::spawn(|| { data = 1000; });

    println!("data = {}", data);
}
// fn main() {
//     let data = Arc::new(Mutex::new(100));
//     let data1 = data.clone();
//     let data2 = data.clone();

//     let r = thread::spawn(move || { *data1.lock().unwrap() = 5000; });

//     let s = thread::spawn(move || { *data2.lock().unwrap() = 1000;  });

//     r.join();
//     s.join();

//     println!("data = {:?}", data);
// }

// error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
//  --> rust-in-action/examples/data_race.rs:6:19
//   |
// 6 |     thread::spawn(|| { data = 500; });
//   |                   ^^   ---- `data` is borrowed here
//   |                   |
//   |                   may outlive borrowed value `data`
//   |
// note: function requires argument type to outlive `'static`
//  --> rust-in-action/examples/data_race.rs:6:5
//   |
// 6 |     thread::spawn(|| { data = 500; });
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
//   |
// 6 |     thread::spawn(move || { data = 500; });
//   |                   ++++

// error[E0499]: cannot borrow `data` as mutable more than once at a time
//  --> rust-in-action/examples/data_race.rs:8:19
//   |
// 6 |     thread::spawn(|| { data = 500; });
//   |     ---------------------------------
//   |     |             |    |
//   |     |             |    first borrow occurs due to use of `data` in closure
//   |     |             first mutable borrow occurs here
//   |     argument requires that `data` is borrowed for `'static`
// 7 |
// 8 |     thread::spawn(|| { data = 1000; });
//   |                   ^^   ---- second borrow occurs due to use of `data` in closure
//   |                   |
//   |                   second mutable borrow occurs here

// error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
//  --> rust-in-action/examples/data_race.rs:8:19
//   |
// 8 |     thread::spawn(|| { data = 1000; });
//   |                   ^^   ---- `data` is borrowed here
//   |                   |
//   |                   may outlive borrowed value `data`
//   |
// note: function requires argument type to outlive `'static`
//  --> rust-in-action/examples/data_race.rs:8:5
//   |
// 8 |     thread::spawn(|| { data = 1000; });
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
//   |
// 8 |     thread::spawn(move || { data = 1000; });
//   |                   ++++

// error[E0502]: cannot borrow `data` as immutable because it is also borrowed as mutable
//   --> rust-in-action/examples/data_race.rs:10:27
//    |
// 6  |     thread::spawn(|| { data = 500; });
//    |     ---------------------------------
//    |     |             |    |
//    |     |             |    first borrow occurs due to use of `data` in closure
//    |     |             mutable borrow occurs here
//    |     argument requires that `data` is borrowed for `'static`
// ...
// 10 |     println!("data = {}", data);
//    |                           ^^^^ immutable borrow occurs here
//    |
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

// Some errors have detailed explanations: E0373, E0499, E0502.
// For more information about an error, try `rustc --explain E0373`.
// error: could not compile `rust-in-action` due to 4 previous errors