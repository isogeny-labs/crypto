#!/bin/bash
# saturation depth R_t(c) for c < d_F (1 never enters): m2 c=2,3 at t=2..7; c=4 at t=6,7; m3 c=4 at t=2,3; c=5 at t=3
OUT=${OUT:-/tmp/claude-0/-home-user/eb02eff3-ee5a-5113-a25c-b6cc0a65fdff/scratchpad/wave3/sat.jsonl}
mkdir -p $(dirname $OUT)
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
for i in 0 1; do for t in 2 3 4 5 6; do for c in 2 3; do python3 depth.py m2 40961 $t $i $c >> $OUT; done; done; done
for i in 0 1; do python3 depth.py m3 40961 2 $i 4 >> $OUT; python3 depth.py m3 40961 3 $i 4 >> $OUT; python3 depth.py m3 40961 3 $i 5 >> $OUT; done
for i in 0 1; do python3 depth.py m2 40961 6 $i 4 >> $OUT; done
for i in 0; do python3 depth.py m2 40961 7 $i 3 >> $OUT; python3 depth.py m2 40961 7 $i 4 >> $OUT; done
echo SWEEP_DONE >> $OUT
