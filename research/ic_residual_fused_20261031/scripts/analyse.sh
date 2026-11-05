#!/usr/bin/env bash
# PROTOCOL.md "Measures". Re-running reproduces every derived file.
set -euo pipefail
cd "$(dirname "$0")/.."
E=../../target/release/ecbench
W=0,0.01,0.03,0.1,0.3,1
cmp() { local s=$1 a=$2 b=$3; shift 3
  for w in "$@"; do
    if [ "$w" = 0 ]; then $E compare --dir sessions/$s --a $a --b $b --save > /dev/null
    else $E compare --dir sessions/$s --a $a --b $b --weight $w --save > /dev/null; fi
  done; }
for c in p16 p18 p36 p37; do
  s=F-$c
  $E band --dir sessions/$s --weights $W --json analysis/band-$s.json > analysis/band-$s.txt
  for b in 2 4 8; do cmp $s res$b res${b}f 0 0.1 1; done
  cmp $s res4 res4aa 0
  for b in res2 res4 res8 res2f res4f res8f; do cmp $s rho $b 0 1; done
  $E verify --dir sessions/$s --replay 12 > analysis/verify-$s.txt 2>&1 || echo "verify $s rc=$?" >> analysis/verify-$s.txt
done
