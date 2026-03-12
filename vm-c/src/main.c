#include "chunk.h"
#include "debug.h"
#include <stdio.h>

int main() {
  printf("running...\n");
  Chunk test_chunk;
  init_chunk(&test_chunk);
  write_chunk(&test_chunk, OP_RETURN);
  disassemble_chunk(&test_chunk, "test_chunk");
  free_chunk(&test_chunk);
  return 0;
}
