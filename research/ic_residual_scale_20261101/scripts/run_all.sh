#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
E=../../target/release/ecbench
for c in p36 p40 p42; do
  [ -d sessions/S-$c ] && continue
  $E run --spec specs-local/S-$c.json --out sessions/S-$c --allow-busy --quiet
done
