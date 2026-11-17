#!/bin/sh
# usage: build.sh <output-binary>   (std-only Rust, no cargo needed)
rustc -O -o "$1" "$(dirname "$0")/certverify.rs"
