#!/usr/bin/env bash
# Specs of PROTOCOL.md: study-1 curves, residual (baseline) vs residual-fused-dp (candidate), fresh seeds.
# Writes specs/ (L2, for isolab-eks) and specs-local/ (L0, Amendment 1 of study 1 applies).
set -euo pipefail
cd "$(dirname "$0")/.."
TS=20261401; MS=20261402
arm() { jq -cn --arg n "$1" --arg r "$2" --arg fb "$3" --arg t "$4" '{name:$n, role:$r, method:{id:"ic.pipeline", params:{factor_base:("prime-abscissa:size="+$fb), linalg:"incremental-gauss", oracle:"mitm-prime-blocked:m=2", targets:$t}}}'; }
spec() {
  local arms
  arms=$(jq -cn \
    --argjson a1 "$(arm res2 baseline 2 residual)" --argjson a2 "$(arm res4 baseline 4 residual)" --argjson a3 "$(arm res8 baseline 8 residual)" \
    --argjson a4 "$(arm res2dp candidate 2 residual-fused-dp)" --argjson a5 "$(arm res4dp candidate 4 residual-fused-dp)" --argjson a6 "$(arm res8dp candidate 8 residual-fused-dp)" \
    --argjson a7 "$(arm res4aa baseline 4 residual)" \
    '[{name:"rho",role:"reference",method:{id:"rho.negation"}}, $a1,$a2,$a3,$a4,$a5,$a6,$a7]')
  for iso in L2 L0; do
    dir=specs; [ $iso = L0 ] && dir=specs-local
    jq -n --arg label "fused+DP residual collector, prime curve $1" --argjson curve "$2" --argjson arms "$arms" --argjson ts $TS --argjson ms $MS --arg iso $iso '{
      schema:"ecbench.spec/v1", label:$label,
      description:"research/ic_residual_dp_20261101/PROTOCOL.md",
      workloads:{curves:[$curve], targets_per_curve:16, target_seed:$ts},
      arms:$arms,
      measurement:{rounds:3, warmup:1, order:"alternate_measured", seed:$ms, isolation_required:$iso, timeout_seconds:1800}}' > "$dir/D-$1.json"
  done
}
spec p16 '{"kind":"prime_explicit","p":53411,"a":4589,"b":186,"group_order":53117,"r":53117,"gx":43982,"gy":22687}'
spec p18 '{"kind":"prime_explicit","p":212099,"a":188885,"b":173081,"group_order":212437,"r":212437,"gx":174945,"gy":10250}'
spec p36 '{"kind":"prime_explicit","p":68719476731,"a":68719476728,"b":33,"group_order":68719022539,"r":68719022539,"gx":2,"gy":51741851039}'
spec p37 '{"kind":"prime_explicit","p":81640512137,"a":6206588639,"b":31351229805,"group_order":81640511861,"r":81640511861,"gx":3,"gy":17330771300}'
