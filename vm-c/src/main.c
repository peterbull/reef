#include "chunk.h"
#include "debug.h"
#include <stdio.h>

int main() {
  printf("running...\n");
  Chunk test_chunk;
  init_chunk(&test_chunk);
  int constant = add_constant(&test_chunk, 1.4);
  int constant2 = add_constant(&test_chunk, 1.9);
  write_chunk(&test_chunk, OP_CONSTANT, 123);
  write_chunk(&test_chunk, constant, 123);
  write_chunk(&test_chunk, OP_CONSTANT, 123);
  write_chunk(&test_chunk, constant2, 127);
  write_chunk(&test_chunk, OP_CONSTANT, 127);
  write_chunk(&test_chunk, constant, 128);
  write_chunk(&test_chunk, OP_CONSTANT, 128);
  write_chunk(&test_chunk, constant, 129);
  write_chunk(&test_chunk, OP_CONSTANT, 129);
  write_chunk(&test_chunk, constant, 123);
  write_chunk(&test_chunk, OP_CONSTANT, 123);
  write_chunk(&test_chunk, constant, 123);
  write_chunk(&test_chunk, OP_CONSTANT, 123);
  write_chunk(&test_chunk, constant, 126);
  write_chunk(&test_chunk, OP_RETURN, 126);
  disassemble_chunk(&test_chunk, "test_chunk");
  free_chunk(&test_chunk);
  return 0;
}
