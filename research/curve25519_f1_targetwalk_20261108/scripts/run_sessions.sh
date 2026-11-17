#!/usr/bin/env bash
# PROTOCOL.md section 3: one ecbench session per spec (one target each), then
# band + verify --replay 1 per session. Usage: run_sessions.sh K  (20 or 24).
# Launched under tools/benchrec.py run <record> -- ...
set -euo pipefail
cd "$(dirname "$0")/.."
E=${ECB:-/home/user/ctarget-f1/release/ecbench}
K=$1
sha256sum "$E" > "data/binary-k${K}.sha256"
for spec in specs/k${K}-s*.json; do
  s=$(basename "$spec" .json)
  out=sessions/$s
  rm -rf "$out"
  "$E" plan --spec "$spec" > "data/plan-$s.txt" 2>&1
  "$E" run --wait --allow-busy --spec "$spec" --out "$out" > "data/run-$s.log" 2>&1
  "$E" band --dir "$out" --ranks --json "data/band-$s.json" > "data/band-$s.txt" 2>&1
  "$E" verify --dir "$out" --replay 1 --out "data/verify-$s.json" > "data/verify-$s.txt" 2>&1
  echo "$s done $(date -u +%H:%M:%S)"
done
