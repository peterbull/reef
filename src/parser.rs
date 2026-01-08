#![allow(unused_variables, dead_code)]

use crate::{
    Literal, Token, TokenType,
    error::{LoxError, lox_error_at_line},
    expr::ExprKind,
};

/*
  Extended Backus-Naur Form (ebnf)

  expression    -> equality; // passthrough
  equality      -> comparison ( ( "!=" | "==") comparison )* ; // a == b == c ...
  comparison    -> term ( (">" | ">=" | "<" | "<=") term )*;
  term          -> factor ( ("-" | "+" ) factor)* ;
  factor        -> unary ( ("/" | "*") unary )*;
  unary         -> ("!" | "-") unary | primary ;
  primary       -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")";

  ex:
  !5 + 3 * 2 >= 10 - 4 / 2 == true != false
*/

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
        self.current >= self.tokens.len()
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
    pub fn parse(&mut self) -> Vec<Result<ExprKind, LoxError>> {
        let mut results = Vec::new();
        while !self.is_at_end() {
            let expr = self.expression();
            match expr {
                Ok(expr) => {
                    results.push(Ok(expr));
                    continue;
                }
                Err(e) => {
                    self.synchronize();
                    results.push(Err(e));
                    continue;
                }
            }
        }
        results
    }
    pub fn expression(&mut self) -> Result<ExprKind, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<ExprKind, LoxError> {
        let mut expr = self.comparison()?;
        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.comparison()?;
            expr = ExprKind::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<ExprKind, LoxError> {
        let mut expr = self.term()?;
        while self.match_type(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.term()?;
            expr = ExprKind::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<ExprKind, LoxError> {
        let mut expr = self.factor()?;
        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.factor()?;
            expr = ExprKind::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<ExprKind, LoxError> {
        let mut expr = self.unary()?;
        while self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.factor()?;
            expr = ExprKind::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<ExprKind, LoxError> {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.unary()?;
            return Ok(ExprKind::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, LoxError> {
        if self.check(&token_type) {
            Ok(self.advance().expect("should be tokens in consume"))
        } else {
            Err(lox_error_at_line(
                self.peek().expect("should be token here"),
                message,
            ))
        }
    }
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self
                .previous()
                .expect("should always have a previous")
                .token_type
                == TokenType::Semicolon
            {
                return;
            }
            match self
                .peek()
                .expect("should always have a token here")
                .token_type
            {
                TokenType::Class => return,
                TokenType::For => return,
                TokenType::While => return,
                TokenType::Fun => return,
                TokenType::Print => return,
                TokenType::If => return,
                TokenType::Var => return,
                _ => {}
            }
            self.advance();
        }
    }

    fn primary(&mut self) -> Result<ExprKind, LoxError> {
        if self.match_type(&[TokenType::False]) {
            return Ok(ExprKind::Literal {
                value: Literal::Boolean(false),
            });
        }
        if self.match_type(&[TokenType::True]) {
            return Ok(ExprKind::Literal {
                value: Literal::Boolean(true),
            });
        }
        if self.match_type(&[TokenType::Nil]) {
            return Ok(ExprKind::Literal {
                value: Literal::Nil,
            });
        }
        if self.match_type(&[TokenType::Number, TokenType::String]) {
            let token = self.previous().expect("should be tokens here").clone();
            let literal_value = token.literal.expect("should be literal here");

            return Ok(ExprKind::Literal {
                value: literal_value,
            });
        }
        if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(
                TokenType::RightParen,
                "there should be a ')' following a '('",
            )?;
        };
        self.consume(
            TokenType::Semicolon,
            "expected semicolon at end of statement",
        )?;
        Err(lox_error_at_line(
            &self.tokens[self.current],
            "expected primary expression",
        ))
    }
}
