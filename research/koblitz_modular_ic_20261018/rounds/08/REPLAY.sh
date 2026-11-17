#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d)}
mkdir -p "$output"
for name in r3b-complete-split-fiber r3-reverse-fiber-screen-negative n4-independent-seed-targets m8-full-map-principal-row m9-second-full-map-row m4-direct-modular-relations q5-mixed-translation-atlases; do
  cp "$expected/$name.json" "$output/"
done
for i in {00..11}; do cp "$expected/r3b-negative-fiber-neighbor-$i.json" "$output/"; done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --round08-relations "$output"
"$runner" --round08-infinity "$output"
"$runner" --verify-round08-relations "$output"
"$runner" --round08-kummer "$output"
"$runner" --verify-round08-kummer "$output"
"$runner" --round08-kummer-glue "$output"
"$runner" --verify-round08-kummer-glue "$output"
for name in m10-full-line-family m11-fixed-infinity-row m10b-small-field-check m11b-small-field-check q6-intrinsic-Kummer-charts q6b-parsed-Miller-check q6c-global-chart-gluing q6d-parsed-gluing-check; do
  cmp "$expected/$name.json" "$output/$name.json"
done
printf 'PASS:all8 new JSON records replay byte for byte; separate relation,Miller and gluing checks pass.\n'
