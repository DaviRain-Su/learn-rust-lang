#![feature(default_free_fn)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#[macro_use]
extern crate lazy_static;
extern crate core;

use std::io;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() -> anyhow::Result<()> {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    let _ = repl::start(io::stdin(), io::stdout())?;

    Ok(())
}
