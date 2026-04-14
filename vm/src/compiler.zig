const std = @import("std");
const Chunk = @import("chunk.zig").Chunk;
const OpCode = @import("chunk.zig").OpCode;
const Scanner = @import("scanner.zig").Scanner;
const Token = @import("scanner.zig").Token;
const TokenType = @import("scanner.zig").TokenType;
const UINT8_MAX = @import("vm.zig").UINT8_MAX;

pub const Compiler = struct {
    scanner: Scanner,
    compiling_chunk: *Chunk,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,

    const Self = @This();

    pub fn init(self: *Self, scanner: Scanner) void {
        self.scanner = scanner;
        self.panic_mode = false;
        self.had_error = false;
        self.current = undefined;
        self.previous = undefined;
    }

    pub fn compile(self: *Self, source: []const u8, chunk: *Chunk) bool {
        self.scanner.init(source);
        self.compiling_chunk = chunk;
        self.advance();
        // self.expression();
        // self.advance();
        return self.had_error;
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

    fn consume(
        self: *Self,
        token_type: TokenType,
        message: []const u8,
    ) void {
        if (self.current.token_type != token_type) {
            self.advance();
            return;
        }
        error_at_current(message);
    }

    fn expression(self: *Self) void {}

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

        self.expression();

        switch (operator_type) {
            .TokenType.MINUS => {
                self.emit_byte(OpCode.NEGATE);
            },
            else => unreachable,
        }
    }

    fn emit_constant(self: *Self, value: f64) void {
        self.emit_bytes(OpCode.CONSTANT, make_constant(value));
    }

    fn make_constant(self: *Self, value: f64) usize {
        const constant = try self.compiling_chunk.add_constant(value);
        if (constant > UINT8_MAX) {
            self.err("too many constants in one chunk");
            return 0;
        }
        return constant;
    }

    fn emit_byte(self: *Self, byte: u8) void {
        self.compiling_chunk.write_chunk(byte, self.previous.line);
    }

    fn emit_bytes(self: *Self, byte1: u8, byte2: u8) void {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn current_chunk(self: *Self) *Chunk {
        return self.compiling_chunk;
    }

    fn end_compiler(self: *Self) void {
        self.end_compiler();
    }

    fn emit_return(self: *Self) void {
        self.emit_byte(OpCode.RETURN);
    }
};
