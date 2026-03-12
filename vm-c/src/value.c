#include "value.h"
#include "memory.h"
#include <stdio.h>

void init_value_array(ValueArray *array) {
  array->count = 0;
  array->capacity = 0;
  array->value = NULL;
}
void write_value_array(ValueArray *array, Value value) {
  if (array->capacity < array->count + 1) {
    int old_capacity = array->capacity;
    array->capacity = GROW_CAPACITY(old_capacity);
    array->value =
        GROW_ARRAY(Value, array->value, old_capacity, array->capacity);
  }
  array->value[array->count] = value;
  array->count++;
}
void free_value_array(ValueArray *array) {
  FREE_ARRAY(Value, array->value, array->count);
  init_value_array(array);
}

void print_value(Value value) { printf("%g", value); };
