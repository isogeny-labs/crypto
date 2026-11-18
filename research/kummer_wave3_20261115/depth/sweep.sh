#!/bin/bash
# depth sweep: c = measured d_F (lab A/C tables): m2 t=2..5 -> 4, t=6 -> 5; m3 t=2 -> 5, t=3 -> 6
OUT=${OUT:-/tmp/claude-0/-home-user/eb02eff3-ee5a-5113-a25c-b6cc0a65fdff/scratchpad/wave3/depth.jsonl}
mkdir -p $(dirname $OUT)
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
for p in 40961 12289; do for i in 0 1 2; do
  for t in 2 3 4 5; do python3 depth.py m2 $p $t $i 4 >> $OUT; done
  python3 depth.py m3 $p 2 $i 5 >> $OUT
done; done
for p in 40961 12289; do for i in 0 1 2; do python3 depth.py m3 $p 3 $i 6 >> $OUT; done; done
for i in 0 1 2; do python3 depth.py m2 40961 6 $i 5 >> $OUT; done
echo SWEEP_DONE >> $OUT
