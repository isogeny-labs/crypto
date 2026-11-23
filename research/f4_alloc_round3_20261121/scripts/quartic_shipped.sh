#!/usr/bin/env bash
# jv_quartic as shipped (jemalloc built in + f4_fp changes) vs main's binary (glibc), ABBA.
# Usage: quartic_shipped.sh BASEBIN NEWBIN ROUNDS OUTDIR
B0=$1; B1=$2; ROUNDS=$3; OUT=$4; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"; ARMS=(B0 B1); TIMEFORMAT='%U %S'
for r in $(seq 1 "$ROUNDS"); do
  for k in 0 1; do
    a=${ARMS[$(( (k + r) % 2 ))]}; bin=$B1; [ $a = B0 ] && bin=$B0; f="$OUT/r${r}_${a}.json"
    cpu=$( { time "$bin" --exp cprime --sizes 269 --seeds 1 --residuals 8000 --constructed 100 --json "$f" > /dev/null 2>&1; } 2>&1 )
    jq -c --arg a "$a" --argjson r "$r" --arg cpu "$cpu" '.[0] | {arm:$a, round:$r, wall_ms,
      cpu_s: ($cpu|split(" ")|map(tonumber)|add), f4_muls: .f4_muls.mean, planted_found, random_decomposable, mismatches}' "$f" >> "$OUT/runs.jsonl"
  done
done
