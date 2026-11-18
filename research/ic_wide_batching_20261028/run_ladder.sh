#!/bin/bash
# Three sessions per size, alternated arm order within each session by ecbench itself.
BIN=${BIN:-/dev/shm/bins/ecbench-r4a}
OUT=${OUT:-sessions}
cd "$(dirname "$0")"
mkdir -p "$OUT"
for rep in 1 2 3; do
  for bits in 20 24 28 32; do
    d="$OUT/wide-$bits-$rep"
    [ -d "$d" ] && continue
    "$BIN" run --spec "specs/wide-$bits.json" --out "$d" --allow-busy > "$OUT/wide-$bits-$rep.log" 2>&1
    tail -1 "$OUT/wide-$bits-$rep.log"
  done
done
