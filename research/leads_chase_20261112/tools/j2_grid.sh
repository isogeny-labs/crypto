#!/bin/bash
# J2 grid (PROTOCOL.md): v2 ic q seed D 3.0 c 4 4 ; calib x3 per q.  usage: j2_grid.sh <v2 binary> <outdir>
set -u; V=$1; OUT=$2; mkdir -p "$OUT"
for q in 65479 262111 1048507 4194271; do
  for i in 1 2 3; do [ -s "$OUT/calib_${q}_$i.jsonl" ] || $V calib $q $i > "$OUT/calib_${q}_$i.jsonl" 2>/dev/null; done
  if [ "$q" = 4194271 ]; then CS="1.60 1.75 2.00"; else CS="1.40 1.50 1.60 1.75 2.00"; fi
  for c in $CS; do for sd in 1 2 3; do
    f="$OUT/ic_${q}_${sd}_c${c}.jsonl"; [ -s "$f" ] && continue
    $V ic $q $sd D 3.0 $c 4 4 > "$f" 2> "$f.err"; echo "$(date -u +%FT%TZ) q=$q seed=$sd c=$c rc=$?" >> "$OUT/grid.log"
  done; done
done
# A/A: repeat one unit
$V ic 65479 1 D 3.0 1.75 4 4 > "$OUT/aa_65479_1_c1.75_repeat.jsonl" 2>/dev/null
echo "done $(date -u +%FT%TZ)" >> "$OUT/grid.log"
