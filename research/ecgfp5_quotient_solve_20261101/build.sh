#!/usr/bin/env bash
# Build the standalone generator/checker/runner (no crates, no cargo). Records the binary hash.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$HERE/bin"
rustc -O --edition 2021 -C target-cpu=x86-64-v2 -o "$HERE/bin/qsolve" "$HERE/src/qsolve.rs"
rustc --version
sha256sum "$HERE/src/qsolve.rs" "$HERE/bin/qsolve"
