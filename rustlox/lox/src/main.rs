// Lox Interpreter
// A simple interpreter for the Lox programming language.
use std::env;

pub mod lox;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut lox_interpreter = match args.len() {
        1 => lox::Lox::new(None),
        _ => lox::Lox::new(Some(args[1].clone())),
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
