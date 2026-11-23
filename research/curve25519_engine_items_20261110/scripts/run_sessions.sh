#!/usr/bin/env bash
# PROTOCOL.md section 3: one ecbench session per spec (one target each), then band (both
# readings: the current default, which prices table work, and --no-price-table-work) and
# verify --replay 1 per session. Usage: run_sessions.sh K  (20 or 24).
# Launched under tools/benchrec.py run <record> -- ...
set -euo pipefail
cd "$(dirname "$0")/.."
E=${ECB:-/home/user/ctarget-f1/release/ecbench}
K=$1
[ -f "data/binary-k${K}.sha256" ] && ! sha256sum -c --status "data/binary-k${K}.sha256" && { echo "binary changed"; exit 3; }
sha256sum "$E" > "data/binary-k${K}.sha256"
for spec in specs/k${K}-s*.json; do
  s=$(basename "$spec" .json)
  out=sessions/$s
  # Per-session resumability: a session whose run completed and whose audit file exists is
  # kept; anything else (an interrupted session) is deleted and rerun from scratch.
  if [ -f "data/verify-$s.json" ] && grep -q '"status": *"complete"' "$out/session.json" 2>/dev/null; then
    echo "$s kept (complete) $(date -u +%H:%M:%S)"; continue
  fi
  rm -rf "$out"
  echo "$s start $(date -u +%H:%M:%S)"
  "$E" plan --spec "$spec" > "data/plan-$s.txt" 2>&1
  "$E" run --wait --allow-busy --spec "$spec" --out "$out" > "data/run-$s.log" 2>&1
  "$E" band --dir "$out" --ranks --json "data/band-$s.json" > "data/band-$s.txt" 2>&1 || echo "$s band rc=$?"
  "$E" band --dir "$out" --ranks --no-price-table-work --json "data/band-np-$s.json" > "data/band-np-$s.txt" 2>&1 || echo "$s band-np rc=$?"
  "$E" verify --dir "$out" --replay 1 --out "data/verify-$s.json" > "data/verify-$s.txt" 2>&1 || echo "$s verify FAILED rc=$?"
  echo "$s done $(date -u +%H:%M:%S)"
done
