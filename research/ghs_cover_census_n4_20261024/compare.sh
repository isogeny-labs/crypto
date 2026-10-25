#!/bin/sh
# Reproduction checks of PROTOCOL.md (R1 and extras). Usage: sh compare.sh   (from the study directory)
# Timing fields (setup_s, wall_s) are removed before the byte comparison; nothing else is.
O=results/run-20261024; C=../ghs_cover_census_n5_20261013/results/run-20261014T0050Z
strip() { sed -E 's/,"setup_s":[0-9.]+//; s/,"wall_s":[0-9.]+//' "$1"; }
for pair in "R1-n3-q5 R1-n3-q5 R1 n=3 q=5" "R1-n3-q7 R1-n3-q7 R1 n=3 q=7" "R1b-n3-q3 pilot-n3-q3-cosets extra n=3 q=3 vs committed pilot" "R1c-n5-q3 D-n5-q3-cosets extra n=5 q=3 vs committed panel D"; do
  set -- $pair; a=$1; b=$2; shift 2
  [ -s $O/$a.jsonl ] || { echo "MISSING $* (not run)"; continue; }
  strip $C/$b.jsonl > $O/.ref.stripped; strip $O/$a.jsonl > $O/.new.stripped
  if cmp -s $O/.ref.stripped $O/.new.stripped; then echo "PASS $* ($(wc -l < $O/.new.stripped | tr -d ' ') records: field, curve, group, mr, derived_checks, totals)"; else echo "FAIL $*"; diff $O/.ref.stripped $O/.new.stripped | head -3; fi
done
rm -f $O/.ref.stripped $O/.new.stripped
for m in cosets cocycles; do jq -c 'select(.rec=="curve") | del(.mismatch)' $O/R4-n4-q3-$m.jsonl > $O/.r4-$m.txt; done
if cmp -s $O/.r4-cosets.txt $O/.r4-cocycles.txt; then echo "PASS R4 n=4 q=3 cosets vs cocycles ($(wc -l < $O/.r4-cosets.txt | tr -d ' ') curve records)"; else echo "FAIL R4"; fi
rm -f $O/.r4-*.txt
