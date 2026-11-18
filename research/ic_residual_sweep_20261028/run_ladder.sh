#!/bin/bash
BIN=${BIN:-/dev/shm/bins/ecbench-r4a}
OUT=${OUT:-sessions}
cd "$(dirname "$0")"
mkdir -p "$OUT"
for rep in 1 2 3; do
  for bits in 20 24 28 32; do
    d="$OUT/sweep-$bits-$rep"
    [ -d "$d" ] && continue
    "$BIN" run --spec "specs/sweep-$bits.json" --out "$d" --allow-busy > "$OUT/sweep-$bits-$rep.log" 2>&1
    tail -1 "$OUT/sweep-$bits-$rep.log"
  done
done
