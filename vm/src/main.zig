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
const IterpretResult = vm_mod.InterpretResult;

const debug_mod = @import("debug.zig");
var vm = &vm_mod.vm;
const VM = vm_mod.VM;

pub fn old_main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const config = try Config.parse(args);

    var vm_instance = VM.init(config);
    vm = &vm_instance;

    var chunk = Chunk.init(allocator);
    defer chunk.free_chunk();

    var constant = try chunk.add_constant(1.2);
    try chunk.write_chunk(OpCode.OP_CONSTANT, 123);
    try chunk.write_byte(@intCast(constant), 123);

    constant = try chunk.add_constant(3.4);
    try chunk.write_chunk(OpCode.OP_CONSTANT, 123);
    try chunk.write_byte(@intCast(constant), 123);

    try chunk.write_chunk(OpCode.OP_ADD, 123);
    constant = try chunk.add_constant(5.6);
    try chunk.write_chunk(OpCode.OP_CONSTANT, 123);
    try chunk.write_byte(@intCast(constant), 123);

    try chunk.write_chunk(OpCode.OP_DIVIDE, 123);
    try chunk.write_chunk(OpCode.OP_NEGATE, 123);

    try chunk.write_chunk(OpCode.OP_RETURN, 127);

    _ = vm.interpret(&chunk);
    vm.deinit();
}
fn repl() void {}

fn readFile(path: []const u8) ![]const u8 {
    var buf: [4096]u8 = undefined;
    const file = std.fs.cwd().openFile(path, .{}) catch |err| {
        std.debug.print(
            "Could not open file {s}: {} ",
            .{ path, err },
        );
        std.process.exit(74);
    };

    defer file.close();
    const n = try file.read(&buf);
    const data = buf[0..n];
    return data;
}

fn runFile(path: []const u8) !void {
    const source = try readFile(path);
    const result = vm.interpret(source);
    if (result == IterpretResult.INTERPRET_COMPILE_ERROR) std.process.exit(65);
    if (result == IterpretResult.INTERPRET_RUNTIME_ERROR) std.process.exit(70);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const config = try Config.parse(args);
    switch (args.len) {
        1 => repl(),
        2 => try runFile(args[1]),
        3 => try runFile(args[1]),
        else => {
            std.debug.print("Usage: reef [path]\n", .{});
            std.process.exit(64);
        },
    }

    var vm_instance = VM.init(config);
    vm = &vm_instance;
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
