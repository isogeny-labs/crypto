#!/bin/bash
cd /home/user/crypto/research/genus_ladder_light_floor_20261122
B=/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/r4/genus_ladder
declare -A S=( [65537]=158/217/250/254/255/283 [131101]=41/58/61/79/84/101 [262147]=69/113/206/207/241/261 )
for P in 65537 131101 262147; do
  $B run --primes $P --genera 3 --curves 6 --curve-seeds $P=${S[$P]} --seeds 2 --arms c --sge 8,3 --excess 0.1,0.01 --excess 0.2,0.01 --excess 0.3,0.01 --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 --order-method bsgs1 --out run_$P.jsonl > run_$P.log 2>&1
  echo "$P exit $?" >> progress.log
done
echo done > all_done
