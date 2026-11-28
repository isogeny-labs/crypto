#!/usr/bin/env bash
# glibc malloc vs jemalloc (LD_PRELOAD) on F4 consumers other than jv_cover, interleaved ABBA.
# Usage: consumer_screen.sh BINDIR ROUNDS OUTDIR
# Decision rule (fixed before data): a consumer gets jemalloc built in when every round favours
# jemalloc and the median paired ratio glibc/jemalloc is at least 1.05, with identical work.
BIN=$1; ROUNDS=$2; OUT=$3; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
JE=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2
NAMES=(glibc jemalloc); TIMEFORMAT='%U %S'
run() { # consumer arm round
  local c=$1 a=$2 r=$3 f="$OUT/${1}_r${3}_${2}.json" pre="" cpu
  [ "$a" = jemalloc ] && pre="LD_PRELOAD=$JE"
  case $c in
    jv_quartic) args=(--exp cprime --sizes 269 --seeds 1 --residuals 8000 --constructed 100) ;;
    jv_quintic) args=(--variant weierstrass --sizes 271 --seeds 1 --residuals 3 --constructed 2 --max-degree 24) ;;
  esac
  cpu=$( { time env $pre RAYON_NUM_THREADS=4 "$BIN/$c" "${args[@]}" --json "$f" > /dev/null 2>&1; } 2>&1 )
  jq -c --arg c "$c" --arg a "$a" --argjson r "$r" --arg cpu "$cpu" \
    '.[0] | {consumer:$c, arm:$a, round:$r, wall_ms, cpu_s: ($cpu|split(" ")|map(tonumber)|add),
             f4_muls: .f4_muls.mean, planted_found, random_decomposable, mismatches}' "$f" >> "$OUT/runs.jsonl"
}
for c in jv_quartic jv_quintic; do
  for r in $(seq 1 "$ROUNDS"); do
    for k in 0 1; do run "$c" "${NAMES[$(( (k + r) % 2 ))]}" "$r"; done
  done
done
