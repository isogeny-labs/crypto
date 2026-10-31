#!/bin/bash
cd /Volumes/SSD990-2/r2w-build
run() { echo "START $1 $(date +%T)"; ./runsess.sh $1; echo "END $1 $(date +%T) $(tail -2 runs/R2W_$1.log | tr '\n' ' ')"; }
for s in W9_k0n15 W10_k0n15 W11_k0n15; do run $s; done
echo ALLDONE
