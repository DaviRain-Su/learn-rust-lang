#![allow(dead_code)]
#![allow(unused_assignments)]
#[macro_use]
extern crate lazy_static;
use std::io;

pub mod lexer;
pub mod token;
pub mod repl;

fn main() {
    println!("Hello {}! This is the Monkey programming language!", whoami::username());
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout());
}
