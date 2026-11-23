#!/bin/bash
# Frozen run script (R12). Binary: copy of target/release/genus_ladder at HEAD f49709baf2 (hash in binary.sha256).
cd /home/user/crypto/research/genus_ladder_bounded_sweep_20261122
B=/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/r12/genus_ladder
declare -A S=( [32771]=8/16/31 [65537]=64/86/157 [131101]=2/14/21 [262147]=13/19/23 )
for P in 32771 65537 131101 262147; do
  $B run --primes $P --genera 3 --curves 3 --curve-seeds $P=${S[$P]} --seeds 2 --arms c --sge 8,3 --bounded 8.5,0.1,3,1 --bounded 9,0.1,3,1 --bounded 9.5,0.1,3,1 --bounded 10,0.1,3,1 --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 --order-method bsgs1 --out run_$P.jsonl > run_$P.log 2>&1
  echo "$P exit $?" >> progress.log
done
echo done > all_done
