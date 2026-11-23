#!/bin/bash
# commands as run (binary: scratchpad c1/genus_ladder, sha in binary.sha256)
B=/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/c1/genus_ladder
cd /home/user/crypto/research/genus_ladder_sge_depth_20261121
case $1 in
g3a) $B run --primes 8209,16411 --genera 3 --curves 3 --seeds 2 --arms c --cc 2.0 --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600 --sge 8,3 --sge 16,3 --sge 32,3 --sge 16,10 --sge 32,10 --out g3a.jsonl > g3a.log 2>&1;;
g45) $B run --primes 1031,2053 --genera 4 --curves 3 --seeds 3 --arms c --cc 2.0 --max-h 8 --threads 4 --sge 8,3 --sge 16,3 --sge 32,3 --sge 16,10 --sge 32,10 --out g4.jsonl > g4.log 2>&1
     $B run --primes 1031 --genera 5 --curves 3 --seeds 3 --arms c --cc 2.0 --max-h 8 --threads 4 --sge 8,3 --sge 16,3 --sge 32,3 --sge 16,10 --sge 32,10 --out g5.jsonl > g5.log 2>&1
     $B run --primes 1031,2053 --genera 4 --curves 3 --seeds 3 --arms R --cc 2.0 --max-h 8 --threads 4 --out rho4.jsonl > rho4.log 2>&1
     $B run --primes 1031 --genera 5 --curves 3 --seeds 3 --arms R --cc 2.0 --max-h 8 --threads 4 --out rho5.jsonl > rho5.log 2>&1;;
g3b) $B run --primes 32771 --genera 3 --curves 3 --seeds 2 --arms c --cc 2.0 --max-h 8 --threads 4 --max-trials 2e8 --seed-search 600 --sge 8,3 --sge 16,3 --sge 32,3 --sge 16,10 --sge 32,10 --out g3b.jsonl > g3b.log 2>&1;;
esac
