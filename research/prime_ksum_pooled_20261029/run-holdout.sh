#!/bin/sh
# PROTOCOL.md "Holdout panel": runs the four holdout specs (generated from the frozen
# selection) and audits them.  Thin ecbench orchestration.
set -eu
cd "$(dirname "$0")"
E=${ECBENCH:-../../target/release/ecbench}
LOCK=${ECBENCH_LOCK:-/tmp/crypto-bench.lock}
mkdir -p sessions audits
for b in 20 24 28 32; do
  "$E" run --spec "specs/holdout-P$b.json" --out "sessions/holdout-P$b" --cpus none --allow-busy --wait --lock "$LOCK" --quiet > "sessions/holdout-P$b.log" 2>&1
  "$E" verify --dir "sessions/holdout-P$b" --replay-all --exit-code > "audits/holdout-P$b.txt" 2>&1
  tail -1 "audits/holdout-P$b.txt"
done
