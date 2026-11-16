#!/usr/bin/env bash
# PROTOCOL.md section 6, gate G3c: the conditional positive control.
# For every measured PC record, recompute the giant-step count from the planted
# scalar d and the record's own baby-table size, from the loops in
# src/cryptanalysis/ecbench/generic.rs at the frozen source:
#   bsgs.textbook  (m = baby_steps):     giant_steps = floor(d / m)
#   bsgs.negation  (m' = baby_steps - 1): giant_steps = ceil(max(d - m', 0) / (2 m' + 1))
# and report how many records match, the mean planted d/r of the PC targets,
# and the pooled S ratio textbook / negation. Usage: gate_pc.sh RECORDS.jsonl
set -euo pipefail
jq -s '
  def pred:
    .workload.planted as $d | .counters.baby_steps as $b |
    if .method.id == "bsgs.textbook" then (($d / $b) | floor)
    else (($b - 1) as $m | (2 * $m + 1) as $s | (if $d > $m then ((($d - $m) / $s) | ceil) else 0 end)) end;
  [ .[] | select((.warmup | not) and (.method.id == "bsgs.textbook" or .method.id == "bsgs.negation")) ] as $rs |
  { records: ($rs | length),
    exact: ([ $rs[] | select(.counters.giant_steps == pred) ] | length),
    mismatches: [ $rs[] | select(.counters.giant_steps != pred) | {arm, planted: .workload.planted, giant_steps: .counters.giant_steps, predicted: pred} ][0:10],
    targets: ([ $rs[] | {w: .workload.workload_id, x: (.workload.planted / .workload.curve.r)} ] | unique_by(.w) | length),
    mean_d_over_r: ([ $rs[] | {w: .workload.workload_id, x: (.workload.planted / .workload.curve.r)} ] | unique_by(.w) | map(.x) | add / length),
    pooled_s_ratio_tb_over_neg: (([ $rs[] | select(.method.id == "bsgs.textbook") | .cost.s ] | add / length)
                                / ([ $rs[] | select(.arm == "bsgs-neg") | .cost.s ] | add / length)) }' "$1"
