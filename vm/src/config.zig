const std = @import("std");

pub const Config = struct {
    debugTrace: bool = false,

    pub fn parse(args: []const []const u8) !Config {
        var config = Config{};
        for (args[1..]) |arg| {
            if (std.mem.eql(u8, arg, "--debug-trace")) {
                config.debugTrace = true;
            }
        }
        return config;
    }
};
