const std = @import("std");
const disassembleInstruction = @import("debug.zig").disassembleInstruction;
const printValue = @import("debug.zig").printValue;

pub const InterpretResult = enum { INTERPRET_OK, INTERPRET_COMPILE_ERROR, INTERPRET_RUNTIME_ERROR };

const chunk_mod = @import("chunk.zig");
const Config = @import("config.zig").Config;

const Chunk = chunk_mod.Chunk;
const OpCode = chunk_mod.OpCode;
const BinaryOp = enum { ADD, SUBTRACT, MULTIPLY, DIVIDE };

const STACK_MAX = 256;

pub const VM = struct {
    chunk: ?*Chunk,
    ip: usize,
    stack: [STACK_MAX]f64 = [_]f64{0} ** 256,
    stackTop: usize = 0,
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

    fn resetStack(self: *Self) void {
        self.stackTop = 0;
    }

    fn push(self: *Self, value: f64) void {
        self.stack[self.stackTop] = value;
        self.stackTop += 1;
    }

    fn pop(self: *Self) f64 {
        self.stackTop -= 1;
        return self.stack[self.stackTop];
    }
    // increments ip
    fn readByte(self: *Self) u8 {
        const byte = self.chunk.?.code.items[self.ip];
        self.ip += 1;
        return byte;
    }

    fn readConstant(self: *Self) f64 {
        const constant = self.chunk.?.constants.items[self.readByte()];
        return constant;
    }

    fn binaryOp(self: *Self, op: BinaryOp) void {
        const b = self.pop();
        const a = self.pop();
        const result = switch (op) {
            .ADD => a + b,
            .SUBTRACT => a - b,
            .MULTIPLY => a * b,
            .DIVIDE => a / b,
        };
        self.push(result);
    }

    fn run(self: *Self) InterpretResult {
        while (true) {
            if (self.config.debugTrace) {
                std.debug.print("             ", .{});
                for (self.stack[0..self.stackTop]) |slot| {
                    std.debug.print("[ ", .{});
                    printValue(slot);
                    std.debug.print(" ]", .{});
                }
                std.debug.print("\n", .{});
                _ = disassembleInstruction(self.chunk.?, self.ip);
            }
            const instruction: OpCode = @enumFromInt(self.readByte());
            switch (instruction) {
                .OP_CONSTANT => {
                    const constant = self.readConstant();
                    self.push(constant);
                },
                .OP_NEGATE => {
                    self.push(-self.pop());
                },
                .OP_ADD => {
                    self.binaryOp(BinaryOp.ADD);
                },
                .OP_SUBTRACT => {
                    self.binaryOp(BinaryOp.SUBTRACT);
                },
                .OP_MULTIPLY => {
                    self.binaryOp(BinaryOp.MULTIPLY);
                },
                .OP_DIVIDE => {
                    self.binaryOp(BinaryOp.DIVIDE);
                },
                .OP_RETURN => {
                    printValue(self.pop());
                    std.debug.print("\n", .{});
                    return InterpretResult.INTERPRET_OK;
                },
            }
        }
    }
};

pub var vm: VM = undefined;
