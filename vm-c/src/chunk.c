#include "chunk.h"
#include "memory.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

void init_chunk(Chunk *chunk) {
  chunk->count = 0;
  chunk->capacity = 0;
  chunk->code = NULL;
  chunk->lines = NULL;
  chunk->line_entries = 0;
  init_value_array(&chunk->constants);
};

// lldb not great at array ptrs, deep dive for later:
// https://github.com/vadimcn/codelldb/discussions/1362#discussion-9256234
// https://lldb.llvm.org/use/variable.html
// p *((int(*)[10])chunk->lines)


void write_chunk(Chunk *chunk, uint8_t byte, int line) {
  if (chunk->capacity < chunk->count + 1) {
    int old_capacity = chunk->capacity;
    chunk->capacity = GROW_CAPACITY(old_capacity);
    chunk->code =
        GROW_ARRAY(uint8_t, chunk->code, old_capacity, chunk->capacity);
    chunk->lines = GROW_ARRAY(int, chunk->lines, old_capacity, chunk->capacity);
  }
  chunk->code[chunk->count] = byte;
  chunk->lines[chunk->count] = line;

  chunk->count++;
}

void free_chunk(Chunk *chunk) {
  FREE_ARRAY(uint8_t, chunk->code, chunk->capacity);
  FREE_ARRAY(int, chunk->lines, chunk->capacity);
  free_value_array(&chunk->constants);
  init_chunk(chunk);
}

int add_constant(Chunk *chunk, Value value) {
  write_value_array(&chunk->constants, value);
  return chunk->constants.count - 1;
}
