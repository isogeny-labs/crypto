#!/bin/bash
# one prime at a time, smallest first; load recorded before/after each invocation
cd /home/user/crypto/research/genus_ladder_wallclock_vs_rho_20261122
B=/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/r9/genus_ladder
declare -A SEEDS=([65537]=64/86/157/158/217/250 [131101]=2/14/21/41/58/61 [262147]=13/19/23/69/113/206)
for P in 65537 131101 262147; do
  echo "== before $P $(date -u +%FT%TZ) $(uptime)" >> raw/load.log
  $B run --primes $P --genera 3 --curves 6 --seeds 2 --arms cR --sge 8,3 --cc 2.0 --max-h 8 --threads 1 \
     --max-trials 4e8 --rho-cap 3e9 --order-method bsgs1 --curve-seeds $P=${SEEDS[$P]} --out raw/run_$P.jsonl > raw/run_$P.log 2>&1
  echo "== after $P exit=$? $(date -u +%FT%TZ) $(uptime)" >> raw/load.log
done
echo done > raw/all_done
