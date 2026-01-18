#![allow(unused_variables, dead_code)]

use crate::{
    Literal, Token, TokenType,
    error::ReefError,
    expr::ExprKind,
    stmt::{Stmt, StmtKind},
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
    statements: Vec<StmtKind>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            statements: Vec::new(),
        }
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
            self.peek()
                .is_none_or(|token| &token.token_type == token_type)
        }
    }

    pub fn parse(&mut self) -> Result<Vec<StmtKind>, ReefError> {
        while !self.is_at_eof() {
            match self.statement() {
                Ok(stmt) => self.statements.push(stmt),
                Err(e) => return Err(e),
            }
        }
        Ok(self.statements.clone())
    }

    fn expression(&mut self) -> Result<ExprKind, ReefError> {
        self.equality()
    }

    fn expression_statement(&mut self) -> Result<StmtKind, ReefError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "expected semicolon after expression")?;
        Ok(StmtKind::Expression { expr })
    }
    fn print_statement(&mut self) -> Result<StmtKind, ReefError> {
        self.advance();
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "expected semicolon after expression")?;
        Ok(StmtKind::Print { expr })
    }

    fn statement(&mut self) -> Result<StmtKind, ReefError> {
        if let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Print => self.print_statement(),
                _ => self.expression_statement(),
            }
        } else {
            self.synchronize();
            match self.peek() {
                Some(token) => Err(ReefError::reef_error_at_line(
                    token,
                    "Error parsing expression",
                )),
                None => Err(ReefError::reef_general_error("Error parsing expression")),
            }
        }
    }

    fn equality(&mut self) -> Result<ExprKind, ReefError> {
        let mut expr = self.comparison()?;
        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().expect("").clone();
            let right = self.comparison()?;
            expr = ExprKind::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<ExprKind, ReefError> {
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
    fn term(&mut self) -> Result<ExprKind, ReefError> {
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

    fn factor(&mut self) -> Result<ExprKind, ReefError> {
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

    fn unary(&mut self) -> Result<ExprKind, ReefError> {
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ReefError> {
        if self.check(&token_type) {
            Ok(self.advance().expect("should be tokens in consume"))
        } else {
            Err(ReefError::reef_error_at_line(
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

    fn primary(&mut self) -> Result<ExprKind, ReefError> {
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
            return Ok(ExprKind::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(ReefError::reef_error_at_line(
            &self.tokens[self.current],
            "expected primary expression",
        ))
    }
}
