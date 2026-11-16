#!/bin/sh
set -eu
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out=${HEC_HARVEST_OUT:-"$study"}
mkdir -p "$out"
rustc --edition 2021 -O "$study/model.rs" -o "$out/harvest-model"
"$out/harvest-model" "$out" > "$out/CHECKS.txt" 2> "$out/CHECKS.stderr"
shasum -a 256 "$study/model.rs" "$study/PROTOCOL.md" "$out/harvest-model" > "$out/source.sha256"
