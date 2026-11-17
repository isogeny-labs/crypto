#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round11-replay.XXXXXX")}
mkdir -p "$output"
cp "$expected/m15-supersingular-diagonal-component.json" "$output/"
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --round11-local "$output"
"$runner" --verify-round11-local "$output"
"$runner" --round11-saturation "$output"
"$runner" --round11-relations "$output"
for name in m16-local-diagonal-contacts m16b-parsed-local-check q9-coupled-projective-saturation m17-residual-correspondences; do cmp "$expected/$name.json" "$output/$name.json"; done
printf 'PASS: all4 new JSON records replay byte for byte; local contacts, saturation and residual correspondences checked.\n'
