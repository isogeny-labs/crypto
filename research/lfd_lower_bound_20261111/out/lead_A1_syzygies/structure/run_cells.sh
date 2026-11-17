#!/usr/bin/env bash
# run syzstruct.py on the preregistered cells, sequentially, single-threaded
set -euo pipefail
cd "$(dirname "$0")"
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
for c in "15 1 3" "15 6 3" "17 1 3" "17 8 3" "19 4 4" "19 7 4"; do
  set -- $c
  python3 syzstruct.py $1 $2 $3 >> cells.jsonl 2>> cells.log
done
