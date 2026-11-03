#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round19-replay.XXXXXX")}
mkdir -p "$output"
for name in q8-graph-{0..3}-F2-relations q13-inverse-associative-saturation q14-two-product-colon q15-linear-square-saturation q16-linear-product-saturation q17-expanded-product-quadrics q23-survivor-source-separators q25-companion-source-separators q27-square-cube-source-separators q28-after-source-square-repairs cm17-explicit-level77-incidence cm22-independent-elliptic-map-holdouts g17-odd-cover-kernel-and-commutators g16-diagonal-cover-section-bases g21-rotation-compatible-scalar-refinement; do
  cp "$expected/$name.json" "$output/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for q in 0 1 2 3 4; do timeout 1200 "$runner" --round19-poles "$output" "$q"; done
for command in map-proof mixed polarization orbit-obstruction orbit-cover profile resultant repair family final-repair; do
  timeout 1200 "$runner" "--round19-$command" "$output"
done
for name in cm23-controlled-poles-q{0..4} cm24-controlled-pole-summary g22-mixed-F2-orbit-sections g23-integral-orbit-polarization-search g24-orbit-transfer-mod5-obstruction g25-correct-polarization-orbit-sections q29-source-profiles q29-coupled-elliptic-resultants q30-after-coupled-norm-repair q31-pure-source-resultant-family q32-after-pure-source-repairs; do
  cmp "$expected/$name.json" "$output/$name.json"
done
for name in KoblitzRound19Maps KoblitzRound19Separators KoblitzRound19Boundary KoblitzRound19Family KoblitzRound19FinalBoundary KoblitzRound19Polarization KoblitzRound19OrbitCover; do
  cmp "$expected/$name.lean" "$output/$name.lean"
  cmp "$study/lean/$name.lean" "$output/$name.lean"
done
"$runner" --round19-check-identities "$round"
printf 'PASS:15 JSON outputs and7 generated Lean modules replay byte for byte;3 source-bound IDC records checked.\n'
