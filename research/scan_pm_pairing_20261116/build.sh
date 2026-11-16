#!/bin/sh
# Thin build orchestration. Builds the repository library and its example
# `ic_algebraic_floor` with cargo (unchanged Cargo.toml), then compiles this
# study's harness against that library with rustc.  No file outside the
# target directory is written.
#   T=<target dir> sh research/scan_pm_pairing_20261116/build.sh
set -eu
cd "$(dirname "$0")/../.."
T=${T:-${CARGO_TARGET_DIR:-target}}
CARGO_TARGET_DIR=$T cargo build --release --example ic_algebraic_floor
RLIB=$(ls -t "$T"/release/deps/libcrypto_lib-*.rlib | head -1)
rustc --edition 2021 -C opt-level=3 --crate-name scan_pm \
  research/scan_pm_pairing_20261116/main.rs \
  --extern crypto_lib="$RLIB" -L dependency="$T/release/deps" -o "$T/scan_pm"
sha256sum "$T/scan_pm" "$RLIB"
