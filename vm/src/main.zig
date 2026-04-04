const std = @import("std");
const c = @cImport({
    @cInclude("chunk.h");
    @cInclude("common.h");
    @cInclude("debug.h");
    @cInclude("value.h");
});

const chunk_mod = @import("chunk.zig");
const vm_mod = @import("vm.zig");
const config_mod = @import("config.zig");
const Chunk = chunk_mod.Chunk;
const OpCode = chunk_mod.OpCode;
const Config = config_mod.Config;

const debug_mod = @import("debug.zig");
var vm = &vm_mod.vm;
const VM = vm_mod.VM;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const config = try Config.parse(args);

    var vm_instance = VM.init(config);
    vm = &vm_instance;

    var chunk = Chunk.init(allocator);
    defer chunk.freeChunk();

    const constant1 = try chunk.addConstant(1.4);
    const constant2 = try chunk.addConstant(1.6);

    try chunk.writeChunk(OpCode.OP_CONSTANT, 123);
    try chunk.writeByte(@intCast(constant1), 123);
    try chunk.writeChunk(OpCode.OP_CONSTANT, 124);
    try chunk.writeByte(@intCast(constant2), 124);

    try chunk.writeChunk(OpCode.OP_RETURN, 127);

    _ = vm.interpret(&chunk);
    vm.deinit();
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
