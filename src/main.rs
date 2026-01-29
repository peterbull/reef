#![allow(unused_variables, dead_code)]
use std::env;

use reef_interpreter::reef::Reef;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    let mut reef = Reef::new();
    match command.as_str() {
        "tokenize" => {
            let filename = &args[2];
            reef.run_file(filename);
        }
        "repl" => {
            let _ = reef.run_repl();
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
