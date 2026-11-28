#!/bin/bash
# PC-closure certificates at c = D-1 and c = D for small arm-A random draws (nice 19, 1 BLAS thread).
cd "$(dirname "$0")/.."
export OPENBLAS_NUM_THREADS=1
for name in "$@"; do
  D=$(python3 -I -c "import json;print(json.load(open('runs/$name.res.json'))['D'])")
  m=$(python3 -I -c "import json;print(json.load(open('systems/$name.json'))['m'])")
  t=$(python3 -I -c "import json;print(json.load(open('systems/$name.json'))['t'])")
  for c in $((D-1)) $D; do
    nice -n 19 timeout 3700 python3 -I scripts/closure.py systems/$name.ms $m $t $c >> closure/certs.jsonl 2>> closure/certs.err
  done
done
