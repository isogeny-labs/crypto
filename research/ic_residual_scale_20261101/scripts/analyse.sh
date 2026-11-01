#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
E=../../target/release/ecbench
W=0,0.01,0.03,0.1,0.3,1
cmp() { local s=$1 a=$2 b=$3; shift 3
  for w in "$@"; do
    if [ "$w" = 0 ]; then $E compare --dir sessions/$s --a $a --b $b --save > /dev/null
    else $E compare --dir sessions/$s --a $a --b $b --weight $w --save > /dev/null; fi
  done; }
for c in p36 p40 p42; do
  s=S-$c
  $E band --dir sessions/$s --weights $W --json analysis/band-$s.json > analysis/band-$s.txt
  for b in 1 4; do cmp $s res$b res${b}dp 0 0.1 1; cmp $s res$b res${b}eq 0 0.1 1; done
  cmp $s res1 res1aa 0
  for a in res1 res4 res1dp res4dp res1eq res4eq; do cmp $s rho $a 0 1; done
  $E verify --dir sessions/$s --replay 12 > analysis/verify-$s.txt 2>&1 || echo "verify $s rc=$?" >> analysis/verify-$s.txt
done
