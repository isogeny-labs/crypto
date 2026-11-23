#!/bin/bash
# n = 3 sweep: 4 curves per prime, 2 targets per curve, 3 rho runs per target; msolve scratch in $SCR
cd "$(dirname "$0")"
export SCR=${SCR:-/tmp/claude-0/-home-user/c4b817f7-0ad0-518c-befe-15465b1e9e2a/scratchpad/ext_field/ms}
mkdir -p "$SCR"
for p in 251 127 67 31; do
  for s in 1 2 3 4; do
    echo "python3 ic_fp3.py $p $s 2 3 runs_n3/p${p}_s${s}.json > runs_n3/p${p}_s${s}.log 2>&1"
  done
done | xargs -P 3 -I{} bash -c "{}"
