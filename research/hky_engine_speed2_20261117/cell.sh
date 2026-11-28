#!/bin/bash
# One cell, draw 0: cell.sh BIN M T CMAX OUT.jsonl [pilot flags…]; prints one line per closure.
# BUDGET (s, default 3000), MAXCOLS (default 3000000), DRAWS (default 1) from the environment.
set -u
BIN=$1; M=$2; T=$3; CMAX=$4; OUT=$5; shift 5
rm -f "$OUT"
l0=$(cut -d' ' -f1 /proc/loadavg); s0=$(date +%s.%N)
"$BIN" --p 2013265921 --ladder-t none --planted ${PLANTED:-0} --random ${DRAWS:-1} --hky-cmax $CMAX --budget ${BUDGET:-3000} \
  --hky-max-cols ${MAXCOLS:-3000000} --hky-max-entries 14000000000 --kinds kummer --m $M --controls tower \
  --t-min $T --max-t $T "$@" --out "$OUT" 2> "${OUT%.jsonl}.log"
rc=$?; s1=$(date +%s.%N)
jq -c --arg l0 "$l0" --arg l1 "$(cut -d' ' -f1 /proc/loadavg)" --arg rc $rc \
  'select(has("closures")) | select(.t == '$T') | {m, t, ti: .target_index, load: [$l0, $l1], rc: $rc, cl: [.closures[] | select(.c >= '${CMIN_SHOW:-0}') | {c, cols, one, cen: .censored, muladds, rr: .rows_reduced, entries, ms: (.ms|round)}]}' "$OUT"
