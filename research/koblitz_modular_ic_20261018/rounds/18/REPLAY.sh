#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round18-replay.XXXXXX")}
mkdir -p "$output"
for name in q8-graph-{0..3}-F2-relations q24-after-all-ordinary-cube-repairs q23-survivor-source-separators q13-inverse-associative-saturation q14-two-product-colon q15-linear-square-saturation q16-linear-product-saturation q17-expanded-product-quadrics g17-odd-cover-kernel-and-commutators g18-factored-odd-kernel-theta-lifts g16-diagonal-cover-section-bases cm17-explicit-level77-incidence; do
 cp "$expected/$name.json" "$output/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in companion repair squares square-repair symmetric frobenius refinement diamond hecke maps holdouts map-proof; do
 timeout 1200 "$runner" "--round18-$command" "$output"
done
for name in q25-companion-source-separators q26-after-companion-cube-repair q27-square-cube-source-separators q28-after-source-square-repairs g19-canonical-symmetric-theta-characters g20-odd-cover-rotation-lift-obstruction g21-rotation-compatible-scalar-refinement cm19-explicit-diamond2-correspondence cm20-shared-residual-Hecke-systems cm21-affine-linear-elliptic-map-search cm22-independent-elliptic-map-holdouts; do
 cmp "$expected/$name.json" "$output/$name.json"
done
for name in KoblitzRound18Companion KoblitzRound18Boundary KoblitzRound18Squares KoblitzRound18FinalBoundary KoblitzRound18Theta KoblitzRound18Frobenius KoblitzRound18Refinement KoblitzRound18Hecke KoblitzRound18MapCounterexamples; do
 cmp "$expected/$name.lean" "$output/$name.lean"
 cmp "$study/lean/$name.lean" "$output/$name.lean"
done
"$runner" --round18-check-identities "$round"
printf 'PASS:11 native JSON outputs and9 generated Lean modules replay byte for byte;3 source-bound IDC records checked.\n'
