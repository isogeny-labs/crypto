#!/usr/bin/env bash
# LEAD-A-selector-counting-law: preregistered grid (research/leads_20261111/lane_A_falldegree.md, Lead A2).
# Each cell (type k n e): first 3 unsatisfiable instances; feasible at e  <=>  e_min <= e (emin solve with EMAX = e).
set -u
out=out/lead_A2/grid.jsonl
cells="R 8 17 2
R 9 22 2
R 10 26 2
R 10 27 2
R 8 18 2
R 11 23 3
R 12 28 3
T 13 25 4
R 11 22 3
R 12 26 3
R 13 25 3
R 13 25 4"
echo "$cells" | while read typ k n e; do
  found=0; seed=0
  while [ $found -lt 3 ]; do
    seed=$((seed+1))
    if [ $typ = R ]; then line=$(python3 emit_instance.py $n $seed $k); else line=$(python3 emit_instance_T.py $n $seed $k); fi
    [ "$(echo "$line" | ./emin count)" = "0" ] || continue
    found=$((found+1))
    res=$(echo "$line" | ./emin solve $e 400000 $seed 2>/dev/null)
    echo "{\"type\": \"$typ\", \"k\": $k, \"n\": $n, \"e\": $e, \"seed\": $seed, \"result\": $res}" >> $out
  done
done
echo GRID-DONE
