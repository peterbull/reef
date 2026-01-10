use std::io::{self, Write};

use crate::error::LoxError;
use crate::interpreter::{self, Interpreter};
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Lox {}
impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn run(&self, text: &str) {
        let mut scanner = Scanner::new(text.to_string());
        let tokens = scanner.scan_tokens();
        scanner.print_info();
        let mut parser = Parser::new(tokens);

        let expressions = parser.parse();
        println!("expression: {:#?}", expressions);
    }
    pub fn run_repl(&self) -> io::Result<()> {
        println!("Starting REPL...");
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text)?;
            if input_text.trim() == "exit" {
                break;
            }
            self.run(&input_text);
        }
        Ok(())
    }
}
