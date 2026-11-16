#!/bin/sh
set -eu
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$study/../.." && pwd)
out=${HEC_SIEVE_GATE_OUT:-"$study"}
mkdir -p "$out"
cargo build --release --manifest-path "$repo/crates/hec-native-ic/Cargo.toml" --message-format=json > "$out/build.jsonl"
rlib=$(jq -r 'select(.reason=="compiler-artifact" and .target.name=="hec_native_ic") | .filenames[] | select(endswith(".rlib"))' "$out/build.jsonl" | tail -1)
test -n "$rlib"
deps=$(dirname "$rlib")
rustc --edition 2021 -O "$study/gate.rs" --extern "hec_native_ic=$rlib" -L "dependency=$deps" -o "$out/point-sieve-gate"
"$out/point-sieve-gate" "$study/fixtures" > "$out/GATE.txt" 2> "$out/GATE.stderr"
rustc --edition 2021 "$study/controls.rs" --extern "hec_native_ic=$rlib" -L "dependency=$deps" -o "$out/point-sieve-controls"
"$out/point-sieve-controls" > "$out/CONTROLS.txt" 2> "$out/CONTROLS.stderr"
shasum -a 256 "$study/gate.rs" "$study/sieve.rs" "$study/controls.rs" "$study/PROTOCOL.md" \
 "$repo/research/hec_lazy_forest_20261115/lazy.rs" "$repo/research/hec_anchor_relations_20261114/dense.rs" \
 "$study"/fixtures/*.out "$rlib" > "$out/source.sha256"
jq -Rn -f "$study/summary.jq" < "$out/GATE.txt" > "$out/SUMMARY.json"
