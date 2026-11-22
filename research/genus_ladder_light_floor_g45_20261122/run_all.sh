#!/bin/bash
# Frozen launch script. Three cells run concurrently (4 vCPU): g4-2053 (1 thread), g5-1031 (1 thread), g4-4099 (2 threads).
# --curve-seeds re-admits the earlier studies' curves without their seed search (2053: ~11 min, 4099: ~20 min, g5: ~36 min of
# search); the instance lines are checked field for field against the earlier studies' (G1). --curves is passed explicitly.
cd /home/user/crypto/research/genus_ladder_light_floor_g45_20261122
B=/tmp/claude-0/-home-user/399ffebe-249c-517d-8b4e-5c372f2544d0/scratchpad/r5/genus_ladder
A="--arms c --cc 2.0 --sge 8,3 --excess 0.1,0.01 --excess 0.2,0.01 --excess 0.3,0.01"
nohup $B run --primes 2053 --genera 4 --curves 3 --curve-seeds 2053=40/60/109 --seeds 2 $A --max-h 2 --threads 1 --max-trials 5e7 --seed-search 4000 --out g4_2053.jsonl > g4_2053.log 2>&1 &
nohup $B run --primes 1031 --genera 5 --curves 3 --curve-seeds 1031=3/39/42 --seeds 3 $A --max-h 8 --threads 1 --out g5_1031.jsonl > g5_1031.log 2>&1 &
nohup $B run --primes 4099 --genera 4 --curves 3 --curve-seeds 4099=72/247/418 --seeds 2 $A --max-h 2 --threads 2 --max-trials 5e7 --seed-search 4000 --out g4_4099.jsonl > g4_4099.log 2>&1 &
wait
echo done > all_done
