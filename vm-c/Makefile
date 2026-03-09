# You can change these variables if you want to use a different compiler or debugger
CC = clang
DBG = lldb

SRC = $(wildcard src/*.c)
OBJ = $(SRC:src/%.c=build/%.o)

.PHONY: all run test debug debug-test check clean

all: build/main

build:
	mkdir -p build

build/%.o: src/%.c | build
	$(CC) -Wall -Wextra -O0 -g -c $< -o $@

build/main: $(OBJ)
	$(CC) -o $@ $(OBJ)

build/test: src/test.c | build
	$(CC) -Wall -Wextra -O0 -g -o $@ $<

run: build/main
	./build/main

test: build/test
	./build/test

debug: build/main
	$(DBG) ./build/main

debug-test: build/test
	$(DBG) ./build/test

check:
	@which $(CC)  > /dev/null && echo "SUCCESS: $(CC) found"  || echo "ERROR: $(CC) not found"
	@which $(DBG) > /dev/null && echo "SUCCESS: $(DBG) found" || echo "ERROR: $(DBG) not found"

clean:
	rm -rf build
