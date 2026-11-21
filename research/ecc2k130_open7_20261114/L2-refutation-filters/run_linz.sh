#!/bin/bash
# Q-b2 linearisation runs (frozen cells). Usage: ./run_linz.sh BIN
B=$1; cd "$(dirname "$0")"
for c in "11 2 5" "11 2 6" "13 2 6" "13 2 7" "19 2 9" "19 2 10" "11 3 4" "13 3 4" "13 3 5" "19 3 6" "19 3 7"; do
  set -- $c
  out=raw/linz_n$1_m$2_d$3.json
  timeout 3000 prlimit --as=2000000000 $B linz $1 0 $2 $3 200 $((20261114000+$1*100+$2*10+$3)) $out 2,3,4 2> raw/linz_n$1_m$2_d$3.log
  echo "$c exit=$?"
done
