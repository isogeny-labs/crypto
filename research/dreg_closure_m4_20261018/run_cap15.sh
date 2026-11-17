#!/bin/bash
# Retry of the four (13, 6, 4) unsatisfiable draws at degree 4 with the column cap raised
# above the 251 176 degree-4 columns and a 15 GB address-space cap (the host has 15.7 GB,
# nothing else running). Same binary, draws and seed as run.sh; outputs in runs-cap15/.
set -u
HERE=$(cd "$(dirname "$0")" && pwd); REPO=$(cd "$HERE/../.." && pwd)
CLOSURE=$REPO/target/release/examples/dreg_closure; f=$HERE/draws/m4-13-6.jsonl; OUT=$HERE/runs-cap15
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 RAYON_NUM_THREADS=4
for k in $(grep -v '"satisfiable"' "$f" | grep -o '"draw":[0-9]*' | cut -d: -f2); do
  o=$OUT/m4-13-6.d$k.jsonl
  if [ -s "$o" ]; then echo "$(date -u +%FT%TZ) d$k done already" >> $OUT/queue.log; continue; fi
  echo "$(date -u +%FT%TZ) d$k start" >> $OUT/queue.log; s=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 10800 "$CLOSURE" --to 4 --max-cols 300000 --draw "$k" "$f") > "$o" 2> "$OUT/m4-13-6.d$k.log"
  echo "$(date -u +%FT%TZ) d$k exit $? after $(( $(date +%s) - s )) s" >> $OUT/queue.log
done
