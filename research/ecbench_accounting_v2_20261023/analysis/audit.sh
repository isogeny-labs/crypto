#!/bin/bash
# Audit every session with exact replays of every deterministic run (branch binary); receipts beside the study.
set -u
cd "$(git rev-parse --show-toplevel)"
S=research/ecbench_accounting_v2_20261023
for d in $S/sessions/*/; do
  n=$(basename $d)
  ./target/release/ecbench verify --dir $d --replay-all --exit-code --out $S/audits/$n.audit.json > $S/audits/$n.verify.txt 2>&1
  echo "$n exit=$?"
done
