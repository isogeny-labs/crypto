#!/bin/bash
cd /Volumes/SSD990-2/r2w-build
until grep -q ALLDONE q5.log; do sleep 5; done
run() { echo "START $1 $(date +%T)"; ./runsess.sh $1; echo "END $1 $(date +%T) $(tail -2 runs/R2W_$1.log | tr '\n' ' ')"; }
for c in k1n9 k0n9 k1n11 k0n13 k0n15; do run W8_$c; done
echo ALLDONE
