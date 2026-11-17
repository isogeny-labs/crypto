#!/bin/bash
# Usage: run_ladder.sh <bits> <rep>...   (OUT=sessions BIN=ecbench)
BIN=${BIN:-/dev/shm/bins/ecbench-l1-a}
OUT=${OUT:-sessions}
cd "$(dirname "$0")"
mkdir -p "$OUT"
bits=$1; shift
for rep in "$@"; do
  d="$OUT/ext-$bits-$rep"
  [ -d "$d" ] && continue
  flock -s /dev/shm/cpu.lock "$BIN" run --spec "specs/ext-$bits.json" --out "$d" --allow-busy --lock "${LOCK:-/dev/shm/out-l1/ext.lock}" --wait > "$OUT/ext-$bits-$rep.log" 2>&1
  tail -1 "$OUT/ext-$bits-$rep.log"
done
