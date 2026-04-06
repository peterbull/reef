const std = @import("std");
const chunk_mod = @import("chunk.zig");
const Chunk = chunk_mod.Chunk;
const OpCode = chunk_mod.OpCode;

pub fn print_value(value: f64) void {
    std.debug.print("{d}", .{value});
}
pub fn simple_instruction(name: []const u8, offset: usize) usize {
    std.debug.print("{s}\n", .{name});
    return offset + 1;
}

pub fn constant_instruction(name: []const u8, chunk: *Chunk, offset: usize) usize {
    const constant: u8 = chunk.code.items.ptr[offset + 1];
    std.debug.print("{s:<16} {d:>4} '{d}'\n", .{ name, constant, chunk.constants.items[constant] });
    return offset + 2;
}

pub fn disassemble_instruction(chunk: *Chunk, offset: usize) usize {
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
            return constant_instruction(@tagName(OpCode.OP_CONSTANT), chunk, offset);
        },
        .OP_NEGATE => {
            return simple_instruction(@tagName(OpCode.OP_NEGATE), offset);
        },
        .OP_RETURN => {
            return simple_instruction(@tagName(OpCode.OP_RETURN), offset);
        },
        .OP_ADD => {
            return simple_instruction(@tagName(OpCode.OP_ADD), offset);
        },
        .OP_SUBTRACT => {
            return simple_instruction(@tagName(OpCode.OP_SUBTRACT), offset);
        },
        .OP_MULTIPLY => {
            return simple_instruction(@tagName(OpCode.OP_MULTIPLY), offset);
        },
        .OP_DIVIDE => {
            return simple_instruction(@tagName(OpCode.OP_DIVIDE), offset);
        },
        // else => {
        //     std.debug.print("unknown op code {d} ", .{instruction});
        //     return offset - 1;
        // },
    }
}

pub fn disassemble_chunk(chunk: *Chunk, name: []const u8) void {
    std.debug.print("== {s} == \n", .{name});
    var offset: usize = 0;
    while (offset < chunk.code.items.len) {
        offset = disassemble_instruction(chunk, offset);
    }
}
