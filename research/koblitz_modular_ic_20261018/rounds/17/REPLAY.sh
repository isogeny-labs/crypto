#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round17-replay.XXXXXX")}
mkdir -p "$output"
for name in q8-graph-{0..3}-F2-relations q22-after-quartic-boundary-repair g16-diagonal-cover-section-bases cm16-integral-sign-separation q13-inverse-associative-saturation q14-two-product-colon q15-linear-square-saturation q16-linear-product-saturation q17-expanded-product-quadrics; do
 cp "$expected/$name.json" "$output/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in survivors integral theta repair lifts lift-proof; do
 timeout 1200 "$runner" "--round17-$command" "$output"
done
for name in q23-survivor-source-separators cm18-integral-Frobenius-lattice-controls g17-odd-cover-kernel-and-commutators q24-after-all-ordinary-cube-repairs g18-factored-odd-kernel-theta-lifts g18-Pic2-correction-square-residual; do
 cmp "$expected/$name.json" "$output/$name.json"
done
for name in KoblitzRound17Survivors KoblitzRound17Integral KoblitzRound17Theta KoblitzRound17Boundary KoblitzRound17LiftProof; do
 cmp "$expected/$name.lean" "$output/$name.lean"
 cmp "$study/lean/$name.lean" "$output/$name.lean"
done
"$runner" --round17-check-identities "$round"
printf 'PASS:6 new JSON outputs and5 generated Lean modules replay byte for byte;3 source-bound IDC records verified.\n'
