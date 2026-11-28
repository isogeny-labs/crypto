#!/usr/bin/env bash
# jv_cover dlp A/B across F4_FP_INNER policies. Usage: jv_inner.sh BIN ROUNDS OUTDIR
BIN=$1; ROUNDS=$2; OUT=$3; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
NAMES=(never nested-build nested)
TIMEFORMAT='%U %S'
for r in $(seq 1 "$ROUNDS"); do
  for k in 0 1 2; do
    a=${NAMES[$(( (k + r) % 3 ))]}
    f="$OUT/r${r}_${a}.json"
    cpu=$( { time env -u F4_FP_MEM F4_FP_INNER=$a RAYON_NUM_THREADS=4 "$BIN" --exp dlp --sizes 53 --seeds 1 --rho-runs 0 --json "$f" > /dev/null 2>&1; } 2>&1 )
    jq -c --arg a "$a" --argjson r "$r" --arg cpu "$cpu" '.[0] | {arm:$a, round:$r, wall_ms, cpu_s: ($cpu|split(" ")|map(tonumber)|add), residuals, relations, oracle_muls, correct}' "$f" >> "$OUT/runs.jsonl"
  done
done
