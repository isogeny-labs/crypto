#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round20-replay.XXXXXX")}
mkdir -p "$output"
for name in q8-graph-{0..3}-F2-relations q13-inverse-associative-saturation q14-two-product-colon q15-linear-square-saturation q16-linear-product-saturation q17-expanded-product-quadrics q23-survivor-source-separators q25-companion-source-separators q27-square-cube-source-separators q28-after-source-square-repairs q29-coupled-elliptic-resultants q31-pure-source-resultant-family q32-after-pure-source-repairs cm17-explicit-level77-incidence cm22-independent-elliptic-map-holdouts g15-theta-class-descent g16-diagonal-cover-section-bases g17-odd-cover-kernel-and-commutators g21-rotation-compatible-scalar-refinement g22-mixed-F2-orbit-sections g25-correct-polarization-orbit-sections; do
  cp "$expected/$name.json" "$output/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for q in 0 1 2 3 4 5 6 7; do timeout 1200 "$runner" --round20-maps "$output" "$q"; done
for command in picard kernel retraction sections quotient boundary map-summary projector; do
  timeout 1200 "$runner" "--round20-$command" "$output"
done
for name in cm25-quadratic-controlled-poles-q{0..7} cm26-quadratic-map-summary g26-canonical-orbit-Pic2-matching g27-common-orbit-map-kernel g28-integral-orbit-retraction g29-full-weight-orbit-sections g30-proper-orbit-image-equations q33-coupled-quotient-resultant q34-after-coupled-quotient-repair; do
  cmp "$expected/$name.json" "$output/$name.json"
done
for name in KoblitzRound20Picard KoblitzRound20Kernel KoblitzRound20Retraction KoblitzRound20Sections KoblitzRound20Separators KoblitzRound20Boundary KoblitzRound20Maps KoblitzRound20Projector; do
  cmp "$expected/$name.lean" "$output/$name.lean"
  cmp "$study/lean/$name.lean" "$output/$name.lean"
done
"$runner" --round20-check-identities "$round"
printf 'PASS:16 JSON outputs and8 generated Lean modules replay byte for byte;3 source-bound IDC records checked.\n'
