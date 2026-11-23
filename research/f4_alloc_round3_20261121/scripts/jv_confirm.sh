#!/usr/bin/env bash
# CONFIRM_PROTOCOL.md: B0 / B1 / B1a on jv_cover --exp dlp, rotated order. Usage: jv_confirm.sh B0BIN B1BIN ROUNDS OUTDIR
B0=$1; B1=$2; ROUNDS=$3; OUT=$4; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
ARMS=(B0 B1 B1a); TIMEFORMAT='%U %S'
for r in $(seq 1 "$ROUNDS"); do
  for k in 0 1 2; do
    a=${ARMS[$(( (k + r) % 3 ))]}; bin=$B1; [ $a = B0 ] && bin=$B0; f="$OUT/r${r}_${a}.json"
    cpu=$( { time env RAYON_NUM_THREADS=4 "$bin" --exp dlp --sizes 53 --seeds 1 --rho-runs 0 --json "$f" > /dev/null 2>&1; } 2>&1 )
    jq -c --arg a "$a" --argjson r "$r" --arg cpu "$cpu" '.[0] | {arm:$a, round:$r, wall_ms,
      cpu_s: ($cpu|split(" ")|map(tonumber)|add), residuals, relations, oracle_muls, correct}' "$f" >> "$OUT/runs.jsonl"
  done
done
