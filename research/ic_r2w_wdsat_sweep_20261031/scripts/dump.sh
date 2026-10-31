#!/bin/bash
P=target/release/examples/wdsat_probe; export WDSAT_NOBF=1
for l in 3 4 5 6 7; do N=48; [ $l = 7 ] && N=20; mkdir -p anf/l$l; for c in "9 0" "9 1" "11 1" "13 0" "15 0"; do set -- $c; $P $1 $2 $l $N 777$l 1 anf/l$l; done; done
