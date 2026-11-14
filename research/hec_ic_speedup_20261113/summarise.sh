#!/usr/bin/env bash
# Per-row summary of one run_record.sh output directory, as JSON lines.
# Layout only: means and medians of the recorded per-seed values (jq).
#
#   research/hec_ic_speedup_20261113/summarise.sh OUT_DIR > OUT_DIR/summary.jsonl
#
# Fields per (genus, p): big_ms (BigUint wall_ic_ms at theta = 1, already the
# engine's mean over its five seeds), ref_* and fast_* (native engine,
# Solver::Reference = the S1 solver and Solver::Fast = S2a; solve_ms and
# total_ms means and medians over five seeds), the derived ratios, and the
# fast solve's share of fast total time.
set -euo pipefail
out=${1:?usage: summarise.sh OUT_DIR}
big=$(grep '^JSON' "$out/biguint.txt" | sed 's/^JSON //' |
  jq -s '[.[] | select(.theta == 1)] | map({key: "\(.genus):\(.p)", value: .wall_ic_ms}) | from_entries')
grep '^JSON' "$out/native_both.txt" | sed 's/^JSON //' | jq -c -s --argjson big "$big" '
  def mean: add / length;
  def med: sort | .[length / 2 | floor];
  group_by([.genus, .p])[] |
  ([.[] | select(.solver == "reference")]) as $R |
  ([.[] | select(.solver == "fast")]) as $F |
  {genus: .[0].genus, p: .[0].p, ell: .[0].ell, m: .[0].m, seeds: ($F | length),
   big_ms: $big["\(.[0].genus):\(.[0].p)"],
   ref_solve_ms: ([$R[].solve_ms] | mean), ref_solve_med: ([$R[].solve_ms] | med),
   fast_solve_ms: ([$F[].solve_ms] | mean), fast_solve_med: ([$F[].solve_ms] | med),
   ref_total_ms: ([$R[].total_ms] | mean), fast_total_ms: ([$F[].total_ms] | mean),
   fast_total_med: ([$F[].total_ms] | med),
   solve_mulmods: ([$F[].solve_mulmods] | mean), all_correct: ([.[].correct] | all)}
  | .solve_ratio = (.ref_solve_ms / .fast_solve_ms)
  | .solve_ratio_med = (.ref_solve_med / .fast_solve_med)
  | .big_over_fast_total = (.big_ms / .fast_total_ms)
  | .fast_solve_share = (.fast_solve_ms / .fast_total_ms)'
