#!/bin/bash
# usage: runsess.sh <spec-name e.g. W10_k0n17>   (sandbox off: /usr/bin/time -l needs a sysctl).  Reruns (max 3 attempts) if a wdsat-s4 error reached stderr.
B=/Volumes/SSD990-2/s1-build/s1b-20261101; S=$B/specs
c=$1
for attempt in 1 2 3; do
 rm -rf $B/runs/T1_$c $B/runs/T1_${c}_tm; mkdir -p $B/runs/T1_${c}_tm
 ECB_TIME_DIR=$B/runs/T1_${c}_tm $B/target/release/ecbench run --spec $S/SPEC-$c.json --out $B/runs/T1_$c --cpus none --wait --lock $B/bench${LANE:-1}.lock --quiet > $B/runs/T1_$c.log 2>&1
 echo "exit $? attempt $attempt" >> $B/runs/T1_$c.log
 if grep -rlq "wdsat-s4:" $B/runs/T1_$c/exec 2>/dev/null; then echo "wdsat-s4 error on stderr, rerun" >> $B/runs/T1_$c.log; mkdir -p $B/runs_discarded; mv $B/runs/T1_$c $B/runs_discarded/T1_${c}_err$attempt; continue; fi
 break
done
