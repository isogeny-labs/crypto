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
if [ "${1:-A}" = A ]; then
  for t in r24:128 r26:175 r28:323; do
    tag=${t%%:*}; n0=${t#*:}
    curve=$(jq -c '.workloads.curves[0]' ../ic_planner_validation_20261020/specs/$tag.json)
    gen "$tag" "$n0" "$curve" 20261023 20261024 "specs/A-$tag.json"
  done
  exit 0
fi
# Phase B (PROTOCOL_AMENDMENT_1.md): rho-neg, v1 = N0, v2-m141 = round(1.41 N0).
genb() { # tag n0 curve-json
  local tag=$1 n0=$2 curve=$3 arms='[{"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}}]'
  for a in v1:1:candidate v2-m141:1.41:baseline; do
    IFS=: read -r name m role <<<"$a"
    size=$(printf '%.0f' "$(echo "$m*$n0" | bc -l)")
    arms=$(jq -c --arg n "$name" --arg r "$role" --arg fb "prime-abscissa:size=$size" \
      '. + [{name:$n, role:$r, method:{id:"ic.pipeline", params:{factor_base:$fb, oracle:"mitm-prime-blocked:m=2", linalg:"incremental-gauss", targets:"walk"}}}]' <<<"$arms")
  done
  jq -n --arg tag "$tag" --argjson curve "$curve" --argjson arms "$arms" '{
    schema: "ecbench.spec/v1",
    label: "ic planner weight rule, Phase B (held out), curve \($tag): N0 against round(1.41 N0)",
    description: "research/ic_planner_weight_rule_20261021/PROTOCOL_AMENDMENT_1.md. v1 = ic-planner/1 plan; v2-m141 = the Phase A rule at w = 1.",
    workloads: {curves: [$curve], targets_per_curve: 8, target_seed: 20261025},
    arms: $arms,
    measurement: {rounds: 3, warmup: 1, order: "alternate", seed: 20261026, isolation_required: "L2", timeout_seconds: 1800}
  }' > "specs/B-$tag.json"
}
genb h21 58 '{"kind":"prime_explicit","p":1579673,"a":49127,"b":471469,"group_order":1579297,"r":1579297,"gx":3,"gy":739631}'
genb h22 67 '{"kind":"prime_explicit","p":2387969,"a":1427047,"b":558169,"group_order":2390917,"r":2390917,"gx":1686673,"gy":2366352}'
genb h24 111 '{"kind":"prime_explicit","p":10940417,"a":10128206,"b":867458,"group_order":10937713,"r":10937713,"gx":3240781,"gy":414610}'
genb h29 342 '{"kind":"prime_explicit","p":320092037,"a":254576120,"b":276414759,"group_order":320091991,"r":320091991,"gx":1,"gy":23879176}'
