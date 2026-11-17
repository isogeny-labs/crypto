#!/usr/bin/env bash
# Exact arithmetic replay. Deliberately does not repeat bounded normalization jobs.
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round22-replay.XXXXXX")}
mkdir -p "$output"
for name in g25-correct-polarization-orbit-sections g28-integral-orbit-retraction g31-equivariant-recovery-denominator q40-after-all-elliptic-source-families q8-graph-{0..3}-F2-relations; do
  cp "$expected/$name.json" "$output/"
done
cp "$expected/cm22-projection-01.log" "$expected/cm22-incidence.sing" "$expected/cm22-generic-qr.sing" "$output/"
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
target=${CARGO_TARGET_DIR:-"$study/native/target"}
runner=${R22_RUNNER:-"$target/release/koblitz-modular-carriers"}
for command in recovery verify-recovery two-map coupled base-divisors exceptional-supports toric-model gate-check; do
  (ulimit -v 4194304; timeout 1200 "$runner" "--round22-$command" "$output")
done
for name in g32-normzero-integral-recovery g32-independent-integer-replay g33-small-two-map-recovery q41-node187-coupled-valuations cm22-base-divisors cm22-exceptional-supports cm22-toric-model identity-recovery-{0..7}; do
  cmp "$expected/$name.json" "$output/$name.json"
done
for name in KoblitzRound22Recovery KoblitzRound22TwoMapData; do
  cmp "$expected/$name.lean" "$output/$name.lean"
  cmp "$study/lean/$name.lean" "$output/$name.lean"
done
cmp "$expected/cm22-rational-gate-check.sing" "$output/cm22-rational-gate-check.sing"
"$runner" --round22-check-identities "$round"
printf 'PASS: full integer recovery, independent verifier, two-map certificate, coupled source bindings and base arithmetic replay byte for byte.\n'
