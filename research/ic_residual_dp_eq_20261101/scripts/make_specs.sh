#!/usr/bin/env bash
# Specs of PROTOCOL.md. Usage: scripts/make_specs.sh
set -euo pipefail
cd "$(dirname "$0")/.."
TS=20261601; MS=20261602
arm() { jq -cn --arg n "$1" --arg r "$2" --arg fb "$3" --arg t "$4" '{name:$n, role:$r, method:{id:"ic.pipeline", params:{factor_base:("prime-abscissa:size="+$fb), linalg:"incremental-gauss", oracle:"mitm-prime-blocked:m=2", targets:$t}}}'; }
spec() {
  local arms='[{"name":"rho","role":"reference","method":{"id":"rho.negation"}}]' b
  for b in 1 2 4; do
    arms=$(jq -c --argjson x "$(arm res$b baseline $b residual)" --argjson y "$(arm res${b}dp candidate $b residual-fused-dp)" --argjson z "$(arm res${b}eq candidate $b residual-fused-eq)" '. + [$x,$y,$z]' <<<"$arms")
  done
  arms=$(jq -c --argjson x "$(arm res2aa baseline 2 residual)" '. + [$x]' <<<"$arms")
  for iso in L2 L0; do
    dir=specs; [ $iso = L0 ] && dir=specs-local
    jq -n --arg label "equivariant DP residual collector, prime curve $1" --argjson curve "$2" --argjson arms "$arms" --argjson ts $TS --argjson ms $MS --arg iso $iso '{
      schema:"ecbench.spec/v1", label:$label, description:"research/ic_residual_dp_eq_20261101/PROTOCOL.md",
      workloads:{curves:[$curve], targets_per_curve:16, target_seed:$ts}, arms:$arms,
      measurement:{rounds:3, warmup:1, order:"alternate_measured", seed:$ms, isolation_required:$iso, timeout_seconds:1800}}' > "$dir/E-$1.json"
  done
}
spec p36 '{"kind":"prime_explicit","p":68719476731,"a":68719476728,"b":33,"group_order":68719022539,"r":68719022539,"gx":2,"gy":51741851039}'
spec p37 '{"kind":"prime_explicit","p":81640512137,"a":6206588639,"b":31351229805,"group_order":81640511861,"r":81640511861,"gx":3,"gy":17330771300}'
