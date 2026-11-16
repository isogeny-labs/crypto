#!/bin/bash
# Confirmation: the dp rule fixed on the held-out sweep (DPRULE=ks DPDIV=1.5,
# dp = round(log2(sqrt(2nT/6)/T) - 1.5)) on the IC comparison's own targets (INST0=0).
set -u
cd "$(dirname "$0")/../.."
bin=target/release/examples/prime_fast_j0_rho_multi
out=research/ic_e222_m221_20261106/results/pf_j0
for cell in "26 20" "26 200" "26 1000" "28 200" "28 1000" "28 3000" "30 200" "30 1000" "30 3000"; do
  set -- $cell
  DPRULE=ks DPDIV=1.5 INST0=0 $bin $out/rhotuned_$1_T$2.jsonl $1 $2 2 > /dev/null || echo "FAIL $1 $2"
done
echo ALLDONE
