#!/bin/bash
cd /private/tmp/claude-501/-Volumes-SSD990-2/5bd55701-773a-411d-8f2b-d51ad35de8bf/scratchpad
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
unset KIC_LINEAR_HARVEST
B=r8src/target/release/examples/ic_improve3_02_harvest_gate
AP="--first 5000 --phase test --mode prod --arms i:off/d:env/r:r3/i2:off"
AB="--first 5000 --phase test --mode bench --arms i:off/d:env/r:r3/i2:off"
run() { for c in "$@"; do set -- $c; nice $B --n $1 --ell $2 --m $3 --targets $4 $AP; nice $B --n $1 --ell $2 --m $3 --targets $4 $AB; done; }
( run "13 4 3 12" "17 6 3 12" "19 7 3 12" "17 9 2 12" "19 10 2 12" "23 12 2 12" > r8n_a.jsonl 2> r8n_a.err ) &
( run "23 8 3 12" "29 15 2 8" "31 16 2 8" "29 10 3 8" "31 10 3 8" "37 19 2 8" > r8n_b.jsonl 2> r8n_b.err ) &
wait
echo ALLDONE > r8n.done
