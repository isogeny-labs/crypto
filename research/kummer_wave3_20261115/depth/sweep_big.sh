#!/bin/bash
OUT=${OUT:-/tmp/claude-0/-home-user/eb02eff3-ee5a-5113-a25c-b6cc0a65fdff/scratchpad/wave3/big.jsonl}
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
for t in 8 9 10; do python3 depth.py m2 40961 $t 0 4 >> $OUT; done
python3 depth.py m2 40961 8 0 5 >> $OUT
python3 depth.py m2 40961 10 0 5 >> $OUT
echo SWEEP_DONE >> $OUT
