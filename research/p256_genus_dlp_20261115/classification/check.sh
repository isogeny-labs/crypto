#!/usr/bin/env bash
set -euo pipefail
study="$(cd "$(dirname "$0")" && pwd)"
manifest="$study/Cargo.toml"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
cargo fmt --manifest-path "$manifest" -p p256-cover-classification --check
cargo test --release --locked --manifest-path "$manifest" -p p256-cover-classification
cargo clippy --no-deps --release --locked --manifest-path "$manifest" -p p256-cover-classification --all-targets -- -D warnings
for item in 'smith:results/smith-p256' 'controls:results/reference-controls' 'catalogue:catalog'; do
  mode="${item%%:*}"
  output="${item#*:}"
  cargo run --release --locked --manifest-path "$manifest" -- "$mode" > "$scratch/result.json"
  diff -u "$study/$output.json" "$scratch/result.json"
done
