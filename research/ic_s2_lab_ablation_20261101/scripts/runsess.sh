#!/bin/bash
# usage: runsess.sh <spec-name e.g. AA_k1n11>.  Sandbox off.  Reruns (max 3) if any wdsat-s4 error reached stderr.
B=/Volumes/SSD990-2/s2-build; S=$B/specs; export TMPDIR=$B/tmp
c=$1
for attempt in 1 2 3; do
 rm -rf $B/runs/S2_$c $B/runs/S2_${c}_tm; mkdir -p $B/runs/S2_${c}_tm
 ECB_TIME_DIR=$B/runs/S2_${c}_tm $B/target/release/ecbench run --spec $S/SPEC-$c.json --out $B/runs/S2_$c --cpus none --wait --quiet > $B/runs/S2_$c.log 2>&1
 echo "exit $? attempt $attempt" >> $B/runs/S2_$c.log
 if grep -rlq "wdsat-s4:\|No space left" $B/runs/S2_$c/exec $B/runs/S2_$c.log 2>/dev/null; then echo "error on stderr, rerun" >> $B/runs/S2_$c.log; mkdir -p $B/runs_discarded; mv $B/runs/S2_$c $B/runs_discarded/S2_${c}_err$attempt; continue; fi
 break
done
