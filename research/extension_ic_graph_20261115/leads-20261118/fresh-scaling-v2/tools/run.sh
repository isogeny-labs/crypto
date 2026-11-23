#!/usr/bin/env bash
# Thin build/run wrapper for the fresh-scaling-v2 design-time rule check.
# Single-file rustc build (no crates), output outside the repository.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
v2="$(dirname "$here")"
out_dir="${CARGO_TARGET_DIR:-/home/user/cargo-target-round2}/fresh-scaling-v2"
mkdir -p "$out_dir"
RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-1.98}" rustc -V
RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-1.98}" rustc --edition 2021 -O -C overflow-checks=on \
  "$here/rule_check.rs" -o "$out_dir/rule_check"
sha256sum "$here/rule_check.rs" "$out_dir/rule_check"
"$out_dir/rule_check" "$v2/../fresh-scaling/estimate.csv" "$v2/estimate-v2.csv" | tee "$v2/rule-check.txt"
