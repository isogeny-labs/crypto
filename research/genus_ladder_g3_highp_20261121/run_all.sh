#!/bin/bash
cd /home/user/crypto/research/genus_ladder_g3_highp_20261121
for P in 65537 131101 262147; do
  /tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/r2/genus_ladder run --primes $P --genera 3 --curves 9 --seeds 2 --arms cR --sge 8,3 --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 --seed-search 600 --order-method bsgs1 --rho-cap 3e9 --out run_$P.jsonl > run_$P.log 2>&1
done
echo done > all_done
