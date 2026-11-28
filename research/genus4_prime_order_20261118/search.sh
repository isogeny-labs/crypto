#!/usr/bin/env bash
# PROTOCOL section 2: prime-order curve search (instance rows only), 4 processes by seed residue.
# usage: search.sh P N   (e.g. 16411 2000)
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
d=research/genus4_prime_order_20261118
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.po
P=$1; N=$2; LO=${3:-1}
for i in 0 1 2 3; do
  s=$(python3 -c "print('/'.join(str(x) for x in range($LO, $N + 1) if x % 4 == $i))")
  $B run --primes $P --genera 4 --max-h 1 --curves 100000 --targets 1 --seeds 1 --arms n --threads 1 \
    --curve-seeds $P=$s --out $d/search_p${P}_${LO}_${i}.jsonl > $d/search_p${P}_${LO}_${i}.log 2>&1 &
done
wait
echo done
