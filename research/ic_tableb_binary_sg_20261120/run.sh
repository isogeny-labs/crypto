#!/bin/bash
# L5-BINSG: stride-generator targets on the binary wwalk incumbent (PROTOCOL.md); audited with every run replayed,
# replayed, then paired comparisons and the §5a band. Counted GAE only (BinaryGroup does not count field_ops,
# so no bitop/v1 reading). L0.
set -u
cd "$(dirname "$0")/../.."
bin=${ECBENCH:-target/release/ecbench}
d=research/ic_tableb_binary_sg_20261120
for b in 15 18 21 24 27; do

  $bin run --spec $d/specs/B$b.json --out $d/sessions/B$b --cpus none --allow-busy --wait > $d/sessions-B$b.run.log 2>&1 || echo "FAIL run B$b"
  $bin verify --dir $d/sessions/B$b --replay-all --out $d/audit-B$b.json > $d/audit-B$b.log 2>&1 || echo "FAIL verify B$b"
  for pair in ww-tw:ww-tw-sg ww-tw:ww-sg rho-neg:ww-tw-sg rho-neg:ww-tw bsgs-neg:ww-tw-sg; do
    a=${pair%%:*}; c=${pair##*:}
    $bin compare --dir $d/sessions/B$b --a $a --b $c > $d/compare-B$b-$a-vs-$c.txt 2>&1
  done
  $bin band --dir $d/sessions/B$b > $d/band-B$b.txt 2>&1
  echo "B$b done"
done
echo ALLDONE
