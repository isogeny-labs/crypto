#!/usr/bin/env bash
# E11 validation: run examples/hky_window.rs on committed unsatisfiable draws
# whose dense d_F and per-degree closure dims are recorded (e4, e6, e7, e8).
# Usage (repo root): POLICIES=w1,w2,supp bash research/fall_degree_bounds_20261008/e11/run_validate.sh [name-filter]
# Writes e11/validate/<name>.jsonl (skips files that already exist).
set -euo pipefail
S=research/fall_degree_bounds_20261008
B=${HKY_WINDOW:-target/release/examples/hky_window}
O=${OUT:-$S/e11/validate}
P=${POLICIES:-w1,w2,supp}
F=${1:-}
mkdir -p "$O"
FS=research/dreg_fixed_surplus_20260923/runs
M4=research/dreg_m4_chain_20261006/runs
# name | ladder inputs | m
jobs=(
  "m3_5-2|$FS/cell-5-2-7.jsonl|3"
  "m3_7-2|$FS/cell-7-2-7.jsonl|3"
  "m3_7-3|$FS/cell-7-3-7.jsonl|3"
  "x_m3_9-3|$FS/cell-9-3-7.jsonl|3"
  "x_m3_11-4|$FS/cell-11-4-6.jsonl|3"
  "x_m3_13-4|$FS/cell-13-4-6.jsonl|3"
  "A_m3_13-5|$S/e6/inputs_13-5.jsonl|3"
  "A_m3_15-5|$S/e6/inputs_15-5.jsonl|3"
  "x_m4_5-2|$M4/cell-5-2-8.jsonl|4"
  "x_m4_7-2|$M4/cell-7-2-7.u*.jsonl|4"
  "x_m4_9-3|$M4/cell-9-3-6.u*.jsonl|4"
  "B_3-18-6|$S/e6/B_3-18-6.jsonl|3"
  "B_5-10-2|$S/e6/B_5-10-2.rerun.jsonl|5"
  "B_4-13-4|$S/e6/B_4-13-4.par.jsonl|4"
)
for f in $S/e8/E8_*.jsonl $S/e7/E7_*.jsonl; do
  b=$(basename "$f" .jsonl)
  jobs+=("$b|$f|0")
done
for j in "${jobs[@]}"; do
  IFS='|' read -r name inp m <<<"$j"
  [ -n "$F" ] && [[ "$name" != *$F* ]] && continue
  out=$O/$name.jsonl
  [ -s "$out" ] && continue
  tmp=$(mktemp)
  # shellcheck disable=SC2086
  cat $inp >"$tmp"
  echo "== $name ($(date -u +%H:%M:%S))" >&2
  "$B" ladder "$tmp" --m "${m/#0/3}" --policies "$P" >"$out.part"
  mv "$out.part" "$out"
  rm -f "$tmp"
done
# Join with the dense references (native: examples/fall_degree_e11_tools.rs).
# The committed summaries were made from the repo root with (LC_ALL=C glob order):
#   T compare --json $S/e11/validation_summary.json $S/e11/validate/*.jsonl >$S/e11/validation_summary.txt
#   T derived $S/e11/validate/*.jsonl >$S/e11/derived.txt
#   T diff-rerun $S/e11/validate $S/e11/validate_final >$S/e11/diff_rerun.txt
# Here the summary of this run's output directory goes to stderr only.
T=${E11_TOOLS:-target/release/examples/fall_degree_e11_tools}
if [ -z "$F" ]; then
  "$T" compare "$O"/*.jsonl >&2
fi
