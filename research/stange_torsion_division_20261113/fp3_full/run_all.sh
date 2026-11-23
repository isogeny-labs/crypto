#!/bin/bash
# Full counted n = 3 sweep: 4 curves (seeds 1..4) per prime, 2 targets per curve, 3 rho runs per target, cap 2.
cd "$(dirname "$0")"
mkdir -p runs
for p in 4091 2039 1019 503 251 127 67 31; do
  for s in 1 2 3 4; do
    echo "python3 ic_fp3_full.py $p $s 2 3 runs/p${p}_s${s}.json 2.0 > runs/p${p}_s${s}.log 2>&1"
  done
done | xargs -P 4 -I{} bash -c "{}"
