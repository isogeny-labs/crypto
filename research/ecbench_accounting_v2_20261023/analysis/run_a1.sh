#!/bin/bash
# Amendment A1: frozen ic.large_prime on both binaries.
set -u
cd "$(git rev-parse --show-toplevel)"
S=research/ecbench_accounting_v2_20261023
MAIN=${MAIN_BIN:?set MAIN_BIN}
./target/release/ecbench run --wait --allow-busy --cpus auto --quiet --spec $S/specs/frozen-large-prime.json --out $S/sessions/frozen-large-prime-branch 2>&1 | tail -1
$MAIN run --wait --allow-busy --cpus auto --quiet --spec $S/specs/frozen-large-prime.json --out $S/sessions/frozen-large-prime-main 2>&1 | tail -1
for n in frozen-large-prime-branch frozen-large-prime-main; do
  ./target/release/ecbench verify --dir $S/sessions/$n --replay-all --exit-code --out $S/audits/$n.audit.json > $S/audits/$n.verify.txt 2>&1; echo "$n exit=$?"
done
