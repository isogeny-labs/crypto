#!/usr/bin/env bash
# analysis/summary.json: the Phase A derivation, the Phase B paired comparisons
# (ecbench compare --save, --weight) and each held-out plan's distance to rho. jq only.
set -euo pipefail
cd "$(dirname "$0")/.."
cmp() { jq -s -c --argjson w "$4" 'map(select((.pricing_weight // 0) == $w)) | first
  | {ratio: .ops.ratio_b_over_a, ci95: .ops.ci95, pairs: .ops.pairs}' sessions/$1/comparisons/$2__$3*.json; }
b=$(for t in hk19:c1 hk23:c3 hk39:c4; do tag=${t%:*}; alt=${t#*:}; s=B-$tag
  jq -n --arg tag "$tag" --arg alt "$alt" --argjson band "$(cat analysis/band-B-$tag.json)" \
    --argjson w0 "$(cmp $s plan $alt 0)" --argjson w3 "$(cmp $s plan $alt 0.3)" --argjson w1 "$(cmp $s plan $alt 1)" \
    --argjson rho "$(jq -c '{ratio: .ops.ratio_b_over_a, ci95: .ops.ci95}' sessions/$s/comparisons/rho__plan.json)" \
    '{tag: $tag, alternative: $alt, slug: $band.rows[0].curve, log2_r: $band.rows[0].log2_r, weights: $band.weights,
      mean_s: ($band.rows | map({(.arm): .mean_s}) | add),
      alt_over_plan: {"0": $w0, "0.3": $w3, "1": $w1}, plan_over_rho: $rho}'; done | jq -s -c .)
jq -n --argjson der "$(cat analysis/derivation.json)" --argjson b "$b" '{
  schema: "ic_planner_weight_rule_binkob.summary/v1", derivation: $der.rule,
  grid: {binary: $der.binary, koblitz: $der.koblitz}, koblitz_holdout: $b,
  p3: {w0_3: ([$b[] | select(.alt_over_plan["0.3"].ci95[1] < 1)] | length),
       w1: ([$b[] | select(.alt_over_plan["1"].ci95[1] < 1)] | length)}}' > analysis/summary.json
