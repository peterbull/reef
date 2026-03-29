const std = @import("std");
const vm = @import("vm");
const c = @cImport({
    @cInclude("chunk.h");
    @cInclude("common.h");
    @cInclude("debug.h");
    @cInclude("value.h");
});

const chunk_mod = @import("chunk.zig");
const Chunk = chunk_mod.Chunk;
const OpCode = chunk_mod.OpCode;

const debug_mod = @import("debug.zig");
pub fn main() !void {
    try vm.bufferedPrint();
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var chunk = Chunk.init();
    defer chunk.freeChunk(allocator);
    _ = try chunk.addConstant(allocator, 1.4);
    _ = try chunk.addConstant(allocator, 1.5);
    // const constant2 = try chunk.addConstant(allocator, 1.6);
    try chunk.writeChunk(allocator, OpCode.OP_CONSTANT, 123);
    try chunk.writeChunk(allocator, OpCode.OP_CONSTANT, 124);
    try chunk.writeChunk(allocator, OpCode.OP_RETURN, 127);
    _ = debug_mod.simpleInstruction("peter", 3);
    debug_mod.disassembleChunk(&chunk, "test_chunk");
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
