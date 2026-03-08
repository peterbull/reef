#!/bin/bash
set -e
cargo build
RUST_BACKTRACE=1 ./target/debug/reef repl
