#!/usr/bin/env bash
# Specs of PROTOCOL.md: per curve, rho.negation, the ic plan (ic-planner/2,
# w = 0) and the residual collector (targets=residual) at base sizes
# 4, 8, 16, 32, 64 and N0. Usage: scripts/make_specs.sh (from the study directory)
set -euo pipefail
cd "$(dirname "$0")/.."
IC=../../target/release/ic
gen() { # tag curve-json
  local tag=$1 curve=$2 p base n0 arms
  p=$($IC plan --curve "$curve" --ecbench-arm plan --json)
  base=$(jq -c '.ecbench_arm.method.params' <<<"$p")
  n0=$(jq -r '.factor_base' <<<"$base" | sed 's/.*size=//')
  arms=$(jq -cn --argjson plan "$(jq -c .ecbench_arm <<<"$p")" '[{name:"rho", role:"reference", method:{id:"rho.negation"}}, $plan]')
  for b in 4 8 16 32 64 "$n0"; do
    name=res$b; [ "$b" = "$n0" ] && name=resN0
    arms=$(jq -c --arg n "$name" --arg fb "prime-abscissa:size=$b" --argjson base "$base" \
      '. + [{name:$n, role:"baseline", method:{id:"ic.pipeline", params:($base + {factor_base:$fb, targets:"residual"})}}]' <<<"$arms")
  done
  jq -n --argjson curve "$curve" --argjson arms "$arms" --arg tag "$tag" '{
    schema: "ecbench.spec/v1",
    label: "prime residual collector (large-prime limit |D| = r) against the plan, curve \($tag)",
    description: "research/ic_prime_residual_lp_20261026/PROTOCOL.md. plan = ic plan output; resB = the same pipeline with targets=residual (ic_framework::residual) on a base of B abscissae.",
    workloads: {curves: [$curve], targets_per_curve: 8, target_seed: 20261031},
    arms: $arms,
    measurement: {rounds: 3, warmup: 1, order: "alternate_measured", seed: 20261101, isolation_required: "L2", timeout_seconds: 1800}
  }' > "specs/$tag.json"
}
gen p21 '{"kind":"prime_explicit","p":1579673,"a":49127,"b":471469,"group_order":1579297,"r":1579297,"gx":3,"gy":739631}'
gen p22 '{"kind":"prime_explicit","p":2387969,"a":1427047,"b":558169,"group_order":2390917,"r":2390917,"gx":1686673,"gy":2366352}'
gen p24 '{"kind":"prime_explicit","p":16777199,"a":16777196,"b":31,"group_order":16782299,"r":16782299,"gx":2,"gy":5412647}'
gen p26 '{"kind":"prime_explicit","p":42652151,"a":26623966,"b":14210560,"group_order":42653927,"r":42653927,"gx":40895024,"gy":26279521}'
gen p28 '{"kind":"prime_explicit","p":268435399,"a":268435396,"b":3,"group_order":268407199,"r":268407199,"gx":1,"gy":1}'
gen p32 '{"kind":"prime_explicit","p":4294967291,"a":4294967288,"b":31,"group_order":4294959973,"r":4294959973,"gx":2,"gy":2120051858}'
