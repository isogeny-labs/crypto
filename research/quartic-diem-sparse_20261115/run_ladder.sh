#!/bin/bash
# Reproduces the measured ladder. usage: run_ladder.sh <binary> <out-dir>
# IC cells (c = 1, 2, 4 + A/A at c = 2; 3 instances per q) and the matched rho runs, one process each.
BIN=${1:?binary}; OUT=${2:?out dir}; mkdir -p "$OUT"
export QDS_GIT=$(git rev-parse HEAD)
(
  for q in 1021 4093 16381 65521 131071; do
    "$BIN" run "$OUT/ic_q$q.jsonl" $q 3 7 100,200,400 none 800 2> "$OUT/ic_q$q.log"
  done
) &
(
  "$BIN" run "$OUT/rho_q1021.jsonl" 1021 3 7 - both 0 20 2> "$OUT/rho_q1021.log"
  "$BIN" run "$OUT/rho_q4093.jsonl" 4093 3 7 - both 0 5 2> "$OUT/rho_q4093.log"
  "$BIN" run "$OUT/rho_q16381.jsonl" 16381 3 7 - both 0 1 2> "$OUT/rho_q16381.log"
) &
wait
