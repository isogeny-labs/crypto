#!/bin/bash
cd /Volumes/SSD990-2/r2w-build
until grep -q ALLDONE q9.log; do sleep 5; done
run() { echo "START $1 $(date +%T)"; ./runsess.sh $1; echo "END $1 $(date +%T) $(tail -2 runs/R2W_$1.log | tr '\n' ' ')"; }
run W12_k0n13
echo ALLDONE
