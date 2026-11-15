#!/bin/sh
set -eu
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$study/../.." && pwd)
out=${HEC_POINT_COLD_BIN:?set isolated binary directory}
mkdir -p "$out"
cargo build --release --manifest-path "$repo/crates/hec-native-ic/Cargo.toml" --message-format=json > "$out/build.jsonl"
rlib=$(jq -r 'select(.reason=="compiler-artifact" and .target.name=="hec_native_ic") | .filenames[] | select(endswith(".rlib"))' "$out/build.jsonl" | tail -1)
test -n "$rlib"
source_commit=$(git -C "$repo" rev-parse HEAD)
HEC_SOURCE="$source_commit" rustc --edition 2021 -O "$study/worker.rs" --extern "hec_native_ic=$rlib" -L "dependency=$(dirname "$rlib")" -o "$out/worker" 2> "$out/worker-build.stderr"
HEC_SOURCE="$source_commit" rustc --edition 2021 -O "$study/collect.rs" -o "$out/collector"
printf '%s\n' "$source_commit" > "$out/SOURCE.txt"
shasum -a 256 "$out/worker" "$out/collector" "$rlib" > "$out/binaries.sha256"
git -C "$repo" ls-files -z -- crates/hec-native-ic/src research/hec_anchor_relations_20261114 research/hec_table_oracle_20261114 research/hec_lazy_forest_20261115 research/hec_point_sieve_20261115 research/hec_point_cold_20261115 | xargs -0 shasum -a 256 > "$out/inputs.sha256"
