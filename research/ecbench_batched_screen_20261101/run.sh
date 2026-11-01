#!/usr/bin/env bash
# Runs the frozen panel of PROTOCOL.md: one ecbench session per cell, an
# exact-replay audit of every record, the one-unit table, and each arm's
# paired comparison against the baseline (and the A/A control).
set -euo pipefail
cd "$(dirname "$0")/../.."
BIN=${ECBENCH:-target/release/ecbench}
OUT=research/ecbench_batched_screen_20261101
ARMS=(ic-b1 ic-b2 ic-b4 ic-b8 ic-b16 ic-b32 ic-b128 ic-b256 ic-b64-aa)
mkdir -p "$OUT/audits" "$OUT/tables" "$OUT/comparisons"
for cell in n13a0 n17a1 n19a0 n23a0; do
  "$BIN" run --spec "$OUT/specs/$cell.json" --out "$OUT/sessions/$cell" --cpus none --allow-busy --settle 1 --quiet
  "$BIN" verify --dir "$OUT/sessions/$cell" --replay-all > "$OUT/audits/$cell.verify.txt"
  "$BIN" table --dir "$OUT/sessions/$cell" > "$OUT/tables/$cell.table.txt"
  mkdir -p "$OUT/comparisons/$cell"
  for arm in "${ARMS[@]}"; do
    "$BIN" compare --dir "$OUT/sessions/$cell" --a ic-b64 --b "$arm" --save \
      > "$OUT/comparisons/$cell/$arm.txt"
  done
done
