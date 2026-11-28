#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
test ! -e "${1:?new stdout path required}"
test ! -e "${2:?new stderr path required}"
cargo build --release --locked --manifest-path tools/torus-section-search/Cargo.toml
set -o noclobber
tools/torus-section-search/target/release/torus-section-search > "$1" 2> "$2"
