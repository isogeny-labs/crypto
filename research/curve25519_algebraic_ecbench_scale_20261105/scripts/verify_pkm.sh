#!/bin/bash
cd "$(dirname "$0")/.."
export RAYON_NUM_THREADS=1; ulimit -v 3500000
B=/home/user/frozen-bin/ecbench_scale
for sq in "k24-c3-a1 2" "k24-c0-a1 2" "k24-c0-a1 3"; do set -- $sq
  uptime >> analysis/load_verify.txt
  $B verify --dir /home/user/scratch_S2/$1 --replay-seq $2 --out analysis/verify/pkm-$1-seq$2.json >/dev/null 2>&1
  echo "$(date -u +%T) $1 seq$2 rc=$?" >> analysis/verify/pkm_replays.log
done
echo DONE >> analysis/verify/pkm_replays.log
