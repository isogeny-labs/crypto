#!/bin/bash
cd /Volumes/SSD990-2/s2-build
until grep -q AADONE batch1.log; do sleep 5; done
for c in k1n11 k0n13 k0n15; do for s in A_alloff A_splitlow A_noinh A_memo0 A_nolin A_chainlayout A_occmult WB WA; do
 echo "START ${s}_$c $(date +%T)"; ./runsess.sh ${s}_$c; echo "END ${s}_$c $(tail -2 runs/S2_${s}_$c.log|tr '\n' ' ')"; done; done
echo ALLDONE
