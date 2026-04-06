const std = @import("std");

pub const Config = struct {
    debug_trace: bool = false,
    pub fn parse(args: []const []const u8) !Config {
        var config = Config{};
        for (args[1..]) |arg| {
            if (std.mem.eql(u8, arg, "--debug-trace")) {
                config.debug_trace = true;
            }
        }
        return config;
    }
};
