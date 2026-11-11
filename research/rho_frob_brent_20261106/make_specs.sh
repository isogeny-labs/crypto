#!/usr/bin/env bash
# Writes the frozen specs (one session per curve). Usage: bash research/rho_frob_brent_20261106/make_specs.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
OUT="$ROOT/research/rho_frob_brent_20261106/specs"
mkdir -p "$OUT"
# a n targets rounds target_seed (the lazy-orbit panel's seeds + 1)
PANEL="0 13 32 3 202610181002
1 17 32 3 202610181004
0 19 32 3 202610181006
0 23 32 3 202610181008
1 29 32 3 202610181010
0 31 32 3 202610181012
0 37 32 3 202610181014
0 41 32 3 202610181016"
while read -r a n t rounds seed; do
  jq -n --argjson a "$a" --argjson n "$n" --argjson seed "$seed" --argjson t "$t" --argjson rounds "$rounds" '{
    schema:"ecbench.spec/v1",
    label:"Brent detection in the fully priced Frobenius rho: Koblitz a=\($a) n=\($n)",
    description:"Frozen in research/rho_frob_brent_20261106/PROTOCOL.md before its data. Only the walk policy differs between rho-frob and rho-frob-v2. Engineering; no security claim.",
    workloads:{curves:[{kind:"koblitz", a:$a, n:$n}], targets_per_curve:$t, target_seed:$seed, target_kind:"public"},
    arms:[
      {name:"rho-frob", role:"reference", method:{id:"rho.signed_frobenius_full"}},
      {name:"rho-frob-v2", role:"candidate", method:{id:"rho.signed_frobenius_v2_full"}},
      {name:"rho-frob-aa", role:"control", method:{id:"rho.signed_frobenius_full"}}],
    measurement:{rounds:$rounds, warmup:1, order:"alternate", seed:($seed + 1000), isolation_required:"L0", timeout_seconds:3600}}' \
    > "$OUT/rho-frob-brent-a$a-n$n.json"
  echo "$OUT/rho-frob-brent-a$a-n$n.json"
done <<<"$PANEL"
