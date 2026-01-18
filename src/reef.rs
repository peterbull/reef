use crate::ast_printer::AstPrinter;
use crate::error::ReefError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::fs;
use std::io::{self, Write};

pub struct Reef {
    had_error: bool,
    had_runtime_error: bool,
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

impl Reef {
    pub fn new() -> Self {
        Reef {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run(&mut self, text: &str) -> Result<(), ReefError> {
        let mut scanner = Scanner::new(text.to_string());

        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);

        let interpreter = Interpreter::new();

        scanner.print_info();

        let stmts = parser.parse()?;
        println!("statements: {:#?}", stmts);
        interpreter.interpret(stmts)?;
        Ok(())
    }
    pub fn run_file(&mut self, filename: &str) {
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {}", filename);
            String::new()
        });
        if !file_contents.is_empty() {
            self.run(&file_contents);
        } else {
            println!("EOF  null");
        }
        if self.had_error {
            std::process::exit(65)
        }
        if self.had_runtime_error {
            std::process::exit(70)
        }
    }
    pub fn run_repl(&mut self) -> io::Result<()> {
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
            self.had_runtime_error = false;
            self.had_error = false;
        }
        Ok(())
    }
    fn report_error(&mut self, error: &ReefError) {
        eprintln!("{:?}", error);
        match error {
            ReefError::ParseError { .. } => self.had_error = true,
            ReefError::RuntimeError { .. } => self.had_runtime_error = true,
        }
    }
}
