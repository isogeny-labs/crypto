#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round12-replay.XXXXXX")}
mkdir -p "$output"
for name in m15-supersingular-diagonal-component m16-local-diagonal-contacts q8-graph-{0..3}-F2-relations q8-F2-polynomial-templates; do cp "$expected/$name.json" "$output/"; done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --round12-boundary "$output"
"$runner" --round12-boundary-lean "$output"
"$runner" --round12-constant-saturation "$output"
"$runner" --round12-ss49 "$output"
"$runner" --verify-round12-local "$output"
for name in q10-full-homogenized-boundary-counterpoints q10c-constant-gate-saturation-step s18-supersingular49-coordinate-adapter s19-complete-supersingular-divisor-strata s20-parsed-local-and-divisor-check; do cmp "$expected/$name.json" "$output/$name.json"; done
cmp "$expected/KoblitzRound12Boundary.lean" "$output/KoblitzRound12Boundary.lean"
printf 'PASS: all5 new JSON outputs and actual-boundary Lean data replay byte for byte.\n'
