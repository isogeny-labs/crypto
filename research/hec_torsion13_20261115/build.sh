#!/bin/sh
set -eu
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
: "${CARGO_TARGET_DIR:?set an isolated target directory}"
cargo build --offline --locked --release --manifest-path "$study/Cargo.toml"
