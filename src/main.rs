#![allow(unused_variables, dead_code)]
use std::env;
use std::fs;

use lox_interpreter::lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let mut lox = Lox::new();
    match command.as_str() {
        "tokenize" => {
            let filename = &args[2];
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                lox.run(&file_contents);
                // panic!("Scanner not implemented");
            } else {
                println!("EOF  null");
            }
        }
        "repl" => {
            let _ = lox.run_repl();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_output() {
        assert_eq!(1, 1)
    }
}
