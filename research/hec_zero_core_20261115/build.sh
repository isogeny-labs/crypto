#!/bin/sh
set -eu
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$study/../.." && pwd)
out=${HEC_ZERO_CORE_BIN:?set isolated build directory}
mkdir -p "$out"
cargo build --release --manifest-path "$repo/crates/hec-native-ic/Cargo.toml" --message-format=json > "$out/build.jsonl"
rlib=$(jq -r 'select(.reason=="compiler-artifact" and .target.name=="hec_native_ic") | .filenames[] | select(endswith(".rlib"))' "$out/build.jsonl" | tail -1)
test -n "$rlib"
rustc --edition 2021 -O "$study/gate.rs" --extern "hec_native_ic=$rlib" -L "dependency=$(dirname "$rlib")" -o "$out/gate" 2> "$out/build.stderr"
rustc --edition 2021 -O "$study/worker.rs" --extern "hec_native_ic=$rlib" -L "dependency=$(dirname "$rlib")" -o "$out/worker" 2> "$out/worker-build.stderr"
HEC_SOURCE=$(git -C "$repo" rev-parse HEAD) rustc --edition 2021 -O "$study/collect.rs" -o "$out/collector"
git -C "$repo" rev-parse HEAD > "$out/SOURCE.txt"
shasum -a 256 "$out/gate" "$rlib" "$study"/*.rs "$study/PROTOCOL.md" "$repo/research/hec_dlp_graph_20261115/core.rs" "$repo/research/hec_anchor_relations_20261114/dense.rs" "$repo"/research/hec_point_sieve_20261115/fixtures/select-g2-*.out > "$out/source.sha256"
