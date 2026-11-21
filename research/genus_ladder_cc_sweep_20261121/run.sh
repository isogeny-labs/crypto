#!/bin/bash
# usage: run.sh fam   (g3|g4) ; one process per cc, threads 1 each, concurrent
B=/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/c2/genus_ladder
cd "$(dirname "$0")/raw"
for cc in 1.0 1.5 2.0 3.0 4.0; do
 if [ "$1" = g3 ]; then
  nohup $B run --primes 8209,16411 --genera 3 --curves 3 --seeds 2 --arms c --sge 8,3 --cc $cc --max-h 8 --threads 1 --max-trials 2e8 --seed-search 600 --out g3_cc$cc.jsonl > g3_cc$cc.log 2>&1 &
 else
  nohup $B run --primes 2053,4099 --genera 4 --curves 3 --seeds 2 --arms c --sge 8,3 --cc $cc --max-h 2 --threads 1 --max-trials 5e7 --seed-search 4000 --out g4_cc$cc.jsonl > g4_cc$cc.log 2>&1 &
 fi
done
wait
