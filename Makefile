.PHONY: build-zig run-zig

build-zig:
	zig build --build-file vm/build.zig

run-zig:
	zig build run --build-file vm/build.zig -freference-trace
	
run-trace:
	zig build run --build-file vm/build.zig -freference-trace -- ./reef/hello.reef --debug-trace
