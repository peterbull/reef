pub const VM = struct {
    const Self = @This();
    pub fn init() void {
        return Self{};
    }
    pub fn deinit() void {}
};

var vm: VM = undefined;
