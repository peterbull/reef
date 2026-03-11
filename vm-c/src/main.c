#include "chunk.h"
#include <stdio.h>

int main() {
  printf("running...\n");
  Chunk *test_chunk;
  init_chunk(test_chunk);
  // test_chunk->count = 1;
  write_chunk(test_chunk, OP_RETURN);
  write_chunk(test_chunk, OP_RETURN);
  free_chunk(test_chunk);
  return 0;
}
