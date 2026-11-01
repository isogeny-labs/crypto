#!/bin/bash
cd /Volumes/SSD990-2/s1-build/s1b-20261101; export TMPDIR=$PWD/tmp LANE=${LANE:-1}
run() { echo "START $1 $(date +%T)"; ./runsess.sh $1; echo "END $1 $(date +%T) $(tail -2 runs/T1_$1.log | tr '\n' ' ')"; }
for s in "$@"; do run $s; done
echo ALLDONE
