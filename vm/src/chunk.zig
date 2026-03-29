const std = @import("std");

pub const OpCode = enum { OP_CONSTANT, OP_RETURN };

pub const Chunk = struct {
    code: std.ArrayList(u8),
    lines: std.ArrayList(u32),
    constants: std.ArrayList(f64),
    const Self = @This();
    pub fn init() Self {
        return Self{
            .code = .{},
            .lines = .{},
            .constants = .{},
        };
    }
    pub fn writeChunk(self: *Self, allocator: std.mem.Allocator, op: OpCode, line: u32) !void {
        const enumInt = @intFromEnum(op);
        try self.code.append(allocator, enumInt);
        try self.lines.append(allocator, line);
    }
    pub fn freeChunk(self: *Self, allocator: std.mem.Allocator) void {
        self.code.deinit(allocator);
        self.lines.deinit(allocator);
        self.constants.deinit(allocator);
    }
    pub fn addConstant(self: *Self, allocator: std.mem.Allocator, value: f64) !usize {
        try self.constants.append(allocator, value);
        return self.constants.items.len - 1;
    }
};
