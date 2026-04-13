const std = @import("std");

pub const TokenType = enum {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    ERROR,
    EOF,
};

pub const Token = struct {
    token_type: TokenType,
    start: []const u8,
    length: usize,
    line: usize,
};

pub const Scanner = struct {
    start: []const u8,
    current: []const u8,
    line: usize,

    const Self = @This();

    pub fn init(self: *Self, source: []const u8) void {
        self.start = source;
        self.current = source;
        self.line = 1;
    }

    pub fn deinit(self: *Self) void {
        _ = self;
    }

    pub fn scan_token(self: *Self) Token {
        self.start = self.current;
        if (self.is_at_end()) return self.make_token(TokenType.EOF);
        const c = self.advance();
        if (is_alpha(c)) {
            return self.identifier();
        }
        if (is_digit(c)) {
            return self.number();
        }
        switch (c) {
            '(' => return self.make_token(TokenType.LEFT_PAREN),
            ')' => return self.make_token(TokenType.RIGHT_PAREN),
            '{' => return self.make_token(TokenType.LEFT_BRACE),
            '}' => return self.make_token(TokenType.RIGHT_BRACE),
            ';' => return self.make_token(TokenType.SEMICOLON),
            ',' => return self.make_token(TokenType.COMMA),
            '.' => return self.make_token(TokenType.DOT),
            '-' => return self.make_token(TokenType.MINUS),
            '+' => return self.make_token(TokenType.PLUS),
            '/' => return self.make_token(TokenType.SLASH),
            '*' => return self.make_token(TokenType.STAR),
            '!' => {
                const token = if (self.match('=')) TokenType.BANG_EQUAL else TokenType.BANG;
                return self.make_token(token);
            },
            '=' => {
                const token = if (self.match('=')) TokenType.EQUAL_EQUAL else TokenType.EQUAL;
                return self.make_token(token);
            },
            '<' => {
                const token = if (self.match('=')) TokenType.LESS_EQUAL else TokenType.LESS;
                return self.make_token(token);
            },
            '>' => {
                const token = if (self.match('=')) TokenType.GREATER_EQUAL else TokenType.GREATER;
                return self.make_token(token);
            },
            else => return self.error_token("bad token"),
        }
    }

    fn number(self: *Self) Token {
        while (is_digit(self.peek())) {
            _ = self.advance();
        }
        if (self.peek() == '.' and is_digit(self.peek_next())) {
            _ = self.advance();
            while (is_digit(self.peek())) {
                _ = self.advance();
            }
        }
        return self.make_token(TokenType.NUMBER);
    }

    fn string(self: *Self) Token {
        while (self.peek() != '"' and !self.is_at_end()) {
            if (self.peek() == '\n') {
                self.line += 1;
            }
            self.advance();
        }
        if (self.is_at_end()) return self.error_token("unterminated string");
        self.advance();
        return self.make_token(TokenType.STRING);
    }

    fn identifier(self: *Self) Token {
        while (is_alpha(self.peek()) or is_digit(self.peek())) {
            _ = self.advance();
        }
        return self.make_token(self.identifier_type());
    }

    fn identifier_type(self: *Self) TokenType {
        const token_len = self.start.len - self.current.len;
        switch (self.start[0]) {
            'a' => return self.check_keyword(1, 2, "nd", TokenType.AND),
            'c' => return self.check_keyword(1, 4, "lass", TokenType.CLASS),
            'e' => return self.check_keyword(1, 3, "lse", TokenType.ELSE),
            'f' => {
                if (token_len > 1) {
                    switch (self.start[1]) {
                        'a' => return self.check_keyword(2, 3, "lse", TokenType.FALSE),
                        'o' => return self.check_keyword(2, 1, "r", TokenType.FOR),
                        'u' => return self.check_keyword(2, 1, "n", TokenType.FUN),
                        else => return TokenType.ERROR,
                    }
                }
            },
            'i' => return self.check_keyword(1, 1, "f", TokenType.IF),
            'n' => return self.check_keyword(1, 2, "il", TokenType.NIL),
            'o' => return self.check_keyword(1, 1, "r", TokenType.OR),
            'p' => return self.check_keyword(1, 4, "rint", TokenType.PRINT),
            'r' => return self.check_keyword(1, 5, "eturn", TokenType.RETURN),
            's' => return self.check_keyword(1, 4, "uper", TokenType.SUPER),
            't' => {
                if (token_len > 1) {
                    switch (self.start[1]) {
                        'h' => return self.check_keyword(2, 2, "is", TokenType.THIS),
                        'r' => return self.check_keyword(2, 2, "ue", TokenType.TRUE),
                        else => return TokenType.ERROR,
                    }
                }
            },
            'v' => return self.check_keyword(1, 2, "ar", TokenType.VAR),
            'w' => return self.check_keyword(1, 4, "hile", TokenType.WHILE),
            else => return TokenType.ERROR,
        }
        return TokenType.IDENTIFIER;
    }
    fn check_keyword(self: *Self, start: usize, length: usize, rest: []const u8, token_type: TokenType) TokenType {
        const token_len = self.start.len - self.current.len;
        if (token_len == start + length and
            std.mem.eql(u8, self.start[start .. start + length], rest))
        {
            return token_type;
        }
        return TokenType.IDENTIFIER;
    }
    fn is_digit(c: u8) bool {
        return c >= '0' and c <= '9';
    }

    fn is_alpha(c: u8) bool {
        return (c >= 'a' and c <= 'z') or (c >= 'A' and c <= 'Z') or c == '_';
    }

    fn skip_whitespace(self: *Self) void {
        while (true) {
            const c = self.peek();
            switch (c) {
                ' ' => self.advance(),
                '\r' => self.advance(),
                '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '/' => {
                    if (self.peek_next() == '/') {
                        while (self.peek() != '\n' and !self.is_at_end()) {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                },
                '"' => self.string(),
                else => return,
            }
        }
    }

    fn peek(self: *Self) u8 {
        return self.current[0];
    }

    fn peek_next(self: *Self) u8 {
        if (self.current.len < 2) return 0;
        return self.current[1];
    }

    fn is_at_end(self: *Self) bool {
        return self.current.len == 0;
    }

    fn match(self: *Self, expected: u8) bool {
        if (self.is_at_end()) return false;
        if (self.current[0] != expected) return false;
        self.current = self.current[1..];
        return true;
    }

    fn make_token(self: *Self, token_type: TokenType) Token {
        return Token{
            .token_type = token_type,
            .start = self.start,
            .length = self.start.len - self.current.len,
            .line = self.line,
        };
    }

    fn error_token(self: *Self, message: []const u8) Token {
        return Token{
            .token_type = TokenType.ERROR,
            .start = message,
            .length = message.len,
            .line = self.line,
        };
    }

    fn advance(self: *Self) u8 {
        const c = self.current[0];
        self.current = self.current[1..];
        return c;
    }
};
