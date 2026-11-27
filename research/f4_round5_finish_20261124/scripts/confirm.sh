#!/usr/bin/env bash
# CONFIRM_PROTOCOL.md: B1 / B1a / B0 (F4_FP_BRANCH=0), rotated. Usage: confirm.sh BIN ROUNDS OUTDIR
BIN=$1; ROUNDS=$2; OUT=$3; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
ARMS=(B1 B1a B0); TIMEFORMAT='%U %S'
for r in $(seq 1 "$ROUNDS"); do
  for k in 0 1 2; do
    a=${ARMS[$(( (k + r) % 3 ))]}; e=(); [ $a = B0 ] && e=(F4_FP_BRANCH=0); j="$OUT/r${r}_${a}.json"
    cpu=$( { time env "${e[@]}" RAYON_NUM_THREADS=4 "$BIN" --exp dlp --sizes 53 --seeds 1 --rho-runs 0 --trace --json "$j" > /dev/null 2>&1; } 2>&1 )
    jq -c --arg a "$a" --argjson r "$r" --arg cpu "$cpu" '.[0] | {arm:$a, round:$r, wall_ms, cpu_s: ($cpu|split(" ")|map(tonumber)|add), residuals, relations, oracle_muls, correct, s, replayed, trace_mismatches, branch_follows, branch_records}' "$j" >> "$OUT/runs.jsonl"
  done
done
