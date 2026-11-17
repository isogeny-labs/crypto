#!/usr/bin/env bash
# Copies the saved ecbench comparisons of the five sessions into figures/ratios.json
# (read by scripts/build_ic_leaderboard.py). Computes no result: ratio = cost of the
# numerator arm over the denominator arm, as ecbench compare saved it (ratio_b_over_a).
# Run compare_rho_frob.sh first for the rho-frob read-outs.
set -euo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"
cd "$HERE"
rows=()
for s in a0-n13 a1-n17 a0-n23 a1-n29 a0-n37; do
  base=$(ls sessions/$s/comparisons | sed -n 's/^ic-lazy-c2__\(ic-lazy-c[0-9]*\)\.json$/\1/p; s/^\(ic-lazy-c[0-9]*\)__ic-lazy-c2\.json$/\1/p' | head -1)
  c() { jq -c '{ratio: .ops.ratio_b_over_a, ci95: .ops.ci95, pairs: .ops.pairs}' "sessions/$s/comparisons/$1.json"; }
  rows+=("$(jq -n \
    --arg session "$s" --arg base "$base" \
    --slurpfile rec <(head -1 sessions/$s/records.jsonl) \
    --slurpfile cmp "sessions/$s/comparisons/ic-lazy-c2__$base.json" \
    --argjson frozen_over_c2 "$(c ic-lazy-c2__$base)" \
    --argjson c2_over_rho_strong "$(c rho-strong__ic-lazy-c2)" \
    --argjson frozen_over_rho_strong "$(c rho-strong__$base)" \
    --argjson c2_over_rho_frob "$(c rho-frob__ic-lazy-c2)" \
    --argjson frozen_over_rho_frob "$(c rho-frob__$base)" \
    --argjson aa "$(c rho-frob__rho-frob-aa)" \
    '{session: $session, session_id: $cmp[0].a.session_id,
      curve: $rec[0].workload.curve.slug, a: ($session|split("-n")[0]|ltrimstr("a")|tonumber),
      n: ($session|split("-n")[1]|tonumber), log2_r: ($rec[0].workload.curve.r|log/(2|log)),
      frozen_c: ($base|ltrimstr("ic-lazy-c")|tonumber),
      frozen_over_c2: $frozen_over_c2,
      c2_over_rho_strong: $c2_over_rho_strong, frozen_over_rho_strong: $frozen_over_rho_strong,
      c2_over_rho_frob: $c2_over_rho_frob, frozen_over_rho_frob: $frozen_over_rho_frob,
      rho_frob_aa: $aa}')")
done
printf '%s\n' "${rows[@]}" | jq -s '{
  schema: "crypto.lazy_columns2/v1",
  study: "research/ecbench_lazy_columns2_20261018",
  unit: "ecbench.gae, paired by target, cluster-bootstrap 95% CI",
  class: "engineering",
  note: "Copied from sessions/*/comparisons; ratio = cost of the numerator arm over the denominator arm, <1 means the numerator is cheaper. frozen_over_c2 > 1 means two columns is cheaper than the column count frozen on 2026-10-14.",
  rows: .}' > figures/ratios.json
echo wrote figures/ratios.json
