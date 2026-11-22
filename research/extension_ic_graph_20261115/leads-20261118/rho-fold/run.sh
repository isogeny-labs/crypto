#!/usr/bin/env bash
# rho-fold: build the standalone Rust checker with rustc (no cargo workspace build)
# and run it on every distinct public target of the lane plus the recorded run50 rho arms.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
lane="$(cd "$here/../.." && pwd)"
out="${CARGO_TARGET_DIR:-/home/user/cargo-target-round2}/rho-fold"
mkdir -p "$out"
rustc --edition 2021 -O -o "$out/rho_fold" "$here/src/rho_fold.rs"
row() { # label file
  jq -r --arg lab "$1" '(.public // .) as $d | [$lab, $d.p, $d.l] + ($d.alpha|flatten) + ($d.g.x|flatten) + ($d.g.y|flatten) + ($d.q.x|flatten) + ($d.q.y|flatten) | map(tostring) | join(" ")' "$2"
}
{
  row run41-51+nagao-e2e "$lane/nagao-e2e/public.json"
  row run29 "$lane/local-cover-two-lp-dlp-validation-20261116-29/public.json"
  row run31 "$lane/local-cover-two-lp-rank-stop-20261116-31/public.json"
  row run33 "$lane/runpod-ic-graph-rho-portfolio-20261116-33/public.json"
  row run34 "$lane/runpod-ic-graph-rho-portfolio-20261116-34/public.json"
  row run35 "$lane/runpod-structural-rank-gate-20261116-35/public.json"
  row run37 "$lane/runpod-factor-base-balance-20261116-37/public.json"
  row run39-40 "$lane/runpod-production-quotient-20261116-39/public.json"
  row run32-p101 "$lane/runpod-rho-native-price-validation-20261116-32/public.json"
  r50="$lane/runpod-deferred-pair-20261117-50"
  for a in rho-plain-b1 rho-neg-b1 rho-plain-b16 rho-neg-b16; do
    steps=$(jq -r '.work.run.steps' "$r50/$a.raw.json")
    case $a in
      rho-plain-b1) name="Rho plain batch1";; rho-neg-b1) name="Rho negation batch1";;
      rho-plain-b16) name="Rho plain batch16";; rho-neg-b16) name="Rho negation batch16";;
    esac
    bits=$(grep -F "| $name |" "$r50/README.md" | cut -d'|' -f3 | tr -d ', ')
    echo "RUN $a $steps $bits"
  done
} > "$here/inputs.txt"
"$out/rho_fold" < "$here/inputs.txt" | tee "$here/results.txt"
