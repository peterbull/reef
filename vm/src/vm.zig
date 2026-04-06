const std = @import("std");
const disassemble_instruction = @import("debug.zig").disassemble_instruction;
const print_value = @import("debug.zig").print_value;

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
    stack_top: usize = 0,
    config: Config,

    const Self = @This();

    pub fn init(config: Config) VM {
        return Self{ .chunk = null, .ip = 0, .config = config };
    }

    pub fn deinit(self: *Self) void {
        _ = self;
    }

    pub fn interpret(self: *Self, source: []const u8) InterpretResult {
        _ = source;
        // self.chunk = chunk;
        self.ip = 0;

        return self.run();
    }

    fn reset_stack(self: *Self) void {
        self.stack_top = 0;
    }

    fn push(self: *Self, value: f64) void {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(self: *Self) f64 {
        self.stack_top -= 1;
        return self.stack[self.stack_top];
    }
    // increments ip
    fn read_byte(self: *Self) u8 {
        const byte = self.chunk.?.code.items[self.ip];
        self.ip += 1;
        return byte;
    }

    fn read_constant(self: *Self) f64 {
        const constant = self.chunk.?.constants.items[self.read_byte()];
        return constant;
    }

    fn binary_op(self: *Self, op: BinaryOp) void {
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
            if (self.config.debug_trace) {
                std.debug.print("             ", .{});
                for (self.stack[0..self.stack_top]) |slot| {
                    std.debug.print("[ ", .{});
                    print_value(slot);
                    std.debug.print(" ]", .{});
                }
                std.debug.print("\n", .{});
                _ = disassemble_instruction(self.chunk.?, self.ip);
            }
            const instruction: OpCode = @enumFromInt(self.read_byte());
            switch (instruction) {
                .OP_CONSTANT => {
                    const constant = self.read_constant();
                    self.push(constant);
                },
                .OP_NEGATE => {
                    self.push(-self.pop());
                },
                .OP_ADD => {
                    self.binary_op(BinaryOp.ADD);
                },
                .OP_SUBTRACT => {
                    self.binary_op(BinaryOp.SUBTRACT);
                },
                .OP_MULTIPLY => {
                    self.binary_op(BinaryOp.MULTIPLY);
                },
                .OP_DIVIDE => {
                    self.binary_op(BinaryOp.DIVIDE);
                },
                .OP_RETURN => {
                    print_value(self.pop());
                    std.debug.print("\n", .{});
                    return InterpretResult.INTERPRET_OK;
                },
            }
        }
    }
};

pub var vm: VM = undefined;
