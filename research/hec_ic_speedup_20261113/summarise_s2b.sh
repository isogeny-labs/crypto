#!/usr/bin/env bash
# Per-row summary of one run_s2b.sh output directory, as JSON lines (jq; layout
# of recorded values only).  Pairs are (repeat, seed): 25 per row.
#   ratio_med       median over pairs of solve_ms(fast) / solve_ms(dense)   (E6)
#   dense_share_med median over dense runs of solve_ms / total_ms           (E7)
# plus means and medians of solve and total times, and medians of
# solve_mulmods and solve_row_ops for both solvers, and the dense arm's
# per-phase medians.
set -euo pipefail
out=${1:?usage: summarise_s2b.sh OUT_DIR}
for f in "$out"/rep*.txt; do
  rep=$(basename "$f" .txt)
  grep '^JSON' "$f" | sed 's/^JSON //' | jq -c --arg rep "$rep" '. + {rep: $rep}'
done | jq -c -s '
  def mean: add / length;
  def med: sort | if length % 2 == 1 then .[length / 2 | floor]
                  else (.[length / 2 - 1] + .[length / 2]) / 2 end;
  group_by([.genus, .p])[] |
  ([.[] | select(.solver == "fast")]) as $F |
  ([.[] | select(.solver == "dense")]) as $D |
  ([group_by([.rep, .seed])[] |
    (map(select(.solver == "fast"))[0].solve_ms) / (map(select(.solver == "dense"))[0].solve_ms)]) as $ratios |
  {genus: .[0].genus, p: .[0].p, ell: .[0].ell, m: .[0].m, pairs: ($ratios | length),
   ratio_med: ($ratios | med), ratio_min: ($ratios | min), ratio_max: ($ratios | max),
   fast_solve_med: ([$F[].solve_ms] | med), dense_solve_med: ([$D[].solve_ms] | med),
   fast_solve_mean: ([$F[].solve_ms] | mean), dense_solve_mean: ([$D[].solve_ms] | mean),
   fast_total_med: ([$F[].total_ms] | med), dense_total_med: ([$D[].total_ms] | med),
   fast_share_med: ([$F[] | .solve_ms / .total_ms] | med),
   dense_share_med: ([$D[] | .solve_ms / .total_ms] | med),
   dense_fb_med: ([$D[].fb_ms] | med), dense_walk_med: ([$D[].walk_ms] | med),
   dense_oracle_med: ([$D[].oracle_ms] | med), dense_row_med: ([$D[].row_ms] | med),
   fast_mulmods_med: ([$F[].solve_mulmods] | med), dense_mulmods_med: ([$D[].solve_mulmods] | med),
   fast_rowops_med: ([$F[].solve_row_ops] | med), dense_rowops_med: ([$D[].solve_row_ops] | med),
   all_correct: ([.[].correct] | all)}'
