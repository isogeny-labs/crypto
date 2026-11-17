#!/bin/bash
# re-run L9 macdump (unchanged, L9 src/run9.py) to round 3 with dumps under $TMPDIR/l12d; 3 single-thread jobs
D=$(cd "$(dirname "$0")/.." && pwd); L9=$D/../L9-124-falls-structure/src
mkdir -p $TMPDIR/l12d
for c in "31 2" "33 0" "35 0"; do set -- $c
  (cd $L9 && L9DUMP=$TMPDIR/l12d python3 run9.py $1 A $2 3 > $D/raw/macdump_n$1_A_s$2_r3.json 2>$TMPDIR/l12d/err_$1) &
done; wait
