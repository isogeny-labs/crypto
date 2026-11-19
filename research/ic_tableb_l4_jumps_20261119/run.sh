#!/bin/bash
set -u
cd "$(dirname "$0")/../.."
bin=${ECBENCH:-target/release/ecbench}
d=research/ic_tableb_l4_jumps_20261119
sizes=${SIZES:-"20 22 24"}
for b in $sizes; do
  $bin run --spec $d/specs/P$b.json --out $d/sessions/P$b --cpus none --allow-busy --wait > $d/sessions-P$b.run.log 2>&1 || echo "FAIL run P$b"
  $bin verify --dir $d/sessions/P$b --replay-all --out $d/audit-P$b.json > $d/audit-P$b.log 2>&1 || echo "FAIL verify P$b"
  for c in tw2l-nj tw2l-sg-nj sib8-p256-j16 tw2l-sg-sib; do
    for a in sib8-p256 rho-neg; do
      $bin compare --dir $d/sessions/P$b --a $a --b $c > $d/compare-P$b-$a-vs-$c.txt 2>&1
      $bin compare --dir $d/sessions/P$b --a $a --b $c --unit bitop/v1 > $d/bitop-P$b-$a-vs-$c.txt 2>&1
    done
  done
  $bin compare --dir $d/sessions/P$b --a rho-neg --b sib8-p256 > $d/compare-P$b-rho-neg-vs-sib8-p256.txt 2>&1
  $bin compare --dir $d/sessions/P$b --a rho-neg --b sib8-p256 --unit bitop/v1 > $d/bitop-P$b-rho-neg-vs-sib8-p256.txt 2>&1
  $bin band --dir $d/sessions/P$b > $d/band-P$b.txt 2>&1
  echo "P$b done"
done
echo ALLDONE
