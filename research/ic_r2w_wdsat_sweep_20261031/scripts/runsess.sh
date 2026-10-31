#!/bin/bash
# usage: runsess.sh <spec-name e.g. W_k0n9>   (sandbox off: /usr/bin/time -l needs sysctl).  Reruns (max 3 attempts) if any wdsat-s4 error reached stderr.
B=/Volumes/SSD990-2/r2w-build; S=/Volumes/SSD990-2/crypto-worktrees/ic-r2w-wdsat-sweep-20261031/research/ic_r2w_wdsat_sweep_20261031/specs
c=$1
for attempt in 1 2 3; do
 rm -rf $B/runs/R2W_$c $B/runs/R2W_${c}_tm; mkdir -p $B/runs/R2W_${c}_tm
 ECB_TIME_DIR=$B/runs/R2W_${c}_tm $B/target/release/ecbench run --spec $S/SPEC-$c.json --out $B/runs/R2W_$c --cpus none --wait --quiet > $B/runs/R2W_$c.log 2>&1
 echo "exit $? attempt $attempt" >> $B/runs/R2W_$c.log
 if grep -rlq "wdsat-s4:" $B/runs/R2W_$c/exec 2>/dev/null; then echo "wdsat-s4 error on stderr, rerun" >> $B/runs/R2W_$c.log; mv $B/runs/R2W_$c $B/runs_firstbuild/R2W_${c}_err$attempt; continue; fi
 break
done
