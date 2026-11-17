#!/bin/bash
# Interleaved A/B: the reference (frozen binary) and each step of the fast
# engine, same cells, same seeds, ROUNDS rounds, arms in a rotated order.
# usage: run_ab.sh OUTDIR TMIN TMAX ROUNDS [arm…]
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
REF=${REF:-$HERE/bin/pilot_baseline}
CAND=${CAND:-$HERE/bin/pilot_candidate}
OUT=$1; TMIN=$2; TMAX=$3; ROUNDS=$4; shift 4
mkdir -p "$OUT"
cell=(--p 2013265921 --ladder-t none --planted 0 --random ${DRAWS:-1} --hky-cmax 7 --budget 7200
      --hky-max-cols 2000000 --hky-max-entries 3000000000 --kinds kummer --m 2 --controls tower
      --t-min $TMIN --max-t $TMAX)
declare -A ARM
ARM[hkyA]="REF --engine hky"
ARM[hkyB]="REF --engine hky"
ARM[L0]="CAND --engine hky2 --hky2-level 0 --hky2-order lifo --hky2-sym off"
ARM[L1]="CAND --engine hky2 --hky2-level 1 --hky2-order lifo --hky2-sym off"
ARM[L2]="CAND --engine hky2 --hky2-level 2 --hky2-order lifo --hky2-sym off"
ARM[L3]="CAND --engine hky2 --hky2-level 3 --hky2-order lifo --hky2-sym off"
ARM[L4]="CAND --engine hky2 --hky2-level 4 --hky2-order lifo --hky2-sym off"
ARM[L4sym]="CAND --engine hky2 --hky2-level 4 --hky2-order lifo --hky2-sym on"
ARM[L4sched]="CAND --engine hky2 --hky2-level 4 --hky2-order sparse-first --hky2-sym off"
ARM[hky2]="CAND --engine hky2"
arms=("$@"); [ ${#arms[@]} -gt 0 ] || arms=(hkyA hkyB L0 L1 L2 L3 L4 L4sym L4sched hky2)
n=${#arms[@]}
for r in $(seq 1 $ROUNDS); do
  for i in $(seq 0 $((n-1))); do
    a=${arms[$(( (i + r - 1) % n ))]}
    set -- ${ARM[$a]}; bin=$1; shift
    [ "$bin" = REF ] && bin=$REF || bin=$CAND
    echo "round $r arm $a before: $(uptime)" >> "$OUT/conditions.log"
    "$bin" "${cell[@]}" "$@" --out "$OUT/$a.r$r.jsonl" 2> "$OUT/$a.r$r.log"
    echo "round $r arm $a after:  $(uptime) rc=$?" >> "$OUT/conditions.log"
  done
done
echo done
