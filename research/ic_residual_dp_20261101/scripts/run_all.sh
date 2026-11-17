#!/usr/bin/env bash
# Runs the frozen local specs. Usage: scripts/run_all.sh
set -euo pipefail
cd "$(dirname "$0")/.."
E=../../target/release/ecbench
for c in p16 p18 p36 p37; do
  [ -d sessions/D-$c ] && continue
  $E run --spec specs-local/D-$c.json --out sessions/D-$c --allow-busy --quiet
done
