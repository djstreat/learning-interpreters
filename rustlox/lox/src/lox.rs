use crate::interpreter::Interpreter;
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use std::fs;
use std::io::{Write, stdin, stdout};

pub fn parse_bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

pub fn parse_bytes_to_float(bytes: &[u8]) -> f64 {
    parse_bytes_to_string(bytes).parse().unwrap()
}

// Lox Interpreter
pub struct Lox {
    main_script: Option<String>,
    has_error: bool,
    lexer: Lexer,
    interpreter: Interpreter,
}

impl Lox {
    pub fn new(script_path: &Option<String>) -> Self {
        // Read the script file if provided
        let contents = match script_path {
            Some(path) => fs::read_to_string(path).expect("Could not read file"),
            None => String::new(),
        };

        let lexer = Lexer::new(Some(contents));

        Lox {
            main_script: script_path.clone(),
            has_error: false,
            lexer,
            interpreter: Interpreter::new(),
        }
    }

    // Implementation for running the Lox interpreter with a script
    pub fn run_script(&mut self) {
        let contents =
            fs::read_to_string(self.main_script.as_ref().unwrap()).expect("Could not read file");
        self.run(contents.as_str());
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
        println!("[line {line}] Error{where_}: {message}");
    }

    fn error(&mut self, token: Token, line: usize, message: &str) {
        self.has_error = true;
        match token.token_type() {
            TokenType::Eof => self.report(line, " at end", message),
            _ => self.report(line, &format!(" at '{}'", token.lexeme()), message),
        }
    }

    fn print_prompt(&self) {
        print!("> ");
        stdout().flush().unwrap();
    }

    // Implementation for running the Lox interpreter
    fn run(&mut self, input: &str) {
        self.lexer = Lexer::new(Some(input.to_string()));
        let scanned_tokens = self.lexer.scan_tokens();
        if scanned_tokens.is_err() {
            self.report(0, "", &format!("{:?}", scanned_tokens.err()));
            return;
        }
        let mut parser = Parser::new(Some(scanned_tokens.unwrap()));
        let statements = parser.parse();
        if let Err(err) = statements {
            self.report(0, "", &format!("{err:?}"));
            return;
        }
        self.interpreter
            .interpret(statements.unwrap().as_mut_slice());
    }
}
