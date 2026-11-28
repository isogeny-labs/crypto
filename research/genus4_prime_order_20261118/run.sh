#!/usr/bin/env bash
# PROTOCOL sections 3-4 (after search.sh and curves.py). usage: run.sh A | B16 | B32 | Q
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
d=research/genus4_prime_order_20261118
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.po
c="--genera 4 --threads 4 --seeds 2"
cs() { python3 -c "import json;print('/'.join(map(str,json.load(open('$d/curves.json'))['$1']['chosen'])))"; }
case "$1" in
  A)
    for cc in 1.4 2.0 2.8; do
      $B run --primes 16411 --curves 2 --curve-seeds 16411=172/180 --targets 3 --max-h 2 --arms x --cc $cc $c \
        --sge 4,1.5 --sge 8,2 --sge 16,3 --sge 32,4 --out $d/a_cc$cc.jsonl > $d/a_cc$cc.log 2>&1
    done
    python3 $d/tune.py ;;
  B16|B32)
    P=$([ "$1" = B16 ] && echo 16411 || echo 32771)
    read cc sge < <(python3 $d/tune.py)
    $B run --primes $P --curves 5 --curve-seeds $P=$(cs $P) --targets 6 --max-h 1 --arms x --cc 2.0 $c --sge 4,1.5 \
      --out $d/b_p${P}_x.jsonl > $d/b_p${P}_x.log 2>&1
    if [ "$cc $sge" != "2.0 4,1.5" ]; then
      $B run --primes $P --curves 5 --curve-seeds $P=$(cs $P) --targets 6 --max-h 1 --arms x --cc $cc $c --sge $sge \
        --out $d/b_p${P}_xstar.jsonl > $d/b_p${P}_xstar.log 2>&1
    fi ;;
  Q)
    $B run --primes 16411 --curves 5 --curve-seeds 16411=$(cs 16411) --targets 1 --seeds 1 --max-h 1 --arms Q --cc 2.0 \
      --genera 4 --threads 4 --rho-cap 5e8 --out $d/b_p16411_q.jsonl > $d/b_p16411_q.log 2>&1 ;;
esac
echo done "$1"
