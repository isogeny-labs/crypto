#!/bin/sh
# Build the generator (W0's qsolve.rs plus the uring arm). No crates, no cargo.
set -eu
cd "$(dirname "$0")"
mkdir -p bin
rustc -O --edition 2021 -C target-cpu=x86-64-v2 -o bin/qsolve src/qsolve.rs
rustc --version
sha256sum src/qsolve.rs bin/qsolve
