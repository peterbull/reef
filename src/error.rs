use crate::{Token, TokenType};

pub enum LoxError {
    ParseError(String),
}
pub fn lox_error(line: usize, message: &str) -> LoxError {
    lox_report(line, "", message)
}
pub fn lox_error_at_line(token: Token, message: &str) -> LoxError {
    let where_info = if token.token_type == TokenType::Eof {
        "at end"
    } else {
        &format!("at '{}'", token.lexeme)
    };
    lox_report(token.line, where_info, message)
}

fn lox_report(line: usize, where_info: &str, message: &str) -> LoxError {
    eprintln!("[line {}] Error {}: {}", line, where_info, message);
    LoxError::ParseError(format!("[line {}] Error {}: {}", line, where_info, message))
}
