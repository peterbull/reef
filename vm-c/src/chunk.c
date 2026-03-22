#include "chunk.h"
#include "memory.h"
#include <stdint.h>
#include <stdio.h>

void init_chunk(Chunk *chunk) {
  chunk->count = 0;
  chunk->capacity = 0;
  chunk->code = NULL;
  chunk->lines = NULL;
  init_value_array(&chunk->constants);
};

// lldb not great at array ptrs, deep dive for later:
// https://github.com/vadimcn/codelldb/discussions/1362#discussion-9256234
// https://lldb.llvm.org/use/variable.html
// p *((int(*)[10])chunk->lines)

static int get_line() {
  // set something like `num_items at line:line`. only
  // runs on runtime errors so doesn't have to be too efficient
  // [4, 123, 5, 126] -- pairs, count first
  // instead of setting single line [123]
  // for line 123
  // get pair [4, 123]
  // if line exists, increment first item by 1 - [5, 123, 5, 126]
  // ex: line 127
  // if it doesn't, add two items, [1, 127]
  // [5, 123, 5, 126, 1, 127]

  return 0;
}

void write_chunk(Chunk *chunk, uint8_t byte, int line) {
  if (chunk->capacity < chunk->count + 1) {
    int old_capacity = chunk->capacity;
    chunk->capacity = GROW_CAPACITY(old_capacity);
    chunk->code =
        GROW_ARRAY(uint8_t, chunk->code, old_capacity, chunk->capacity);
    chunk->lines = GROW_ARRAY(int, chunk->lines, old_capacity, chunk->capacity);
  }
  chunk->code[chunk->count] = byte;

  int entries = 0;
  for (int i = 0; i < chunk->count; i += 2) {
    if (chunk->count == 1) {
      chunk->lines = GROW_ARRAY(int, chunk->lines, 0, 2);
    }

    if (chunk->lines[i + 1] == line) {
      // increment counter for this entry
      chunk->lines[i]++;
    }

    if (i == chunk->count - 2) {
      // TODO: grow array
      chunk->lines = GROW_ARRAY(int, chunk->lines, i, i + 2);
      chunk->lines[entries]++;
      chunk->lines[entries + 1] = line;
    }

    entries += 2;
    printf("muh chunks, %d", chunk->lines[i]);
    fflush(stdout);
  }
  printf("chunk lines: ");
  printf("[");
  for (int i =0; i < entries; i++ ) {
    printf("%d, ", chunk->lines[i]);
  }
  printf("]\n");
  // chunk->lines[chunk->count] = line;
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
