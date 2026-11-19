#!/usr/bin/env bash
# PROTOCOL sections 3-4 (after search.sh and curves.py). usage: run.sh TUNE5 | B4 | B5 | RHO5
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
d=research/genus_ladder_scaling_20261119
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.po
cs() { python3 -c "import json;print('/'.join(map(str,json.load(open('$d/curves.json'))['$1']['$2'])))"; }
case "$1" in
  TUNE5)
    for cc in 1.0 1.4 2.0 2.8; do
      $B run --primes 2357 --genera 5 --max-h 1 --curves 2 --curve-seeds 2357=$(cs g5_p2357 tune) --targets 3 --seeds 2 \
        --arms x --cc $cc --sge 4,1.5 --sge 8,2 --threads 4 --out $d/tune5_cc$cc.jsonl > $d/tune5_cc$cc.log 2>&1
    done
    python3 $d/tune.py ;;
  B4)
    for cc in 2.0 1.4; do
      $B run --primes 60013 --genera 4 --max-h 1 --curves 3 --curve-seeds 60013=$(cs g4_p60013 confirm) --targets 6 --seeds 2 \
        --arms x --cc $cc --sge 4,1.5 --threads 4 --out $d/b4_p60013_cc$cc.jsonl > $d/b4_p60013_cc$cc.log 2>&1
    done ;;
  B5)
    read cc sge < <(python3 $d/tune.py)
    for P in 2357 4099; do
      $B run --primes $P --genera 5 --max-h 1 --curves 3 --curve-seeds $P=$(cs g5_p$P confirm) --targets 6 --seeds 2 \
        --arms x --cc 2.0 --sge 4,1.5 --threads 4 --out $d/b5_p${P}_x5.jsonl > $d/b5_p${P}_x5.log 2>&1
      if [ "$cc $sge" != "2.0 4,1.5" ]; then
        $B run --primes $P --genera 5 --max-h 1 --curves 3 --curve-seeds $P=$(cs g5_p$P confirm) --targets 6 --seeds 2 \
          --arms x --cc $cc --sge $sge --threads 4 --out $d/b5_p${P}_x5star.jsonl > $d/b5_p${P}_x5star.log 2>&1
      fi
    done ;;
  RHO5)
    c1=$(python3 -c "import json;print(json.load(open('$d/curves.json'))['g5_p2357']['confirm'][0])")
    $B run --primes 2357 --genera 5 --max-h 1 --curves 1 --curve-seeds 2357=$c1 --targets 1 --seeds 2 --arms Q --cc 2.0 \
      --threads 2 --rho-cap 2e9 --out $d/rho5_p2357.jsonl > $d/rho5_p2357.log 2>&1 ;;
esac
echo done "$1"
