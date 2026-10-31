#!/bin/bash
cd /Volumes/SSD990-2/r2w-build
run() { echo "START $1 $(date +%T)"; ./runsess.sh $1; echo "END $1 $(date +%T) $(tail -2 runs/R2W_$1.log | tr '\n' ' ')"; }
for c in k1n9 k0n9 k1n11 k0n13 k0n15; do run W3_$c; done
run L3_k0n13
for p in W W6 W7; do for c in k1n9 k0n9 k1n11 k0n13 k0n15; do run ${p}_$c; done; done
echo ALLDONE
