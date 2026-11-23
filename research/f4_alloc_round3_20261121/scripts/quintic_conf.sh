#!/usr/bin/env bash
# jv_quintic (large F4 systems): glibc vs jemalloc default vs jemalloc without page purging
# (MALLOC_CONF dirty/muzzy decay -1), rotated order. Usage: quintic_conf.sh BINDIR ROUNDS OUTDIR
BIN=$1; ROUNDS=$2; OUT=$3; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
JE=/usr/lib/x86_64-linux-gnu/libjemalloc.so.2
ARMS=(${QARMS:-glibc je je_nodecay}); TIMEFORMAT='%U %S'
for r in $(seq 1 "$ROUNDS"); do
  for k in $(seq 0 $(( ${#ARMS[@]} - 1 ))); do
    a=${ARMS[$(( (k + r) % ${#ARMS[@]} ))]}; f="$OUT/r${r}_${a}.json"
    case $a in
      glibc) e=() ;;
      je) e=(LD_PRELOAD=$JE) ;;
      je_nodecay) e=(LD_PRELOAD=$JE MALLOC_CONF=dirty_decay_ms:-1,muzzy_decay_ms:-1) ;;
      je_nooversize) e=(LD_PRELOAD=$JE MALLOC_CONF=oversize_threshold:0) ;;
      je_both) e=(LD_PRELOAD=$JE MALLOC_CONF=oversize_threshold:0,dirty_decay_ms:-1,muzzy_decay_ms:-1) ;;
    esac
    cpu=$( { time env "${e[@]}" RAYON_NUM_THREADS=4 "$BIN/jv_quintic" --variant weierstrass --sizes 271 --seeds 1 \
      --residuals 3 --constructed 2 --max-degree 24 --json "$f" > /dev/null 2>&1; } 2>&1 )
    jq -c --arg a "$a" --argjson r "$r" --arg cpu "$cpu" '.[0] | {arm:$a, round:$r, wall_ms,
      user_s: ($cpu|split(" ")[0]|tonumber), sys_s: ($cpu|split(" ")[1]|tonumber), f4_muls: .f4_muls.mean, planted_found}' "$f" >> "$OUT/runs.jsonl"
  done
done
