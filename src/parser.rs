#![allow(unused_variables, dead_code)]

use crate::{Literal, Token, TokenType, expr::Expr};

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

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
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
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }
    fn unary(&mut self) -> Expr {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }
    fn primary(&mut self) -> Expr {
        if self.match_type(&[TokenType::False]) {
            return Expr::Literal {
                value: Literal::Boolean(false),
            };
        }
        if self.match_type(&[TokenType::True]) {
            return Expr::Literal {
                value: Literal::Boolean(true),
            };
        }
        if self.match_type(&[TokenType::Nil]) {
            return Expr::Literal {
                value: Literal::Nil,
            };
        }
        if self.match_type(&[TokenType::Number, TokenType::String]) {
            let token = self.previous().expect("should be tokens here").clone();
            let literal_value = token.literal.expect("should be literal here");

            return Expr::Literal {
                value: literal_value,
            };
        }
        if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression();
            if self.match_type(&[TokenType::RightParen]) {
                return Expr::Grouping {
                    expression: Box::new(expr),
                };
            } else {
                panic!("Expected ')' after opening paren")
            };
        }
        panic!("Expected primary expression")
    }
}
