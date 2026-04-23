const std = @import("std");
const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Scanner = @import("scanner.zig").Scanner;
const Token = @import("scanner.zig").Token;
const TokenType = @import("scanner.zig").TokenType;
const UINT8_MAX = @import("vm.zig").UINT8_MAX;
const Config = @import("config.zig").Config;
const disassemble_chunk = @import("debug.zig").disassemble_chunk;
pub const Precedence = enum {
    NONE,
    ASSIGNMENT, // =
    OR, // or
    AND, // and
    EQUALITY, // == !=
    COMPARISON, // < > <= >=
    TERM, // + -
    FACTOR, // * /
    UNARY, // ! -
    CALL, // . ()
    PRIMARY,
};
pub const ParseFn = fn (compiler: *Compiler) void;
pub const ParseRule = struct {
    prefix: ?ParseFn,
    infix: ?ParseFn,
    precedence: Precedence,
};
pub const rules = std.EnumArray(TokenType, ParseRule).init({
    TokenType.LEFT_PAREN = .{ .prefix = Compiler.grouping, .infix = null, .precedence = Precedence.NONE };
    TokenType.RIGHT_PAREN = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.LEFT_BRACE = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.RIGHT_BRACE = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.COMMA = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.DOT = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.MINUS = .{ .prefix = Compiler.unary, .infix = Compiler.binary, .precedence = Precedence.TERM };
    TokenType.PLUS = .{ .prefix = null, .infix = Compiler.binary, .precedence = Precedence.TERM };
    TokenType.SEMICOLON = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.SLASH = .{ .prefix = null, .infix = Compiler.binary, .precedence = Precedence.FACTOR };
    TokenType.STAR = .{ .prefix = null, .infix = Compiler.binary, .precedence = Precedence.FACTOR };
    TokenType.BANG = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.BANG_EQUAL = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.EQUAL = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.EQUAL_EQUAL = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.GREATER = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.GREATER_EQUAL = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.LESS = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.LESS_EQUAL = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.IDENTIFIER = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.STRING = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.NUMBER = .{ .prefix = Compiler.number, .infix = null, .precedence = Precedence.NONE };
    TokenType.AND = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.CLASS = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.ELSE = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.FALSE = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.FOR = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.FUN = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.IF = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.NIL = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.OR = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.PRINT = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.RETURN = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.SUPER = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.THIS = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.TRUE = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.VAR = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.WHILE = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.ERROR = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
    TokenType.EOF = .{ .prefix = null, .infix = null, .precedence = Precedence.NONE };
});
fn get_rule(operator_type: TokenType) *ParseRule {
    return &rules[operator_type];
}
pub const Compiler = struct {
    scanner: Scanner,
    compiling_chunk: *Chunk,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    config: *Config,

    const Self = @This();

    pub fn init(self: *Self, scanner: Scanner, config: *Config) void {
        self.scanner = scanner;
        self.panic_mode = false;
        self.had_error = false;
        self.current = undefined;
        self.previous = undefined;
        self.config = config;
    }

    pub fn compile(self: *Self, source: []const u8, chunk: *Chunk) bool {
        self.scanner.init(source);
        self.compiling_chunk = chunk;
        self.advance();
        self.expression();
        self.consume(.EOF, "Expect end of expression.");
        self.end_compiler();
        return !self.had_error;
    }
    fn expression(self: *Self) void {
        self.parse_precedence(.ASSIGNMENT);
    }

    fn advance(self: *Self) void {
        self.previous = self.current;
        while (true) {
            self.current = self.scanner.scan_token();
            if (self.current.token_type != TokenType.ERROR) {
                break;
            }
        }
    }

    fn error_at_current(self: *Self, message: []const u8) void {
        return self.error_at(self.current, message);
    }

    fn err(self: *Self, message: []const u8) void {
        return self.error_at(self.previous, message);
    }

    fn error_at(self: *Self, token: Token, message: []const u8) void {
        if (self.panic_mode) {
            return;
        }
        self.panic_mode = true;

        std.debug.print("[line {d}]", .{token.line});

        switch (token.token_type) {
            TokenType.EOF => std.debug.print("at end", .{}),
            TokenType.ERROR => {},
            else => std.debug.print("at {s}", .{token.start[0..token.length]}),
        }
        std.debug.print(": {s}\n", .{message});
        self.had_error = true;
    }

    fn consume(self: *Self, token_type: TokenType, message: []const u8) void {
        if (self.current.token_type == token_type) {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn parse_precedence(self: *Self, precedence: Precedence) void {
        self.advance();

        const prefix_rule = get_rule(self.previous.token_type).prefix orelse {
            self.err("expected expression");
            return;
        };

        prefix_rule(self);

        while (@intFromEnum(precedence) <= @intFromEnum(get_rule(self.current.token_type).precedence)) {
            self.advance();
            if (get_rule(self.previous.token_type).infix) |infix_rule| {
                infix_rule(self);
            }
        }
    }

    fn number(self: *Self) void {
        const token = self.previous.start[0..self.previous.length];
        const value = std.fmt.parseFloat(f64, token) catch unreachable;
        self.emit_constant(value);
    }
    fn grouping(self: *Self) void {
        self.expression();
        self.consume(TokenType.RIGHT_PAREN, "Expect ')' after expression");
    }
    fn unary(self: *Self) void {
        const operator_type = self.previous.token_type;

        self.parse_precedence(Precedence.UNARY);

        switch (operator_type) {
            TokenType.MINUS => {
                self.emit_byte(@intFromEnum(OpCode.NEGATE));
            },
            else => unreachable,
        }
    }
    fn binary(self: *Self) void {
        const operator_type = self.previous.token_type;
        const rule = get_rule(operator_type);
        const next_precedence: Precedence = @enumFromInt(@intFromEnum(rule.precedence) + 1);
        self.parse_precedence(next_precedence);
        switch (operator_type) {
            TokenType.PLUS => self.emit_byte(OpCode.ADD),
            TokenType.MINUS => self.emit_byte(OpCode.SUBTRACT),
            TokenType.STAR => self.emit_byte(OpCode.MULTIPLY),
            TokenType.SLASH => self.emit_byte(OpCode.DIVIDE),
            else => unreachable,
        }
    }

    fn emit_constant(self: *Self, value: f64) void {
        self.emit_bytes(OpCode.CONSTANT, make_constant(value));
    }

    fn make_constant(self: *Self, value: f64) u8 {
        const constant = self.compiling_chunk.add_constant(value) catch {
            self.err("too many constants in one chunk");
            return 0;
        };
        if (constant > UINT8_MAX) {
            self.err("too many constants in one chunk");
            return 0;
        }
        return @intCast(constant);
    }

    fn emit_byte(self: *Self, op: OpCode) void {
        self.compiling_chunk.write_chunk(@intFromEnum(op), self.previous.line);
    }

    fn emit_bytes(self: *Self, byte1: u8, byte2: u8) void {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn current_chunk(self: *Self) *Chunk {
        return self.compiling_chunk;
    }

    fn end_compiler(self: *Self) void {
        self.emit_return();
        if (self.config.debug_print_code) {
            disassemble_chunk(self.compiling_chunk, "code");
        }
    }

    fn emit_return(self: *Self) void {
        self.emit_byte(OpCode.RETURN);
    }
};
