#![allow(unused_variables, dead_code)]
use crate::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            Some(&self.tokens[self.current - 1])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn is_at_end(&self) -> bool {
        self.current > self.tokens.len()
    }
    fn is_at_eof(&self) -> bool {
        self.peek()
            .map(|token| token.token_type == TokenType::Eof)
            .unwrap_or(true)
    }

    fn match_type(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self
                .peek()
                .expect("peek shouldn't be hitting the end here")
                .token_type
                == token_type
        }
    }

    fn expression(&mut self) {
        self.equality();
    }

    fn equality(&mut self) {}
    fn comparison(&mut self) {}
}
