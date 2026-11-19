#!/bin/bash
# L4-BINWALK measurement: one ecbench session per binary Table B curve (PROTOCOL.md), audited with every run
# replayed, then paired comparisons and the §5a band. Counted GAE only (BinaryGroup does not count field_ops,
# so no bitop/v1 reading). L0.
set -u
cd "$(dirname "$0")/../.."
bin=${ECBENCH:-target/release/ecbench}
d=research/ic_tableb_binary_wwalk_20261119
for b in 15 18 21 24 27; do
  inc=ksum5; [ $b -le 18 ] && inc=ksum4
  $bin run --spec $d/specs/B$b.json --out $d/sessions/B$b --cpus none --allow-busy --wait > $d/sessions-B$b.run.log 2>&1 || echo "FAIL run B$b"
  $bin verify --dir $d/sessions/B$b --replay-all --out $d/audit-B$b.json > $d/audit-B$b.log 2>&1 || echo "FAIL verify B$b"
  for pair in $inc:ww $inc:ww-tw ww:ww-tw rho-neg:ww rho-neg:ww-tw bsgs-neg:ww-tw; do
    a=${pair%%:*}; c=${pair##*:}
    $bin compare --dir $d/sessions/B$b --a $a --b $c > $d/compare-B$b-$a-vs-$c.txt 2>&1
  done
  $bin band --dir $d/sessions/B$b > $d/band-B$b.txt 2>&1
  echo "B$b done"
done
echo ALLDONE
