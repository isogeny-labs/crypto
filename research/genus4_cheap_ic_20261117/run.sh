#!/usr/bin/env bash
# PROTOCOL sections 2-3: IC arms only, both primes, counts deterministic.
set -euo pipefail
d=research/genus4_cheap_ic_20261117
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.cheapic
c="--genera 4 --targets 3 --seeds 2 --cc 2.0 --max-h 2 --threads 2 --arms wsfxz"
$B run --primes 8209 --curves 6 $c --curve-seeds 8209=363/393/407/457/474/519 --out $d/p8209.jsonl > $d/p8209.log 2>&1
$B run --primes 16411 --curves 3 $c --curve-seeds 16411=172/180/2640 --out $d/p16411.jsonl > $d/p16411.log 2>&1
echo done
