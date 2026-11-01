#!/bin/bash
cd /Volumes/SSD990-2/s2-build
for c in k1n11 k0n13 k0n15; do echo "START AA_$c $(date +%T)"; ./runsess.sh AA_$c; echo "END AA_$c $(tail -2 runs/S2_AA_$c.log|tr '\n' ' ')"; done
echo AADONE
