const std = @import("std");
const vm = @import("vm");
const c = @cImport({
    @cInclude("chunk.h");
    @cInclude("common.h");
    @cInclude("debug.h");
    @cInclude("value.h");
});
const OpCode = enum { OP_RETURN };

const Chunk = struct {
    count: i32,
    capacity: i32,
    code: ?[*]u8,
    pub fn init() Chunk {
        return Chunk{ .count = 0, .capacity = 0, .code = null };
    }
};
fn test_fn(a: i32, b: i32) i32 {
    return a + b;
}
pub fn main() !void {
    // Prints to stderr, ignoring potential errors.
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});
    try vm.bufferedPrint();
    const chunk = Chunk.init();
    std.debug.print("chunk {any}\n", .{chunk});
    defer std.debug.print("hey", .{});
    var cChunk: c.Chunk = undefined;
    c.init_chunk(&cChunk);
    c.write_chunk(&cChunk, c.OP_RETURN, 123);
    const constant: u8 = @intCast(c.add_constant(&cChunk, 1.2));
    c.write_chunk(&cChunk, c.OP_CONSTANT, 123);
    c.write_chunk(&cChunk, constant, 123);
    c.write_chunk(&cChunk, c.OP_RETURN, 127);
    c.disassemble_chunk(&cChunk, "test_chunk");
    c.free_chunk(&cChunk);
    std.debug.print("my c chunk: {}", .{cChunk});
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
