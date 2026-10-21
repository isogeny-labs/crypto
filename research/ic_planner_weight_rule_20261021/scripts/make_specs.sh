#!/usr/bin/env bash
# Generate the Phase A specs from the frozen curve list and multiplier grid
# (PROTOCOL.md). Usage: scripts/make_specs.sh  (from the study directory)
set -euo pipefail
cd "$(dirname "$0")/.."
ms=(0.5 0.71 1 1.41 2 2.83 4)
gen() { # tag n0 curve-json label-prefix target-seed meas-seed out
  local tag=$1 n0=$2 curve=$3 tseed=$4 mseed=$5 out=$6
  local arms='[{"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}}]'
  for m in "${ms[@]}"; do
    local name size role
    name=m$(printf '%03d' "$(echo "$m*100/1" | bc)")
    size=$(printf '%.0f' "$(echo "$m*$n0" | bc -l)")
    role=baseline; [ "$m" = 1 ] && role=candidate
    arms=$(jq -c --arg n "$name" --arg r "$role" --arg fb "prime-abscissa:size=$size" \
      '. + [{name:$n, role:$r, method:{id:"ic.pipeline", params:{factor_base:$fb, oracle:"mitm-prime-blocked:m=2", linalg:"incremental-gauss", targets:"walk"}}}]' <<<"$arms")
  done
  jq -n --arg tag "$tag" --argjson curve "$curve" --argjson arms "$arms" \
     --argjson ts "$tseed" --argjson ms "$mseed" '{
    schema: "ecbench.spec/v1",
    label: "ic planner weight rule, Phase A, curve \($tag): base multipliers 0.5-4 x N0",
    description: "research/ic_planner_weight_rule_20261021/PROTOCOL.md. m100 = ic-planner/1 plan; every other IC arm scales the factor base only.",
    workloads: {curves: [$curve], targets_per_curve: 8, target_seed: $ts},
    arms: $arms,
    measurement: {rounds: 3, warmup: 1, order: "alternate", seed: $ms, isolation_required: "L2", timeout_seconds: 1800}
  }' > "$out"
}
for t in r24:128 r26:175 r28:323; do
  tag=${t%%:*}; n0=${t#*:}
  curve=$(jq -c '.workloads.curves[0]' ../ic_planner_validation_20261020/specs/$tag.json)
  gen "$tag" "$n0" "$curve" 20261023 20261024 "specs/A-$tag.json"
done
