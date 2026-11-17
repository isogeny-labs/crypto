#!/usr/bin/env bash
# PROTOCOL section 2-3. Three processes; counts deterministic.
set -euo pipefail
d=research/genus4_lean_rho_20261116
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.lean
c="--genera 4 --seeds 2 --cc 2.0 --max-h 2 --threads 1 --calibrate"
s32=$(python3 -c "print('/'.join(map(str,range(1,4001))))")
$B run --primes 32771 --curves 3 --targets 1 --arms wBQ $c --threads 2 --rho-cap 3e9 --curve-seeds 32771=$s32 --out $d/p32771.jsonl > $d/p32771.log 2>&1 &
$B run --primes 8209 --curves 6 --targets 3 --arms wBQ $c --rho-cap 1.2e8 --curve-seeds 8209=363/393/407/457/474/519 --out $d/p8209.jsonl > $d/p8209.log 2>&1 &
$B run --primes 16411 --curves 3 --targets 3 --arms wBQ $c --rho-cap 5e8 --curve-seeds 16411=172/180/2640 --out $d/p16411.jsonl > $d/p16411.log 2>&1 &
wait
echo done
