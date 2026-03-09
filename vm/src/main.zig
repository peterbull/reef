const std = @import("std");
const vm = @import("vm");

const OpCode = enum { OP_RETURN };

const Chunk = struct {
    count: i32,
    capacity: i32,
    code: ?[*]u8,
    pub fn init() Chunk {
        return Chunk{ .count = 0, .capacity = 0, .code = null };
    }
};

pub fn main() !void {
    // Prints to stderr, ignoring potential errors.
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});
    try vm.bufferedPrint();
    const chunk = Chunk.init();
    std.debug.print("chunk {any}\n", .{chunk});
}

test "simple test" {
    const gpa = std.testing.allocator;
    var list: std.ArrayList(i32) = .empty;
    defer list.deinit(gpa); // Try commenting this out and see if zig detects the memory leak!
    try list.append(gpa, 42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "fuzz example" {
    const Context = struct {
        fn testOne(context: @This(), input: []const u8) anyerror!void {
            _ = context;
            // Try passing `--fuzz` to `zig build test` and see if it manages to fail this test case!
            try std.testing.expect(!std.mem.eql(u8, "canyoufindme", input));
        }
    };
    try std.testing.fuzz(Context{}, Context.testOne, .{});
}
