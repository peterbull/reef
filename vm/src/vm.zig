const std = @import("std");

pub const InterpretResult = enum { INTERPRET_OK, INTERPRET_COMPILE_ERROR, INTERPRET_RUNTIME_ERROR };

const chunk_mod = @import("chunk.zig");
const Chunk = chunk_mod.Chunk;
const OpCode = chunk_mod.OpCode;

pub const VM = struct {
    chunk: *Chunk,
    ip: std.ArrayList(u8),

    const Self = @This();

    pub fn init() void {}
    pub fn deinit() void {}
    pub fn interpret(self: *Self, chunk: *Chunk) InterpretResult {
        self.chunk = chunk;
        self.ip = self.chunk.code;

        @panic("TODO: not yet implemented");
    }
    fn readByte(self: *Self) u8 {
        const byte = self.chunk.code[self.ip];
        self.ip += 1;
        return byte;
    }
    fn run(self: *Self) void {
        while (true) {
            const instruction: OpCode = @enumFromInt(self.readByte());
            switch (instruction) {
                .OP_CONSTANT => {},
                .OP_RETURN => {},
            }
        }
    }
};

var vm: VM = undefined;
