const std = @import("std");

pub const OpCode = enum(u8) { OP_CONSTANT, OP_ADD, OP_SUBTRACT, OP_MULTIPLY, OP_DIVIDE, OP_NEGATE, OP_RETURN };

pub const Chunk = struct {
    code: std.ArrayList(u8),
    lines: std.ArrayList(u32),
    constants: std.ArrayList(f64),
    allocator: std.mem.Allocator,

    const Self = @This();
    pub fn init(allocator: std.mem.Allocator) Self {
        return Chunk{
            .allocator = allocator,
            .code = .{},
            .lines = .{},
            .constants = .{},
        };
    }

    pub fn write_chunk(self: *Self, op: OpCode, line: u32) !void {
        const enumInt = @intFromEnum(op);
        try self.code.append(self.allocator, enumInt);
        try self.lines.append(self.allocator, line);
    }

    pub fn write_byte(self: *Self, byte: u8, line: u32) !void {
        // writes idx of constant to the code array.
        try self.code.append(self.allocator, byte);
        try self.lines.append(self.allocator, line);
    }

    pub fn free_chunk(
        self: *Self,
    ) void {
        self.code.deinit(self.allocator);
        self.lines.deinit(self.allocator);
        self.constants.deinit(self.allocator);
    }
    pub fn add_constant(self: *Self, value: f64) !usize {
        try self.constants.append(self.allocator, value);
        return self.constants.items.len - 1;
    }
};
