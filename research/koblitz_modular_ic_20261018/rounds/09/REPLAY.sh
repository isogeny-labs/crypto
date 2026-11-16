#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d)}
mkdir -p "$output"
for name in q6-intrinsic-Kummer-charts q6c-global-chart-gluing m8-full-map-principal-row m11-fixed-infinity-row n4-independent-seed-targets; do
  cp "$expected/$name.json" "$output/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --round09-controls "$output"
"$runner" --round09-descent "$output"
"$runner" --verify-round09-descent "$output"
"$runner" --round09-invariants "$output"
for name in l7-diagonal-twist-reflection-controls m12-coupled-function-restrictions q7-Frobenius-coordinate-circuits q7b-parsed-descent-chart-check q7c-invariant-rational-generators; do
  cmp "$expected/$name.json" "$output/$name.json"
done
printf 'PASS:all5 new JSON records replay byte for byte; exact descent parser and trace-dual recovery checks pass.\n'
