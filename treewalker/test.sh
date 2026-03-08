#!/bin/bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$DIR"
cargo build
RUST_BACKTRACE=full ./target/debug/reef tokenize "$DIR/../reef/hello.reef"
