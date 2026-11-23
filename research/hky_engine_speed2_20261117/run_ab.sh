#!/bin/bash
# Interleaved A/B of v1 (hky2) and v2-default (hky3), same cell, draw 0, order v1 v2 v2 v1 each round.
# usage: run_ab.sh OUTDIR M T CMAX ROUNDS  (reports deterministic muladds and sorted wall ms per arm)
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
V1=${V1:-$HERE/../hky_engine_speed_20261117/bin/pilot_candidate}
V2=${V2:-$HERE/bin/pilot_v2}
OUT=$1; M=$2; T=$3; CMAX=$4; ROUNDS=$5
mkdir -p "$OUT"
cell=(--p 2013265921 --ladder-t none --planted 0 --random 1 --hky-cmax $CMAX --budget 7200
  --hky-max-cols 2000000 --hky-max-entries 14000000000 --kinds kummer --m $M --controls tower --t-min $T --max-t $T)
for r in $(seq 1 $ROUNDS); do
  for a in v1 v2 v2 v1; do
    bin=$V1; eng=hky2; [ $a = v2 ] && { bin=$V2; eng=hky3; }
    echo "r$r $a load $(cut -d' ' -f1-3 /proc/loadavg)" >> "$OUT/cond.log"
    "$bin" "${cell[@]}" --engine $eng --out "$OUT/$a.r$r.jsonl" 2>/dev/null
  done
done
echo "cell m=$M t=$T cmax=$CMAX  (load: $(tail -1 "$OUT/cond.log" | cut -d' ' -f3-))"
for a in v1 v2; do
  jq -rs --arg a "$a" --argjson t "$T" '[.[]|select(.t==$t)] as $x
    | ($x|map(.ms)|sort) as $m | ($x[0]) as $f
    | "\($a): muladds_total=\($f.muladds) last_closure=\($f.muladds_last_closure) | wall ms sorted \($m|map(round))"' "$OUT"/$a.r*.jsonl
done
