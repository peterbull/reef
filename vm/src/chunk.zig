const std = @import("std");
pub const OpCode = enum { OP_CONSTANT, OP_RETURN };
pub const ValueArray = struct {
    count: i32,
    capacity: i32,
    values: std.ArrayList(f64),
    pub fn init() ValueArray {
        return ValueArray{ .count = 0, .capacity = 0, .values = &[_]f64{} };
    }
};

pub const Chunk = struct {
    count: i32,
    capacity: i32,
    code: std.ArrayList(u8),
    lines: std.ArrayList(u32),
    constants: std.ArrayList(ValueArray),
    const Self = @This();
    pub fn init() Self {
        return Self{
            .count = 0,
            .capacity = 0,
            .code = .{},
            .lines = .{},
            .constants = .{},
        };
    }
    pub fn writeChunk(self: *Self, allocator: std.mem.Allocator, op: OpCode, line: u32) !void {
        try self.code.append(allocator, @intFromEnum(op));
        try self.lines.append(allocator, line);
    }
    pub fn freeChunk(self: *Self, allocator: std.mem.Allocator) void {
        self.code.deinit(allocator);
        self.code.deinit(allocator);
    }
};
