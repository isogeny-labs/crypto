#!/usr/bin/env bash
# PROTOCOL.md claims C1/C2. Re-running reproduces every derived file.
set -euo pipefail
cd "$(dirname "$0")/.."
E=../../target/release/ecbench
s=E-p36
mkdir -p analysis
cmp() { local a=$1 b=$2; shift 2
  for w in "$@"; do
    if [ "$w" = 0 ]; then $E compare --dir sessions/$s --a $a --b $b --save > /dev/null
    else $E compare --dir sessions/$s --a $a --b $b --weight $w --save > /dev/null; fi
  done; }
$E band --dir sessions/$s --weights 0,0.01,0.03,0.1,0.3,1 --json analysis/band-$s.json > analysis/band-$s.txt
cmp res1eq res1eqc 0 0.1 1
cmp res2eq res2eqc 0 0.1 1
cmp res1eq res1eqaa 0
for a in res1eq res1eqc res2eq res2eqc; do cmp rho $a 0 0.1 1; done
$E verify --dir sessions/$s --replay 12 > analysis/verify-$s.txt 2>&1 || echo "verify rc=$?" >> analysis/verify-$s.txt
