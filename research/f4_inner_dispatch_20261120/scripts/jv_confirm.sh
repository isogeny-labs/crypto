#!/usr/bin/env bash
# Pre-registered confirmation (CONFIRM_PROTOCOL.md). Usage: jv_confirm.sh B0 B1 B2 ROUNDS OUTDIR
B0=$1; B1=$2; B2=$3; ROUNDS=$4; OUT=$5; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
NAMES=(B0 B1 B2 B1a); TIMEFORMAT='%U %S'
bin_of() { case $1 in B0) echo "$B0";; B1|B1a) echo "$B1";; B2) echo "$B2";; esac; }
for r in $(seq 1 "$ROUNDS"); do
  for k in 0 1 2 3; do
    a=${NAMES[$(( (k + r) % 4 ))]}; f="$OUT/r${r}_${a}.json"
    cpu=$( { time env -u F4_FP_MEM -u LD_PRELOAD RAYON_NUM_THREADS=4 "$(bin_of $a)" --exp dlp --sizes 53 --seeds 1 --rho-runs 0 --json "$f" > /dev/null 2>&1; } 2>&1 )
    jq -c --arg a "$a" --argjson r "$r" --arg cpu "$cpu" '.[0] | {arm:$a, round:$r, wall_ms, cpu_s: ($cpu|split(" ")|map(tonumber)|add), residuals, relations, oracle_muls, correct, s}' "$f" >> "$OUT/runs.jsonl"
  done
done
