#!/bin/bash
# rows.tsv from the jsonl: one line per finished run (failed runs listed in failed.tsv).
cd "$(dirname "$0")"
jq -r 'select(.failed|not) | [
  .run, .k, .form, .t, .guess, .lifts_mode, (.run|test("prefix")|if . then "prefix" else "full" end),
  .targets_done, .capped, .hits, .hits_verified, .hits_exact,
  .calls_per_target, .muladds_per_target, .solving_degree_max, .solving_degree_mean,
  .solver_extra.max_rows_max, .solver_extra.max_cols_max, .solver_extra.max_nnz_max, .undecided,
  .columns, .covered_points, .covered_points_all_cosets, (.covered_projected_points // 0),
  (.covered_projected_points / .coverage_projected | round),
  (.setup_native.pkm_fiber_size // 0), (.setup_native.pkm_fiber_complete // 0),
  (.uptime | capture("load average: (?<a>[0-9.]+)").a), .f4_calls, .f4_muladds, .wall_s
] | @tsv' f4cost_scale.jsonl > rows.tsv
jq -r 'select(.failed) | [.run,.rc,.k,.form,.t,.guess,.lifts_mode]|@tsv' f4cost_scale.jsonl > failed.tsv
