const std = @import("std");
const disassembleInstruction = @import("debug.zig").disassembleInstruction;

pub const InterpretResult = enum { INTERPRET_OK, INTERPRET_COMPILE_ERROR, INTERPRET_RUNTIME_ERROR };

const chunk_mod = @import("chunk.zig");
const Config = @import("config.zig").Config;

const Chunk = chunk_mod.Chunk;
const OpCode = chunk_mod.OpCode;

pub const VM = struct {
    chunk: ?*Chunk,
    ip: usize,
    config: Config,
    const Self = @This();

    pub fn init(config: Config) VM {
        return Self{ .chunk = null, .ip = 0, .config = config };
    }
    pub fn deinit(self: *Self) void {
        _ = self;
    }
    pub fn interpret(self: *Self, chunk: *Chunk) InterpretResult {
        self.chunk = chunk;
        self.ip = 0;

        return self.run();
    }
    fn readByte(self: *Self) u8 {
        const byte = self.chunk.?.code.items[self.ip];
        self.ip += 1;
        return byte;
    }
    fn readConstant(self: *Self) f64 {
        const constant = self.chunk.?.constants.items.ptr[self.readByte()];
        return constant;
    }
    fn run(self: *Self) InterpretResult {
        while (true) {
            if (self.config.debugTrace) {
                _ = disassembleInstruction(self.chunk.?, self.ip);
            }
            const instruction: OpCode = @enumFromInt(self.readByte());
            switch (instruction) {
                .OP_CONSTANT => {
                    const constant = self.readConstant();
                    std.debug.print("{d}\n", .{constant});
                },
                .OP_RETURN => {
                    return InterpretResult.INTERPRET_OK;
                },
            }
        }
    }
};

pub var vm: VM = undefined;
