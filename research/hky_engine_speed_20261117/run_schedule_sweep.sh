#!/bin/bash
# The schedule table of README section 6: multiply-adds of one closure under
# different queue orders and batch sizes (deterministic counters).
# usage: run_schedule_sweep.sh OUTDIR
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
CAND=${CAND:-$HERE/bin/pilot_candidate}
OUT=$1; mkdir -p "$OUT"
one() { # t c name flags…
  local t=$1 c=$2 name=$3; shift 3
  "$CAND" --p 2013265921 --ladder-t none --planted 0 --random 1 --hky-cmin $c --hky-cmax $c --budget 3600 \
    --hky-max-entries 6000000000 --kinds kummer --m 2 --controls tower --t-min $t --max-t $t \
    --engine hky2 "$@" --out "$OUT/t$t-c$c-$name.jsonl" 2> "$OUT/t$t-c$c-$name.log"
  printf 't=%s c=%s %-28s %s %s %s\n' $t $c "$name" \
    "$(grep -o '"muladds_last_closure":[0-9]*' "$OUT/t$t-c$c-$name.jsonl")" \
    "$(grep -o '"rows_reduced":[0-9]*' "$OUT/t$t-c$c-$name.jsonl" | tail -1)" \
    "$(grep -o 'words=[0-9]*' "$OUT/t$t-c$c-$name.log" | tail -1)"
}
for tc in "8 5" "9 5" "10 5"; do
  set -- $tc; t=$1; c=$2
  one $t $c lifo-4096-unsplit   --hky2-order lifo --hky2-batch 4096 --hky2-sym off
  for b in 4096 256 64 16; do one $t $c lifo-$b --hky2-order lifo --hky2-batch $b; done
  one $t $c lowdeg-64           --hky2-order low-degree-first --hky2-batch 64
  for b in 32 128 4096; do one $t $c sparse-$b --hky2-order sparse-first --hky2-batch $b; done
  one $t $c sparse-32-unsplit   --hky2-order sparse-first --hky2-batch 32 --hky2-sym off
  for w in 2 3 4; do one $t $c sparse-32-width$w --hky2-order sparse-first --hky2-batch 32 --hky2-sparse-width $w; done
  one $t $c lifo-64-leadonly    --hky2-order lifo --hky2-batch 64 --hky2-lead-only on
done
