use std::fs;
use std::io::{Write, stdin, stdout};

use lexer::ast::{Expression, LiteralValue};
use lexer::lexer::Lexer;

// Lox Interpreter
pub struct Lox {
    main_script: Option<String>,
    has_error: bool,
    lexer: Lexer,
}

impl Lox {
    pub fn new(ref script_path: Option<String>) -> Self {
        // Read the script file if provided
        let contents = match script_path {
            Some(path) => fs::read_to_string(path).expect("Could not read file"),
            None => String::new(),
        };

        Lox {
            main_script: script_path.clone(),
            has_error: false,
            lexer: Lexer::new(Some(contents)),
        }
    }

    // Implementation for running the Lox interpreter with a script
    pub fn run_script(&self) {
        let contents =
            fs::read_to_string(self.main_script.as_ref().unwrap()).expect("Could not read file");
    }

    // Run interpreter in REPL mode
    pub fn run_repl(&mut self) {
        println!("[Lox Interpreter REPL]");
        loop {
            self.print_prompt();
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Could not read input");
            match input.trim() {
                "" => continue,  // Continue to the next iteration
                "exit" => break, // Exit the REPL
                _ => {
                    let _ = self.run(&input);
                    self.has_error = false;
                }
            }
        }
    }

    // Report an error with a line number and message
    pub fn report(&self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
    }

    fn error(&mut self, line: usize, message: &str) {
        self.has_error = true;
        self.report(line, "", message);
    }

    fn print_prompt(&self) {
        print!("> ");
        _ = stdout().flush();
    }

    // Implementation for running the Lox interpreter
    fn run(&self, input: &str) {
        let tokens = self.lexer.tokenize_input();
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
