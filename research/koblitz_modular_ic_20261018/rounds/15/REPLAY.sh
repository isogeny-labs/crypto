#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
output=${2:-$(mktemp -d "${TMPDIR:-/tmp}/koblitz-round15-replay.XXXXXX")}
mkdir -p "$output"
for name in g14-canonical-polarized-target q5-mixed-translation-atlases gl2b-integral-polarizations q17-expanded-product-quadrics q8-graph-{0..3}-F2-relations m13-coupled-cusp-valuations round01-results lmfdb-CM-local-polynomials lmfdb-CM-newforms-query; do
 cp "$expected/$name.json" "$output/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in theta norms supports jets cm proofs; do
 timeout 1200 "$runner" "--round15-$command" "$output"
done
for name in g15-theta-class-descent m15-new-coupled-Hecke-norms m15-target-y-multiplicativity-control m16-new-pushed-supports q18-original-boundary-first-jets cm15-one-prime-carriers; do
 cmp "$expected/$name.json" "$output/$name.json"
done
for i in 0 1 2; do
 for name in m15-function m16-function; do cmp "$expected/$name-$i.json" "$output/$name-$i.json"; done
done
for name in KoblitzRound15Jets KoblitzRound15Theta KoblitzRound15CM; do
 cmp "$expected/$name.lean" "$output/$name.lean"
 cmp "$study/lean/$name.lean" "$output/$name.lean"
done
"$runner" --round15-check-identities "$round"
printf 'PASS:12 new JSON outputs and3 source-bound generated Lean modules replay byte for byte;5 IDC records verified.\n'
