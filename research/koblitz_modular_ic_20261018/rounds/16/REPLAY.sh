#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round16-replay.XXXXXX")}
mkdir -p "$output"
for name in cm15-one-prime-carriers g15-theta-class-descent g14-canonical-polarized-target q17-expanded-product-quadrics q8-graph-{0..3}-F2-relations q13-inverse-associative-saturation q14-two-product-colon q15-linear-square-saturation q16-linear-product-saturation; do
 cp "$expected/$name.json" "$output/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in cm ramify sections valuations separators repair carrier; do
 timeout 1200 "$runner" "--round16-$command" "$output"
done
for name in cm16-integral-sign-separation q19-actual-ramification-six g16-diagonal-cover-section-bases q20-original-gate-valuation-propagation q21-source-bound-quartic-separators q22-after-quartic-boundary-repair cm17-explicit-level77-incidence; do
 cmp "$expected/$name.json" "$output/$name.json"
done
for name in KoblitzRound16CM KoblitzRound16Ramification KoblitzRound16Sections KoblitzRound16Separators KoblitzRound16Boundary; do
 cmp "$expected/$name.lean" "$output/$name.lean"
 cmp "$study/lean/$name.lean" "$output/$name.lean"
done
"$runner" --round16-check-identities "$round"
printf 'PASS:7 new JSON outputs and5 generated Lean modules replay byte for byte;5 source-bound IDC records verified.\n'
