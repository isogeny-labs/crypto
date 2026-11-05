#!/bin/sh
# PROTOCOL.md "Train panel": runs the four train specs, audits them, and writes the
# band that select.mjs reads.  Thin ecbench orchestration.
set -eu
cd "$(dirname "$0")"
E=${ECBENCH:-../../target/release/ecbench}
LOCK=${ECBENCH_LOCK:-/tmp/crypto-bench.lock}
mkdir -p sessions audits analysis plans
for b in 20 24 28 32; do
  "$E" plan --spec "specs/train-P$b.json" > "plans/train-P$b.txt" 2>&1
  "$E" run --spec "specs/train-P$b.json" --out "sessions/train-P$b" --cpus none --allow-busy --wait --lock "$LOCK" --quiet > "sessions/train-P$b.log" 2>&1
  "$E" verify --dir "sessions/train-P$b" --replay-all --exit-code > "audits/train-P$b.txt" 2>&1
  tail -1 "audits/train-P$b.txt"
done
"$E" band --dir sessions/train-P20 sessions/train-P24 sessions/train-P28 sessions/train-P32 --weights 0,0.1,1 --json analysis/band-train.json > analysis/band-train.md
node select.mjs
