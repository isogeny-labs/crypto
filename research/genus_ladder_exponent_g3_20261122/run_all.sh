#!/bin/bash
# one prime at a time, smaller first; load recorded before/after each invocation (raw/load.log is append-only, never delete)
cd /home/user/crypto/research/genus_ladder_exponent_g3_20261122
B=/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/r10/genus_ladder
for P in 524309 1048583; do
  echo "== before $P $(date -u +%FT%TZ) $(uptime)" >> raw/load.log
  $B run --primes $P --genera 3 --curves 3 --seeds 2 --arms cR --sge 8,3 --cc 2.0 --max-h 8 --threads 1 \
     --max-trials 1e9 --rho-cap 3e10 --seed-search 600 --order-method bsgs1 --out raw/run_$P.jsonl > raw/run_$P.log 2>&1
  echo "== after $P exit=$? $(date -u +%FT%TZ) $(uptime)" >> raw/load.log
done
echo done > raw/all_done
