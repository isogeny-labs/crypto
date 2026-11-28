#!/bin/bash
# Round-4 L4-C1SIB: one ecbench session per Table B curve (PROTOCOL.md), audited with every run replayed,
# then paired comparisons (counted GAE and bitop/v1) and the §5a band.  Shared CPU, --cpus none --allow-busy.
set -u
cd "$(dirname "$0")/../.."
bin=${ECBENCH:-target/release/ecbench}
d=research/ic_tableb_l4_c1sib_20261119
for b in 20 22 24; do
  $bin run --spec $d/specs/P$b.json --out $d/sessions/P$b --cpus none --allow-busy --wait > $d/sessions-P$b.run.log 2>&1 || echo "FAIL run P$b"
  $bin verify --dir $d/sessions/P$b --replay-all --out $d/audit-P$b.json > $d/audit-P$b.log 2>&1 || echo "FAIL verify P$b"
  for pair in rho-neg:c1 rho-neg:c1-sib8-p256 rho-neg:c1-sib8-p256-sg rho-neg:sib8-p256 bsgs-neg:c1 bsgs-neg:c1-sib8-p256 bsgs-neg:c1-sib8-p256-sg rho-neg:bsgs-neg sib8-p256:c1 sib8-p256:c1-sib8-p256 sib8-p256:c1-sib8-p256-sg c1:c1-sib8-p256 c1-sib8-p256:c1-sib8-p256-sg; do
    a=${pair%%:*}; c=${pair##*:}
    $bin compare --dir $d/sessions/P$b --a $a --b $c > $d/compare-P$b-$a-vs-$c.txt 2>&1
    $bin compare --dir $d/sessions/P$b --a $a --b $c --unit bitop/v1 > $d/bitop-P$b-$a-vs-$c.txt 2>&1
  done
  $bin band --dir $d/sessions/P$b > $d/band-P$b.txt 2>&1
  echo "P$b done"
done
echo ALLDONE
