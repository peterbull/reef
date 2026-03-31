const std = @import("std");

pub const OpCode = enum(u8) { OP_CONSTANT, OP_RETURN };

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

    pub fn writeChunk(self: *Self, op: OpCode, line: u32) !void {
        const enumInt = @intFromEnum(op);
        try self.code.append(self.allocator, enumInt);
        try self.lines.append(self.allocator, line);
    }

    pub fn writeByte(self: *Self, byte: u8, line: u32) !void {
        try self.code.append(self.allocator, byte);
        try self.lines.append(self.allocator, line);
    }

    pub fn freeChunk(
        self: *Self,
    ) void {
        self.code.deinit(self.allocator);
        self.lines.deinit(self.allocator);
        self.constants.deinit(self.allocator);
    }
    pub fn addConstant(self: *Self, value: f64) !usize {
        try self.constants.append(self.allocator, value);
        return self.constants.items.len - 1;
    }
};
