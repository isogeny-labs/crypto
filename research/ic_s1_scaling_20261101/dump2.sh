#!/bin/bash
B=/Volumes/SSD990-2/s1-build/s1b-20261101; cd $B; export TMPDIR=$B/tmp WDSAT_NOBF=1
P=$B/target/release/examples/wdsat_probe
for l in $(seq 3 16); do N=32; mkdir -p anf2/l$l
 for c in "9 0" "9 1" "15 0" "15 1" "17 1"; do set -- $c; [ $l -lt $1 ] && $P $1 $2 $l $N 777$l 1 anf2/l$l >> dump2_$1_$2.log 2>&1 || true; done
done
for l in $(seq 3 16); do mkdir -p bins2/first/l$l; (python3 /Volumes/SSD990-2/s1-build/wdsat_cell.py build anf2/l$l --src /Volumes/SSD990-2/s1-build/wdsat_src --out bins2/first/l$l --pad 8 --first > bins2_l$l.log 2>&1 || echo FAIL $l) & done; wait
cp -R target target2
cd tree; export CARGO_TARGET_DIR=$B/target2 WDSAT_BIN_DEFAULT=$B/bins2 WDSAT_TMP_DEFAULT=$B/tmp
cargo build --release --offline --bin ecbench > $B/build2.log 2>&1; echo "BUILD2-DONE rc=$?" >> $B/build2.log
