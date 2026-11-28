#!/usr/bin/env bash
# PROTOCOL section 4. usage: run.sh IC1 | RHO16 | RHO32 | IC2
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
d=research/genus4_cost_calibration_20261119
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.po
T="taskset -c 2"
ic() {  # $1 = pass
  for P in 16411 32771; do
    cs=$([ $P = 16411 ] && echo 373/455 || echo 355/882)
    for cc in 1.0 1.4 2.0 2.8; do
      $T $B run --primes $P --genera 4 --max-h 1 --curves 2 --curve-seeds $P=$cs --targets 1 --seeds 2 --arms x \
        --cc $cc --sge 4,1.5 --sge 8,2 --threads 1 --out $d/ic_$1_p${P}_cc$cc.jsonl > $d/ic_$1_p${P}_cc$cc.log 2>&1
    done
  done
}
rho() {  # $1 = p, $2 = curve, $3 = seed, $4 = tag
  $T $B run --primes $1 --genera 4 --max-h 1 --curves 1 --curve-seeds $1=$2 --targets 1 --seeds $3 --arms Q --cc 2.0 \
    --threads 1 --rho-cap 2e9 --out $d/rho_$4.jsonl > $d/rho_$4.log 2>&1
}
case "$1" in
  IC1) ic pass1 ;;
  IC2) ic pass2 ;;
  RHO16) rho 16411 373 3 p16411 ;;
  RHO32) rho 32771 355 1 p32771 ;;
esac
echo done "$1"
