#!/bin/bash
set -xe

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p "$DIR/build"
clang -Wall -Wextra -o "$DIR/build/main" "$DIR/src/main.c"
