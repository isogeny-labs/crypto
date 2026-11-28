#!/usr/bin/env bash
# jv_cover dlp: glibc malloc vs jemalloc (LD_PRELOAD). Usage: jv_alloc.sh BIN ROUNDS OUTDIR
BIN=$1; ROUNDS=$2; OUT=$3; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
NAMES=(glibc jemalloc); TIMEFORMAT='%U %S'
for r in $(seq 1 "$ROUNDS"); do
  for k in 0 1; do
    a=${NAMES[$(( (k + r) % 2 ))]}; f="$OUT/r${r}_${a}.json"
    pre=""; [ $a = jemalloc ] && pre="LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2"
    cpu=$( { time env -u F4_FP_MEM $pre RAYON_NUM_THREADS=4 "$BIN" --exp dlp --sizes 53 --seeds 1 --rho-runs 0 --json "$f" > /dev/null 2>&1; } 2>&1 )
    jq -c --arg a "$a" --argjson r "$r" --arg cpu "$cpu" '.[0] | {arm:$a, round:$r, wall_ms, cpu_s: ($cpu|split(" ")|map(tonumber)|add), residuals, relations, oracle_muls, correct}' "$f" >> "$OUT/runs.jsonl"
  done
done
