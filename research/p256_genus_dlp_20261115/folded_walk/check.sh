#!/usr/bin/env bash
set -euo pipefail
study="$(cd "$(dirname "$0")" && pwd)"
manifest="$study/Cargo.toml"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cargo fmt --manifest-path "$manifest" -p p256-folded-walk --check
cargo test --release --locked --manifest-path "$manifest" -p p256-folded-walk
cargo clippy --release --locked --no-deps --manifest-path "$manifest" -p p256-folded-walk --all-targets -- -D warnings
for mode in toy trace; do
  cargo run --release --locked --manifest-path "$manifest" -- "$mode" > "$tmp/$mode.json"
  diff -u "$study/results/$mode.json" "$tmp/$mode.json"
done
