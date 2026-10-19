#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d)}
mkdir -p "$output"
for name in q6-intrinsic-Kummer-charts q7-Frobenius-coordinate-circuits q7c-invariant-rational-generators m1-modular-base; do cp "$expected/$name.json" "$output/"; done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --round10-polynomials "$output"
"$runner" --verify-round10-polynomials "$output"
"$runner" --round10-expand "$output"
"$runner" --round10-projective-control "$output"
"$runner" --round10-divisors "$output"
"$runner" --round10-collision "$output"
"$runner" --round10-diagonal "$output"
for name in q8-F2-polynomial-templates q8-F2-presentations-summary q8-graph-{0..3}-F2-relations q8b-parsed-F2-presentation-check q8c-expanded-F2-equations q8p-projective-saturation-prototype m13-coupled-cusp-valuations m13b-complete-product-divisor m14-Phi11-collision-resultant m15-supersingular-diagonal-component; do cmp "$expected/$name.json" "$output/$name.json"; done
printf 'PASS:all13 new JSON records replay byte for byte;all equation,divisor and bounded support checks pass.\n'
