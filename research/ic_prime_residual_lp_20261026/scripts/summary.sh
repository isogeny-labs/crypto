#!/usr/bin/env bash
# analysis/summary.json from the committed ecbench outputs (band, compare --save),
# the null runs (scripts/null.jq) and the slopes (scripts/slope.jq). jq only.
set -euo pipefail
cd "$(dirname "$0")/.."
cmp() { # session a b weight -> {ratio, ci95, pairs}
  jq -s -c --arg w "$4" 'map(select((.pricing_weight // 0) == ($w | tonumber))) | first
    | {ratio: .ops.ratio_b_over_a, ci95: .ops.ci95, pairs: .ops.pairs}' sessions/$1/comparisons/$2__$3*.json
}
rows=$(for t in p21 p22 p24 p26 p28 p32; do
  jq -n --arg tag "$t" --argjson band "$(cat analysis/band-$t.json)" \
    --argjson r0 "$(cmp $t plan res4 0)" --argjson r01 "$(cmp $t plan res4 0.1)" --argjson r1 "$(cmp $t plan res4 1)" \
    --argjson e0 "$(cmp $t plan res8 0)" --argjson e01 "$(cmp $t plan res8 0.1)" --argjson e1 "$(cmp $t plan res8 1)" \
    --argjson rp "$(jq -c '{ratio: .ops.ratio_b_over_a, ci95: .ops.ci95}' sessions/$t/comparisons/rho__plan.json)" \
    --argjson rr "$(jq -c '{ratio: .ops.ratio_b_over_a, ci95: .ops.ci95}' sessions/$t/comparisons/rho__res4.json)" '
    { tag: $tag, slug: $band.rows[0].curve, log2_r: $band.rows[0].log2_r, weights: $band.weights,
      mean_s: ($band.rows | map({(.arm): .mean_s}) | add),
      res4_over_plan: {"0": $r0, "0.1": $r01, "1": $r1},
      res8_over_plan: {"0": $e0, "0.1": $e01, "1": $e1},
      plan_over_rho: $rp, res4_over_rho: $rr }'
done | jq -s -c .)
null=$(for f in analysis/null-*.json; do jq -c --arg c "$(basename "$f" .json)" -f scripts/null.jq "$f" | jq -c --arg c "$(basename "$f" .json)" '. + {case: $c}'; done | jq -s -c .)
jq -n --argjson rows "$rows" --argjson null "$null" --argjson slope "$(jq -s -c -f scripts/slope.jq analysis/band-p*.json)" \
  '{schema: "ic_prime_residual_lp.summary/v1", rows: $rows, null: $null, slope_ln_gae_on_ln_r: $slope}' > analysis/summary.json
