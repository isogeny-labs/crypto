#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round14-replay.XXXXXX")}
mkdir -p "$output"
for name in q8-graph-{0..3}-F2-relations q8-F2-polynomial-templates q10c-constant-gate-saturation-step q13-inverse-associative-saturation q14-two-product-colon q15-linear-square-saturation m1-modular-base q5-mixed-translation-atlases gl2b-integral-polarizations i1-degree210-images m4-direct-modular-relations m8-full-map-principal-row m11-fixed-infinity-row m13-coupled-cusp-valuations m13b-complete-product-divisor s19-complete-supersingular-divisor-strata o17-ordinary-projected-images; do cp "$expected/$name.json" "$output/"; done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in norms supports middles products quadrics geometry boundary-lean; do timeout 1200 "$runner" "--round14-$command" "$output"; done
pids=()
wait_batch() {
 local result=0
 for pid in "${pids[@]}"; do if ! wait "$pid"; then result=1; fi; done
 pids=()
 return "$result"
}
for index in $(seq 0 24); do
 (
  timeout 1200 "$runner" --round14-fiber "$output" "$index"
  timeout 1200 "$runner" --round14-verify-point "$output" "$index"
 ) &
 pids+=("$!")
 if [ "${#pids[@]}" -eq 4 ]; then wait_batch; fi
done
wait_batch
timeout 1200 "$runner" --round14-assemble "$output"
timeout 1200 "$runner" --round14-images7 "$output"
for index in 0 1 2 5 12 17 20; do
 timeout 1200 "$runner" --round14-images-point "$output" "$index" &
 pids+=("$!")
 if [ "${#pids[@]}" -eq 4 ]; then wait_batch; fi
done
wait_batch
timeout 1200 "$runner" --round14-relations "$output"
timeout 1200 "$runner" --round14-cusps "$output"
for name in o19-five-Hecke-norms o20-five-pushed-divisors o21-five-middle-candidates o24-five-source-supports o25-degree7-individual-images o27-complete-projected-relations o28-complete-closed-coupled-divisors q16-linear-product-saturation q17-expanded-product-quadrics g14-canonical-polarized-target; do cmp "$expected/$name.json" "$output/$name.json"; done
for index in 0 1 2 5 12 17 20; do cmp "$expected/o26-point-$index-images.json" "$output/o26-point-$index-images.json"; done
for index in $(seq 0 4); do for stem in o19-function o20-function; do cmp "$expected/$stem-$index.json" "$output/$stem-$index.json"; done; done
for index in $(seq 0 24); do for stem in o21-point o22-point o23-point; do cmp "$expected/$stem-$index.json" "$output/$stem-$index.json"; done; done
for name in KoblitzRound14Boundary KoblitzRound14Supports KoblitzRound14Relations KoblitzRound14Cusps; do cmp "$expected/$name.lean" "$output/$name.lean"; cmp "$study/lean/$name.lean" "$output/$name.lean"; done
"$runner" --round14-check-identities "$round"
printf 'PASS:102 new JSON/checkpoint outputs and4 generated Lean modules replay byte for byte;5 source-bound identities parsed and checked.\n'
