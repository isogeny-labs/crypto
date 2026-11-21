#!/bin/sh
set -eu
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$study/../.." && pwd)
out=${HEC_AFFINE_OUT:-"$study"}
mkdir -p "$out"
cargo build --release --manifest-path "$repo/crates/hec-native-ic/Cargo.toml" --message-format=json > "$out/build.jsonl"
rlib=$(jq -r 'select(.reason=="compiler-artifact" and .target.name=="hec_native_ic") | .filenames[] | select(endswith(".rlib"))' "$out/build.jsonl" | tail -1)
test -n "$rlib"
rustc --edition 2021 -O "$study/gate.rs" --extern "hec_native_ic=$rlib" -L "dependency=$(dirname "$rlib")" -o "$out/affine-gate"
"$out/affine-gate" > "$out/GATE.txt" 2> "$out/GATE.stderr"
shasum -a 256 "$study/gate.rs" "$study/PROTOCOL.md" "$study/SESSION.md" "$repo/crates/hec-native-ic/src/jac.rs" "$rlib" > "$out/source.sha256"
