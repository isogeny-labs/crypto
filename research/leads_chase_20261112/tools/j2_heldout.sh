#!/bin/bash
set -u; V=$1; OUT=$2; mkdir -p "$OUT"
for i in 1 2 3; do [ -s "$OUT/calib_2097091_$i.jsonl" ] || $V calib 2097091 $i > "$OUT/calib_2097091_$i.jsonl" 2>/dev/null; done
for q in 2097091 4194271; do for sd in 4 5 6 7 8; do for c in 1.60 1.75 2.00; do
  f="$OUT/ic_${q}_${sd}_c${c}.jsonl"; [ -s "$f" ] && continue
  $V ic $q $sd D 3.0 $c 4 4 > "$f" 2> "$f.err"; echo "$(date -u +%FT%TZ) q=$q seed=$sd c=$c rc=$?" >> "$OUT/grid.log"
done; done; done
echo "done $(date -u +%FT%TZ)" >> "$OUT/grid.log"
