#!/bin/sh
# Native correctness gate. No wall-time experiment or scientific Python.
set -eu
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$study/../.." && pwd)
out=${HEC_LAZY_GATE_OUT:-"$study"}
mkdir -p "$out"
cargo build --release --manifest-path "$repo/crates/hec-native-ic/Cargo.toml" \
  --message-format=json > "$out/build.jsonl"
rlib=$(jq -r 'select(.reason == "compiler-artifact" and .target.name == "hec_native_ic") | .filenames[] | select(endswith(".rlib"))' "$out/build.jsonl" | tail -1)
test -n "$rlib"
deps=$(dirname "$rlib")
rustc --edition 2021 --test "$study/lazy.rs" --extern "hec_native_ic=$rlib" \
  -L "dependency=$deps" -o "$out/lazy-gate"
"$out/lazy-gate" --nocapture > "$out/GATE.txt"
rustc --edition 2021 -O --test "$study/lazy.rs" --extern "hec_native_ic=$rlib" \
  -L "dependency=$deps" -o "$out/lazy-gate-release"
"$out/lazy-gate-release" > "$out/GATE-RELEASE.txt"
jq -Rn -f "$study/counts.jq" < "$out/GATE.txt" > "$out/COUNTS.json"
shasum -a 256 "$study/lazy.rs" "$study/PROTOCOL.md" \
  "$repo/research/hec_anchor_relations_20261114/dense.rs" "$rlib" > "$out/source.sha256"
