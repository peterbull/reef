#![allow(unused_variables, dead_code)]
use std::env;
use std::fs;

use lox_interpreter::lox::Lox;

fn scanner_error(line: usize, msg: String) {
    println!("ERROR: Line {}, {}", line, msg);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                Lox::run(&file_contents);
                // panic!("Scanner not implemented");
            } else {
                println!("EOF  null");
            }
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
