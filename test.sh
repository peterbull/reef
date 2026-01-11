#!/bin/bash

set -e

cargo build

RUST_BACKTRACE=FULL ./target/debug/interpreter tokenize ./lox/hello.lox
