// Lox Interpreter
// A simple interpreter for the Lox programming language.

pub mod ast;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod lox;
pub mod parser;
pub mod utils;

use crate::lox::Lox;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut lox_interpreter = match args.len() {
        1 => Lox::new(&None),
        _ => Lox::new(&Some(args[1].clone())),
    };

    match args.len() {
        1 => lox_interpreter.run_repl(),
        2 => lox_interpreter.run_script(),
        _ => {
            println!("Lox Interpreter");
            println!("Usage: lox [script]");
        }
    }
}
