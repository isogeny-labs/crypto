#!/usr/bin/env bash
set -euo pipefail
study="$(cd "$(dirname "$0")/.." && pwd)"
manifest="$study/native/Cargo.toml"
check_dir="$(mktemp -d)"
trap 'rm -rf "$check_dir"' EXIT
cargo fmt --manifest-path "$manifest" --check
cargo test --release --locked --manifest-path "$manifest"
cargo clippy --release --locked --manifest-path "$manifest" --all-targets -- -D warnings
for item in 'verify:native-p256' 'toy-correctness:native-toy' 'toy-zeta:native-zeta' 'certificates:identities'; do
  mode="${item%%:*}"
  filename="${item#*:}"
  cargo run --release --locked --manifest-path "$manifest" -- "$mode" > "$check_dir/$filename.json"
  diff -u "$study/results/$filename.json" "$check_dir/$filename.json"
done
