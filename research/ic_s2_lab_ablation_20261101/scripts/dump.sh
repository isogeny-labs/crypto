#!/bin/bash
cd /Volumes/SSD990-2/s2-build; export TMPDIR=/Volumes/SSD990-2/s2-build/tmp WDSAT_NOBF=1
P=target/release/examples/wdsat_probe
for lc in "5 11 1" "8 11 1" "6 13 0" "8 13 0" "7 15 0" "10 15 0"; do set -- $lc; l=$1; n=$2; a=$3; mkdir -p anf/l$l; $P $n $a $l 40 777$l 1 anf/l$l > /dev/null 2>dump_${l}_$n.err || echo FAIL $lc; done
for l in 5 6 7 8 10; do mkdir -p bins/first/l$l; python3 wdsat_cell.py build anf/l$l --src wdsat_src --out bins/first/l$l --pad 5 --first > bins_l$l.log 2>&1 || echo FAILBUILD $l; done
ls bins/first/*
