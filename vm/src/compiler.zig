const Chunk = @import("chunk.zig").Chunk;
const Scanner = @import("scanner.zig").Scanner;
const Token = @import("scanner.zig").Token;
const TokenType = @import("scanner.zig").TokenType;

pub const Compiler = struct {
    parser: Scanner,
    current: Token,
    previous: Token,

    const Self = @This();

    pub fn init(self: *Self, scanner: Scanner) void {
        self.parser = scanner;
    }

    pub fn compile(source: []const u8, chunk: *Chunk) bool {
        _ = source;
        _ = chunk;
        // init_scanner(source);
        unreachable;
    }
    pub fn advance(self: *Self) void {
        self.previous = self.current;
        while (true) {
            self.current = self.parser.scan_token();
            if (self.current.type != TokenType.ERROR) {
                break;
            }
        }
    }
    pub fn error_at_current(message: []const u8) void {
        _ = message;
    }
    pub fn err(message: []const u8) void {
        _ = message;
    }
    pub fn error_at(token: TokenType, message: []const u8) void {}
};
