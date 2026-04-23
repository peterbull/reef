const std = @import("std");

pub const Config = struct {
    debug_trace: bool = false,
    debug_print_code: bool = false,
    const Self = @This();
    pub fn parse(self: *Self, args: []const []const u8) !void {
        for (args[1..]) |arg| {
            if (std.mem.eql(u8, arg, "--debug-trace")) {
                self.debug_trace = true;
            }
            if (std.mem.eql(u8, arg, "--debug-print-code")) {
                self.debug_print_code = true;
            }
        }
    }
};
