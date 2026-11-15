#!/usr/bin/env bash
set -euo pipefail
study="$(cd "$(dirname "$0")" && pwd)"
manifest="$study/Cargo.toml"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
cargo fmt --manifest-path "$manifest" -p p256-transfer-search --check
cargo test --release --locked --manifest-path "$manifest" -p p256-transfer-search
cargo clippy --release --locked --no-deps --manifest-path "$manifest" -p p256-transfer-search --all-targets -- -D warnings
for mode in audit identities; do
  cargo run --release --locked --manifest-path "$manifest" -- "$mode" > "$scratch/output.json"
  diff -u "$study/results/$mode.json" "$scratch/output.json"
done
