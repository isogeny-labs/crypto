#!/bin/bash
# L4-SCALE: one ecbench session per size (PROTOCOL.md), every run replayed, paired comparisons, band.  L0.
set -u
cd "$(dirname "$0")/../.."
bin=${ECBENCH:-target/release/ecbench}
d=research/ic_scale_l4_20261119
for b in 28 32; do
  $bin run --spec $d/specs/P$b.json --out $d/sessions/P$b --cpus none --allow-busy --wait > $d/sessions-P$b.run.log 2>&1 || echo "FAIL run P$b"
  $bin verify --dir $d/sessions/P$b --replay-all --out $d/audit-P$b.json > $d/audit-P$b.log 2>&1 || echo "FAIL verify P$b"
  for pair in rho-neg:tw2l rho-neg:sib8-p256 tw2l:sib8-p256 sib8-p256:sib8-p256-aa; do
    a=${pair%%:*}; c=${pair##*:}
    $bin compare --dir $d/sessions/P$b --a $a --b $c --save $d/saved-P$b-$a-vs-$c.json > $d/compare-P$b-$a-vs-$c.txt 2>&1
    $bin compare --dir $d/sessions/P$b --a $a --b $c --unit bitop/v1 > $d/bitop-P$b-$a-vs-$c.txt 2>&1
  done
  $bin band --dir $d/sessions/P$b > $d/band-P$b.txt 2>&1
  echo "P$b done"
done
echo ALLDONE
