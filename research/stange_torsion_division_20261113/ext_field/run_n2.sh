#!/bin/bash
# n = 2 sweep: 5 curves (seeds 1..5) per prime, 3 targets per curve, 3 rho runs per target
cd "$(dirname "$0")"
for p in 251 1019 4091 16363 65519 262139; do
  for s in 1 2 3 4 5; do
    echo "python3 ic_fp2.py $p $s 3 3 runs_n2/p${p}_s${s}.json > runs_n2/p${p}_s${s}.log 2>&1"
  done
done | xargs -P 4 -I{} bash -c "{}"
