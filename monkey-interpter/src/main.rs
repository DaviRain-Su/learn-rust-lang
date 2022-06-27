#![feature(default_free_fn)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#[macro_use]
extern crate lazy_static;
extern crate core;

use std::io;

pub mod lexer;
pub mod repl;
pub mod token;
pub mod ast;
pub mod parser;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout());
}
