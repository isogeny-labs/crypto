#!/bin/bash
# Amendment A4: the j = 0 sessions again, on the registered curve.
set -u
cd "$(git rev-parse --show-toplevel)"
S=research/ecbench_accounting_v2_20261023
MAIN=${MAIN_BIN:?set MAIN_BIN}
R="run --wait --allow-busy --cpus auto --quiet"
./target/release/ecbench $R --spec docs/ecbench/specs/check-prime-j0-floor-v2.json --out $S/sessions/j0-v2 2>&1 | tail -1
./target/release/ecbench $R --spec $S/specs/frozen-j0.json --out $S/sessions/j0-frozen-branch 2>&1 | tail -1
$MAIN $R --spec $S/specs/frozen-j0.json --out $S/sessions/j0-frozen-main 2>&1 | tail -1
for n in j0-v2 j0-frozen-branch j0-frozen-main; do
  ./target/release/ecbench verify --dir $S/sessions/$n --replay-all --exit-code --out $S/audits/$n.audit.json > $S/audits/$n.verify.txt 2>&1; echo "$n exit=$?"
done
$MAIN verify --dir $S/sessions/j0-frozen-main --replay 0 --exit-code > $S/audits/j0-frozen-main.main-binary.verify.txt 2>&1; echo "main-binary audit of j0-frozen-main exit=$?"
