use crate::{
    Value,
    token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum ReefError {
    ParseError(String),
    RuntimeError(String),
    Return(Value),
}
impl ReefError {
    pub fn reef_error(line: usize, message: &str) -> ReefError {
        ReefError::reef_report(line, "", message)
    }
    pub fn reef_general_error(message: &str) -> ReefError {
        eprintln!("Error: {}", message);
        ReefError::ParseError(format!("Error: {}", message))
    }
    pub fn reef_error_at_line(token: &Token, message: &str) -> ReefError {
        let where_info = if token.token_type == TokenType::Eof {
            "at end"
        } else {
            &format!("at '{}'", token.lexeme)
        };
        ReefError::reef_report(token.line, where_info, message)
    }

    pub fn reef_report(line: usize, where_info: &str, message: &str) -> ReefError {
        eprintln!("[line {}] Error {}: {}", line, where_info, message);
        ReefError::ParseError(format!("[line {}] Error {}: {}", line, where_info, message))
    }

    pub fn reef_runtime_error(token: &Token, message: &str) -> ReefError {
        eprintln!("Error {:?}: {}", token, message);
        ReefError::RuntimeError(format!("Error {:?}: {}", token, message))
    }
    pub fn reef_return(value: Value) -> ReefError {
        ReefError::Return(value)
    }
}

// TODO: track runtime errors in main Reef struct
