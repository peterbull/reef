const std = @import("std");
const chunk_mod = @import("chunk.zig");
const Chunk = chunk_mod.Chunk;
const OpCode = chunk_mod.OpCode;

pub fn printValue(value: f64) void {
    std.debug.print("{d}\n", .{value});
}
pub fn simpleInstruction(name: []const u8, offset: usize) usize {
    std.debug.print("{s}\n", .{name});
    return offset + 1;
}

pub fn constantInstruction(name: []const u8, chunk: *Chunk, offset: usize) usize {
    const constant: u8 = chunk.code.items.ptr[offset + 1];
    std.debug.print("{s: >16} {d:0>4}\n", .{ name, constant });
    printValue(chunk.constants.items.ptr[constant]);
    std.debug.print("\n", .{});

    return offset + 2;
}

pub fn disassembleChunk(chunk: *Chunk, name: []const u8) void {
    std.debug.print("== {s} == \n", .{name});
    var offset: usize = 0;
    while (offset < chunk.code.items.len) {
        offset = disassembleInstruction(chunk, offset);
    }
}
pub fn disassembleInstruction(chunk: *Chunk, offset: usize) usize {
    std.debug.print("{d:0>4} ", .{offset});
    if ((offset > 0) and (chunk.lines.items[offset] == chunk.lines.items[offset - 1])) {
        std.debug.print("   | ", .{});
    } else {
        std.debug.print("{d:>4} ", .{chunk.lines.items.ptr[offset]});
    }
    const instruction = chunk.code.items.ptr[offset];
    const op: OpCode = @enumFromInt(instruction);
    switch (op) {
        .OP_CONSTANT => {
            return constantInstruction(@tagName(OpCode.OP_CONSTANT), chunk, offset);
        },
        .OP_RETURN => {
            return simpleInstruction(@tagName(OpCode.OP_RETURN), offset);
        },

        // else => {
        //     std.debug.print("unknown op code {d} ", .{instruction});
        //     return offset - 1;
        // },
    }
}
