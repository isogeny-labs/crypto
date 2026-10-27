#!/usr/bin/env bash
# Generate the specs of PROTOCOL.md (Phase A) or PROTOCOL_AMENDMENT_1.md (Phase B)
# from `ic plan --ecbench-arm`: the candidate arm is the plan, and every other IC
# arm is the plan with only its factor base changed.
# Usage (from the study directory): scripts/make_specs.sh A
#                                   scripts/make_specs.sh B <binary-dk> <koblitz-mult>
#   B arms: the plan and the one rule-chosen alternative per family
#   (binary: dimension d0 + <binary-dk>; Koblitz: columns max(1, round(<mult> c0))).
set -euo pipefail
cd "$(dirname "$0")/.."
IC=../../target/release/ic
phase=${1:-A}
BDK=${2:-}
KM=${3:-}
TS=20261027; MS=20261028
[ "$phase" = B ] && { TS=20261029; MS=20261030; }

# stdin: one "name|factor_base|role" per line.
spec() { # tag curve-json reference base-params-json label
  local tag=$1 curve=$2 ref=$3 base=$4 label=$5 arms
  arms=$(jq -cn --arg ref "$ref" '[{name:"rho", role:"reference", method:{id:$ref}}]')
  while IFS='|' read -r name fb role; do
    arms=$(jq -c --arg n "$name" --arg fb "$fb" --arg r "$role" --argjson base "$base" \
      '. + [{name:$n, role:$r, method:{id:"ic.pipeline", params:($base + {factor_base:$fb})}}]' <<<"$arms")
  done
  jq -n --argjson curve "$curve" --argjson arms "$arms" --argjson ts "$TS" --argjson ms "$MS" --arg label "$label" '{
    schema: "ecbench.spec/v1", label: $label,
    description: "research/ic_planner_weight_rule_binkob_20261026. The candidate arm is the ic plan (ic-planner/2, w = 0) output; every other IC arm changes the factor base only.",
    workloads: {curves: [$curve], targets_per_curve: 8, target_seed: $ts},
    arms: $arms,
    measurement: {rounds: 3, warmup: 1, order: "alternate_measured", seed: $ms, isolation_required: "L2", timeout_seconds: 1800}
  }' > "specs/$phase-$tag.json"
}

binary() { # tag n seed: Phase A grid d0-2 .. d0+2 (base x 1/4 .. x 4)
  local tag=$1 curve="{\"kind\":\"binary_random\",\"n\":$2,\"seed\":$3,\"max_cofactor\":8}" p base d0 ks
  p=$($IC plan --curve "$curve" --ecbench-arm plan --json)
  base=$(jq -c '.ecbench_arm.method.params | del(.factor_base)' <<<"$p")
  d0=$(jq -r '.ecbench_arm.method.params.factor_base' <<<"$p" | sed 's/.*dimension=//')
  if [ "$phase" = A ]; then ks="-2 -1 0 1 2"; else ks="0 $BDK"; fi
  for k in $ks; do
    role=baseline; [ "$k" = 0 ] && role=candidate
    name=d$(printf '%+d' "$k" | tr '+-' 'pm'); [ "$k" = 0 ] && name=plan
    echo "$name|binary-subspace:dimension=$((d0 + k))|$role"
  done | spec "$tag" "$curve" rho.negation "$base" \
    "ic planner weight rule (binary, Koblitz), Phase $phase, binary curve $tag: factor-base dimension"
}

koblitz() { # tag a n: Phase A grid m in {0.5 .. 4} x c0 (distinct column counts only)
  local tag=$1 curve="{\"kind\":\"koblitz\",\"a\":$2,\"n\":$3}" p base c0 ms seen=" "
  p=$($IC plan --koblitz "$2:$3" --ecbench-arm plan --json)
  base=$(jq -c '.ecbench_arm.method.params | del(.factor_base)' <<<"$p")
  c0=$(jq -r '.ecbench_arm.method.params.factor_base' <<<"$p" | sed 's/.*columns=\([0-9]*\).*/\1/')
  if [ "$phase" = A ]; then ms="1 0.5 0.71 1.41 2 2.83 4"; else ms="1 $KM"; fi
  for m in $ms; do
    c=$(python3 -c "print(max(1, round($m * $c0)))")
    case "$seen" in *" $c "*) continue;; esac
    seen="$seen$c "
    role=baseline; name=c$c; [ "$m" = 1 ] && { role=candidate; name=plan; }
    echo "$name|compact-orbit-scan:columns=$c,raw_x_cap=100000000|$role"
  done | spec "$tag" "$curve" rho.signed_frobenius "$base" \
    "ic planner weight rule (binary, Koblitz), Phase $phase, Koblitz curve $tag: orbit columns"
}

if [ "$phase" = A ]; then
  binary b21 21 827675; binary b23 23 827675; binary b25 25 827675
  koblitz k23 0 23; koblitz k31 0 31; koblitz k37 0 37
else
  [ -n "$BDK" ] && { binary hb19 19 59297; binary hb23 23 59297; binary hb25 25 59297; binary hb27 27 827675; }
  [ -n "$KM" ] && { koblitz hk19 1 19; koblitz hk23 1 23; koblitz hk39 0 39; }
fi
