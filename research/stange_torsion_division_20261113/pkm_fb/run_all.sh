#!/bin/bash
# Exact commands for every saved run (outputs in runs/). PKM_SCRATCH receives failing msolve inputs.
cd "$(dirname "${BASH_SOURCE[0]}")"
export PKM_SCRATCH=${PKM_SCRATCH:-/tmp/claude-0/-home-user/c4b817f7-0ad0-518c-befe-15465b1e9e2a/scratchpad/pkm_fb}
r() { out=runs/$1.json; shift; python3 pkm_tower.py "$@" > $out 2> ${out%.json}.err; }
stream_m2() {
  for k in 2 3 4 5 6 7; do r m2_k$k 2 $k --targets=8; done
  r m2_k8 2 8 --targets=6
  r m2_k9 2 9 --targets=3 --reps=1 --timeout=600
  for k in 10 11; do r m2_k${k}_D 2 $k --targets=8 --arms=D; done   # k=12 was started and stopped (38 s/run), not saved
}
stream_m3() {
  for k in 2 3; do r m3_k$k 3 $k --targets=8; done
  r m3_k4 3 4 --targets=4
  r m3_k5 3 5 --targets=2 --reps=1 --timeout=900   # stopped after target 0; runs/m3_k5.json rebuilt from runs/m3_k5.err
}
stream_fixed() {
  for k in 2 3 4 5 6 7 8; do r fixp_k$k 2 $k --targets=6 --p=1073153; done
  r fixp_k9_D 2 9 --targets=6 --p=1073153 --arms=D
  r fixp_k10_D 2 10 --targets=6 --p=1073153 --arms=D
}
stream_e2e() {
  : > runs/e2e.jsonl
  for k in 4 5 6; do for s in 1 2 3; do for a in D T2; do python3 pkm_e2e.py $k $s $a >> runs/e2e.jsonl; done; done; done
  for s in 1 2 3; do python3 pkm_e2e.py 7 $s D >> runs/e2e.jsonl; done
  python3 pkm_e2e.py 7 1 T2 >> runs/e2e.jsonl
}
stream_isog() {
  for k in 2 3 4 5 6 7; do r iso_m2_k$k 2 $k --fb=isog --arms=D,I2 --targets=6; done
  r iso_m2_k8 2 8 --fb=isog --arms=D,I2 --targets=4
  for k in 2 3; do r iso_m3_k$k 3 $k --fb=isog --arms=D,I2 --targets=6; done
  r iso_m3_k4 3 4 --fb=isog --arms=D,I2 --targets=3
}
"$@"
# then: python3 fit.py > fit.out   (fits.json, fit_table.md, pkm_fb.{png,svg,pdf})
