#!/bin/bash
B=/Volumes/SSD990-2/s1-build/s1b-20261101; cd $B; export TMPDIR=$B/tmp WDSAT_NOBF=1
P=$B/target/release/examples/wdsat_probe
for l in $(seq 3 18); do N=32; mkdir -p anf/l$l
 for c in "11 1" "13 0" "19 0" "19 1"; do set -- $c; [ $l -lt $1 ] && $P $1 $2 $l $N 777$l 1 anf/l$l >> dump_$1_$2.log 2>&1 || true; done
done
echo DUMPDONE
