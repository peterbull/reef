use std::collections::HashMap;

use crate::{
    error::lox_error,
    token::{Literal, Token, TokenType},
};

fn scanner_error(line: usize, msg: String) {
    println!("ERROR: Line {}, {}", line, msg);
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
    start: usize,
    current: usize,
    keywords: HashMap<&'static str, TokenType>,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Scanner {
            source,
            tokens: Vec::new(),
            line: 1,
            start: 0,
            current: 0,
            keywords,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, None, self.line));
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, Some(literal), self.line));
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn handle_token(&mut self, c: &char) {
        match c {
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    lox_error(self.line, "unexpected character");
                }
            }
        }
    }

    fn is_digit(&self, c: &char) -> bool {
        c.is_ascii_digit()
        //  alt
        // ('0'..='9').contains(&c)
    }

    fn is_alpha(&self, c: &char) -> bool {
        c.is_alphabetic() || *c == '_'
        //  alt
        // ('0'..='9').contains(&c)
    }
    fn is_alphanumeric(&self, c: &char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }
    fn identifier(&mut self) {
        while self.is_alphanumeric(&self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = self
            .keywords
            .get(text)
            .copied()
            .unwrap_or(TokenType::Identifier);
        self.add_token(token_type);
    }
    fn number(&mut self) {
        while self.is_digit(&self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(&self.peek_next()) {
            self.advance();
            while self.is_digit(&self.peek()) {
                self.advance();
            }
        }
        let str_num = self.source[self.start..self.current].to_string();
        let num_literal = Literal::Number(str_num.parse::<f64>().unwrap());
        self.add_token_with_literal(TokenType::Number, num_literal);
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        let str_val = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String, Literal::String(str_val));
        // closing "
        self.advance();
    }

    fn match_next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = self.source.chars().nth(self.current).unwrap();
        if c != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.is_next_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();
            self.handle_token(&c);
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        self.tokens.clone()
    }

    pub fn print_info(&self) {
        println!("printing tokens:");
        for tok in &self.tokens {
            println!("{:?}", tok);
        }
        println!("end lines: {}", self.line);
        println!("finish value for current: {}", self.current);
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn is_next_end(&self) -> bool {
        self.current + 1 >= self.source.len()
    }
}
