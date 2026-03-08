#![allow(unused_variables, dead_code)]

use crate::{
    Literal, Token, TokenType,
    error::ReefError,
    expr::{Expr, ExprKind},
    stmt::StmtKind,
};
use std::rc::Rc;

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

    fn previous(&mut self) -> Option<&Token> {
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
            match self.declaration() {
                Ok(stmt) => self.statements.push(stmt),
                Err(e) => return Err(e),
            }
        }
        Ok(self.statements.clone())
    }

    fn declaration(&mut self) -> Result<StmtKind, ReefError> {
        let decl_result = {
            if self.match_type(&[TokenType::Fun]) {
                return self.function("function");
            }
            if self.match_type(&[TokenType::Var]) {
                return self.var_declaration();
            }
            if self.match_type(&[TokenType::Class]) {
                return self.class_declaration();
            }
            self.statement()
        };
        match &decl_result {
            Ok(_) => {}
            Err(e) => self.synchronize(),
        }
        decl_result
    }

    fn class_declaration(&mut self) -> Result<StmtKind, ReefError> {
        let name = self
            .consume(TokenType::Identifier, "expect class name")?
            .clone();
        self.consume(TokenType::LeftBrace, "expect '{' before class body")?;
        let mut methods: Vec<StmtKind> = Vec::new();
        if !&self.check(&TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after class body")?;
        Ok(StmtKind::Class { name, methods })
    }

    fn var_declaration(&mut self) -> Result<StmtKind, ReefError> {
        let name = self
            .consume(TokenType::Identifier, "expect variable name")?
            .clone();
        let mut initializer = Rc::new(ExprKind::None);
        if self.match_type(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }
        self.consume(TokenType::Semicolon, "expected ';' after var declaration")?;
        Ok(StmtKind::Var { name, initializer })
    }

    fn function(&mut self, kind: &str) -> Result<StmtKind, ReefError> {
        let name = &self
            .consume(
                TokenType::Identifier,
                &format!("expect '(' after {} name", { kind }),
            )?
            .clone();
        self.consume(TokenType::LeftParen, "expect '(' before function params")?;
        let mut parameters: Vec<Token> = Vec::new();
        if !&self.check(&TokenType::RightParen) && !self.is_at_end() {
            loop {
                if parameters.len() >= 255 {
                    return Err(ReefError::reef_error_at_line(
                        self.peek().unwrap(),
                        "can't have more than 255 params",
                    ));
                }
                let identifier = self
                    .consume(TokenType::Identifier, "expect parameter name")?
                    .clone();
                parameters.push(identifier.clone());
                if !self.match_type(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let prev_token = self.consume(TokenType::RightParen, "Expect ')' after params")?;
        let prev_token = prev_token.clone();

        let brace_check = self.check(&TokenType::LeftBrace);
        if !brace_check {
            Err(ReefError::reef_error_at_line(
                &prev_token.clone(),
                &format!("expect '{{' before {} body", { kind }),
            ))
        } else {
            let body = self.block_statements()?;
            Ok(StmtKind::Function {
                name: name.clone(),
                parameters,
                body,
            })
        }
    }

    fn statement(&mut self) -> Result<StmtKind, ReefError> {
        let peek_result = self.peek();
        match peek_result {
            Some(token) => match token.token_type {
                TokenType::For => self.for_statement(),
                TokenType::If => self.if_statement(),
                TokenType::Print => self.print_statement(),
                TokenType::While => self.while_statement(),
                TokenType::LeftBrace => {
                    let statements = self.block_statements()?;
                    Ok(StmtKind::Block { statements })
                }
                TokenType::Return => self.return_statement(),
                _ => self.expression_statement(),
            },
            None => Err(ReefError::reef_general_error("Error parsing expression")),
        }
    }

    fn for_statement(&mut self) -> Result<StmtKind, ReefError> {
        self.advance();
        self.consume(TokenType::LeftParen, "expect '(' to begin for loop")?;
        let initializer;
        if self.match_type(&[TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_type(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }
        let mut condition = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }

        self.consume(TokenType::Semicolon, "expect ';' after loop condition")?;

        let mut increment: Option<Expr> = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }

        self.consume(TokenType::RightParen, "expect ')' after for clauses")?;

        let mut body = self.statement()?;
        if let Some(inc) = increment {
            body = StmtKind::Block {
                statements: vec![body, StmtKind::Expression { expr: inc }],
            }
        }
        if condition.is_none() {
            condition = Some(Rc::new(ExprKind::Literal {
                value: Literal::Boolean(true),
            }))
        }
        body = StmtKind::While {
            condition: condition.expect("should always be a condition here"),
            body: Box::new(body),
        };
        if let Some(init) = initializer {
            body = StmtKind::Block {
                statements: vec![init, body],
            }
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<StmtKind, ReefError> {
        self.advance();
        self.consume(TokenType::LeftParen, "expect '(' to begin while expression")?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "expect ')' to close while expression",
        )?;
        let body = self.statement()?;
        Ok(StmtKind::While {
            condition,
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<StmtKind, ReefError> {
        self.advance();
        self.consume(
            TokenType::LeftParen,
            "expect '(' to begin if while expression",
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "expect ')' to close if while expression",
        )?;
        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;
        if self.match_type(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }
        Ok(StmtKind::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn block_statements(&mut self) -> Result<Vec<StmtKind>, ReefError> {
        self.advance();
        let mut statements: Vec<StmtKind> = Vec::new();
        while !self.match_type(&[TokenType::RightBrace]) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(decl)
        }
        Ok(statements)
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

    fn return_statement(&mut self) -> Result<StmtKind, ReefError> {
        self.advance();
        let keyword = self
            .previous()
            .expect("should have a preceding token")
            .clone();
        let mut value = Rc::new(ExprKind::None);
        if !self.check(&TokenType::Semicolon) {
            value = self.expression()?;
        }
        self.consume(
            TokenType::Semicolon,
            "expected semicolon after Return expression",
        )?;
        Ok(StmtKind::Return { keyword, value })
    }

    fn or_expression(&mut self) -> Result<Expr, ReefError> {
        let mut expr = self.and_expression()?;
        while self.match_type(&[TokenType::Or]) {
            let operator = self
                .previous()
                .expect("should have a preceding token")
                .clone();
            let right = self.and_expression()?;
            expr = Rc::new(ExprKind::Logical {
                left: expr,
                operator,
                right,
            })
        }
        Ok(expr)
    }

    fn and_expression(&mut self) -> Result<Expr, ReefError> {
        let mut expr = self.equality()?;
        while self.match_type(&[TokenType::And]) {
            let operator = self
                .previous()
                .expect("should have a preceding token")
                .clone();
            let right = self.equality()?;
            expr = Rc::new(ExprKind::Logical {
                left: expr,
                operator,
                right,
            })
        }
        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, ReefError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ReefError> {
        let expr = self.or_expression()?;
        if self.match_type(&[TokenType::Equal]) {
            let equals = self
                .previous()
                .expect("should have a preceding token")
                .clone();
            let value = self.assignment()?;

            match expr.as_ref() {
                ExprKind::Variable { name } => {
                    return Ok(Rc::new(ExprKind::Assign {
                        name: name.clone(),
                        value,
                    }));
                }
                ExprKind::Get { object, name } => {
                    return Ok(Rc::new(ExprKind::Set {
                        object: Rc::clone(object),
                        name: name.clone(),
                        value,
                    }));
                }
                _ => {
                    return Err(ReefError::reef_general_error(&format!(
                        "invalid assignment target: {:?}",
                        equals
                    )));
                }
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ReefError> {
        let mut expr = self.comparison()?;
        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().expect("").clone();
            let right = self.comparison()?;
            expr = Rc::new(ExprKind::Binary {
                left: expr,
                operator,
                right,
            })
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ReefError> {
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
            expr = Rc::new(ExprKind::Binary {
                left: expr,
                operator,
                right,
            })
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ReefError> {
        let mut expr = self.factor()?;
        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.factor()?;
            expr = Rc::new(ExprKind::Binary {
                left: expr,
                operator,
                right,
            })
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ReefError> {
        let mut expr = self.unary()?;
        while self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.factor()?;
            expr = Rc::new(ExprKind::Binary {
                left: expr,
                operator,
                right,
            })
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ReefError> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ReefError::reef_error_at_line(
                        self.peek().expect("should be a preceding token"),
                        "can't have more than 255 arguments",
                    ));
                }
                let expr = self.expression()?;
                arguments.push(expr);
                if !self.match_type(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments")?;

        Ok(Rc::new(ExprKind::Call {
            callee,
            token: paren.clone(),
            arguments,
        }))
    }

    fn call(&mut self) -> Result<Expr, ReefError> {
        let mut expr = self.primary()?;
        loop {
            if self.match_type(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_type(&[TokenType::Dot]) {
                let name = self.consume(TokenType::Identifier, "Expect property name after '.'")?;
                expr = Rc::new(ExprKind::Get {
                    object: expr,
                    name: name.clone(),
                });
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ReefError> {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self
                .previous()
                .expect("token should exist after match")
                .clone();
            let right = self.unary()?;
            return Ok(Rc::new(ExprKind::Unary { operator, right }));
        }
        self.call()
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

    fn primary(&mut self) -> Result<Expr, ReefError> {
        if self.match_type(&[TokenType::False]) {
            return Ok(Rc::new(ExprKind::Literal {
                value: Literal::Boolean(false),
            }));
        }
        if self.match_type(&[TokenType::True]) {
            return Ok(Rc::new(ExprKind::Literal {
                value: Literal::Boolean(true),
            }));
        }
        if self.match_type(&[TokenType::Nil]) {
            return Ok(Rc::new(ExprKind::Literal {
                value: Literal::Nil,
            }));
        }
        if self.match_type(&[TokenType::Number, TokenType::String]) {
            let token = self.previous().expect("should be tokens here").clone();
            let literal_value = token.literal.expect("should be literal here");
            return Ok(Rc::new(ExprKind::Literal {
                value: literal_value,
            }));
        }

        if self.match_type(&[TokenType::Identifier]) {
            let name = self.previous().expect("should be tokens here").clone();
            return Ok(Rc::new(ExprKind::Variable { name }));
        }

        if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "there should be a ')' following a '('",
            )?;
            return Ok(Rc::new(ExprKind::Grouping { expression: expr }));
        }
        if self.match_type(&[TokenType::This]) {
            let keyword = self.previous().expect("should be tokens here too").clone();
            return Ok(Rc::new(ExprKind::This { keyword }));
        }

        Err(ReefError::reef_error_at_line(
            &self.tokens[self.current],
            "expected primary expression",
        ))
    }
}
