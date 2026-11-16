#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
E=../../target/release/ecbench
for c in p36 p37; do
  [ -d sessions/E-$c ] && continue
  $E run --spec specs-local/E-$c.json --out sessions/E-$c --allow-busy --quiet
done
