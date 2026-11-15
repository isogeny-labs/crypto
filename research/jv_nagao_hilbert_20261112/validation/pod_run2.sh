#!/usr/bin/env bash
# Second rho batch (statistically independent sample): the same frozen rhostrong command, run indices 14..27
# on the same 9 instances (126 runs), single-threaded jobs, 30 in parallel. Indices 0..13 are deterministic
# replays of the pooled reference (identical records), so new indices are the only way to draw fresh walks.
set -u
B=./jv_cover; Q=rho2; mkdir -p $Q
export B Q; export T0=$(date +%s)
job() { local p=$1 s=$2 r=$3 a=$(date +%s); local L=rhostrong_p${p}_s${s}_run${r}
  RAYON_NUM_THREADS=1 timeout 5400 $B --exp rhostrong --sizes $p --first-seed $s --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run $r --json $Q/$L.json > $Q/$L.log 2>&1
  echo "$L exit $? wall_s $(( $(date +%s) - a )) at_s $(( $(date +%s) - T0 ))" >> $Q/walls.txt; }
export -f job
for r in $(seq 14 27); do for p in 1601 1451 1301; do for s in 1 2 3; do echo "$p $s $r"; done; done; done \
  | xargs -P30 -L1 bash -c 'job $0 $1 $2'
echo "ALL DONE total_s $(( $(date +%s) - T0 ))" >> $Q/walls.txt
tar czf out2.tgz $Q
