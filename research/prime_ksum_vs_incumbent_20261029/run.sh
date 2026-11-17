#!/bin/sh
# PROTOCOL.md sections 2-4, thin orchestration only (every figure is native `ecbench`).
# Runs the four frozen specs, one session per size, then the replay-all audit.
# Run from this directory; ECBENCH defaults to the repository's release build.
# The registered panel used the binary hashed in BINARY.sha256.
set -eu
cd "$(dirname "$0")"
E=${ECBENCH:-../../target/release/ecbench}
LOCK=${ECBENCH_LOCK:-/tmp/crypto-bench.lock}
mkdir -p sessions audits
for b in 20 24 28 32; do
  "$E" run --spec "specs/P$b.json" --out "sessions/P$b" --cpus none --allow-busy --wait --lock "$LOCK" --quiet > "sessions/P$b.log" 2>&1
  "$E" verify --dir "sessions/P$b" --replay-all --exit-code > "audits/P$b.txt" 2>&1
  tail -1 "audits/P$b.txt"
done
