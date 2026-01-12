#!/bin/bash

set -e

cargo build

RUST_BACKTRACE=FULL ./target/debug/reef tokenize ./reef/hello.reef
