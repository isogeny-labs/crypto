#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round13-replay.XXXXXX")}
mkdir -p "$output"
for name in q8-graph-{0..3}-F2-relations q8-F2-polynomial-templates q10c-constant-gate-saturation-step m1-modular-base; do cp "$expected/$name.json" "$output/"; done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in norm supports middles fibers images saturation pair-colon square-colon boundary-lean square-lean; do
 timeout 1200 "$runner" "--round13-$command" "$output"
done
timeout 1200 "$runner" --verify-round13 "$output"
for name in o13-cyclic121-Hecke-norms o14-pushed-closed-supports o15-ordinary-middle-candidates o16-ordinary-source-fibers o17-ordinary-projected-images o18-parsed-function-and-fiber-check q13-inverse-associative-saturation q14-two-product-colon q15-linear-square-saturation o16-fn0-point4 o16-fn0-point5 o16-fn1-point0 o16-fn1-point1 o16-fn1-point2 o16-fn1-point3; do cmp "$expected/$name.json" "$output/$name.json"; done
for name in KoblitzRound13Data KoblitzRound13Boundary KoblitzRound13SquareData; do cmp "$expected/$name.lean" "$output/$name.lean"; cmp "$study/lean/$name.lean" "$output/$name.lean"; done
printf 'PASS: all15 new JSON/checkpoint outputs and3 generated Lean modules replay byte for byte.\n'
