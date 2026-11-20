#!/bin/bash
# Round-2 STACK + FEWCOL: one ecbench session per Table B curve (PROTOCOL.md), audited with every run replayed,
# then paired comparisons and the §5a band.  Counted cost at L0.
set -u
cd "$(dirname "$0")/../.."
bin=${ECBENCH:-target/release/ecbench}
d=research/ic_tableb_fewcol_20261118
for b in 20 22 24; do
  $bin run --spec $d/specs/P$b.json --out $d/sessions/P$b --cpus none --allow-busy --wait > $d/sessions-P$b.run.log 2>&1 || echo "FAIL run P$b"
  $bin verify --dir $d/sessions/P$b --replay-all --out $d/audit-P$b.json > $d/audit-P$b.log 2>&1 || echo "FAIL verify P$b"
  for pair in ksum5-sib:ksum5-ps ksum5-sib:tw2l tw2l:tw2l-aa rho-neg:tw2l rho-neg:ksum5-ps; do
    a=${pair%%:*}; c=${pair##*:}
    $bin compare --dir $d/sessions/P$b --a $a --b $c > $d/compare-P$b-$a-vs-$c.txt 2>&1
  done
  $bin band --dir $d/sessions/P$b > $d/band-P$b.txt 2>&1
  echo "P$b done"
done
echo ALLDONE
