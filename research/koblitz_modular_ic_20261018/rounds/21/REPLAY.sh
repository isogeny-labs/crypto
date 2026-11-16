#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round21-replay.XXXXXX")}
mkdir -p "$output"
for name in q8-graph-{0..3}-F2-relations q13-inverse-associative-saturation q14-two-product-colon q15-linear-square-saturation q16-linear-product-saturation q17-expanded-product-quadrics q23-survivor-source-separators q25-companion-source-separators q27-square-cube-source-separators q31-pure-source-resultant-family q33-coupled-quotient-resultant q34-after-coupled-quotient-repair g25-correct-polarization-orbit-sections g28-integral-orbit-retraction; do
  cp "$expected/$name.json" "$output/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in tangent descent boundary quotients final-boundary families last-boundary; do
  (ulimit -v 4194304; timeout 1200 "$runner" "--round21-$command" "$output")
done
  "$runner" --round21-summary "$output"
for name in round21-native-summary q35-trace-zero-tangent-obstruction g31-equivariant-recovery-denominator q36-after-trace-zero-tangent-repair q37-all-ordinary-coupled-resultants q38-after-all-ordinary-norm-repairs q39-all-elliptic-pure-source-families q40-after-all-elliptic-source-families; do
    cmp "$expected/$name.json" "$output/$name.json"
  done
  for name in KoblitzRound21TangentData KoblitzRound21DescentData KoblitzRound21Boundary KoblitzRound21Separators KoblitzRound21FinalBoundary KoblitzRound21Family KoblitzRound21LastBoundary; do
    cmp "$expected/$name.lean" "$output/$name.lean"
    cmp "$study/lean/$name.lean" "$output/$name.lean"
  done

"$runner" --round21-check-identities "$round"

for name in KoblitzRound21 KoblitzRound21Equivariant; do
  cmp "$expected/$name.lean" "$study/lean/$name.lean"
done
