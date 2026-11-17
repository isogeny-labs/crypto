#!/bin/bash
# dp-bit sweep for the batched folded multi-target rho (Kuhn-Struik restarts) on
# held-out instances (INST0=10: different planted targets from the IC comparison).
set -u
cd "$(dirname "$0")/../.."
bin=target/release/examples/prime_fast_j0_rho_multi
out=research/ic_e222_m221_20261106/results/pf_j0/dpsweep
for cell in "26 20" "26 200" "26 1000" "28 200" "28 1000" "28 3000" "30 200" "30 1000" "30 3000"; do
  set -- $cell
  for dp in 2 3 4 5 6 7 8 9 10 11 12; do
    INST0=10 DP=$dp $bin $out/b$1_T$2_dp$dp.jsonl $1 $2 2 > /dev/null || echo "FAIL $1 $2 dp$dp"
  done
  echo "sweep $1 $2 done"
done
echo ALLDONE
