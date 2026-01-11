use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum LoxError {
    ParseError(String),
    RuntimeError(String),
}
impl LoxError {
    pub fn lox_error(line: usize, message: &str) -> LoxError {
        LoxError::lox_report(line, "", message)
    }
    pub fn lox_general_error(message: &str) -> LoxError {
        eprintln!("Error: {}", message);
        LoxError::ParseError(format!("Error: {}", message))
    }
    pub fn lox_error_at_line(token: &Token, message: &str) -> LoxError {
        let where_info = if token.token_type == TokenType::Eof {
            "at end"
        } else {
            &format!("at '{}'", token.lexeme)
        };
        LoxError::lox_report(token.line, where_info, message)
    }

    pub fn lox_report(line: usize, where_info: &str, message: &str) -> LoxError {
        eprintln!("[line {}] Error {}: {}", line, where_info, message);
        LoxError::ParseError(format!("[line {}] Error {}: {}", line, where_info, message))
    }

    pub fn lox_runtime_error(token: &Token, message: &str) -> LoxError {
        eprintln!("Error {:?}: {}", token, message);
        LoxError::RuntimeError(format!("Error {:?}: {}", token, message))
    }
}

// TODO: track runtime errors in main Lox struct
