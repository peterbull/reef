#ifndef creef_debug_h
#define creef_debug_h

#include "chunk.h"

void disassemble_chunk(Chunk *chunk, const char* name);
int disassemble_instruction(Chunk *chunk, int offset);

#endif 
