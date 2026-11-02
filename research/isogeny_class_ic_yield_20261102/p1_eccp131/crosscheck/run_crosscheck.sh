#!/usr/bin/env bash
# Engine (p1_eccp131 rust/src/engine.rs) against the repository's F4 (f4_fp; f4_prime_dynamic above 2^32).
# Usage: run_crosscheck.sh OUTDIR     env: BIN_P1 (built rust/ binary), BIN_XC (built crosscheck/ binary), P1_LIBPARI
# Towers are built with `build-tower --p` (libpari via dlopen); the crosscheck binary itself needs no PARI.
set -euo pipefail
: "${BIN_P1:?}"; : "${BIN_XC:?}"; : "${P1_LIBPARI:?}"
OUT=${1:?outdir}; mkdir -p "$OUT"
# 2^61-1, ~2^58, ~2^45 (above f4_fp's 2^32 limit), 2^32-5 (the largest prime f4_fp accepts), 2^31-1, 2^20-ish, 2^14-ish; all = 3 mod 4
for p in 2305843009213693951 288230376151711687 35184372088763 4294967291 2147483647 1048571 16363; do
  [ -f "$OUT/tower_$p.json" ] || "$BIN_P1" build-tower --p $p --tmax 4 --out "$OUT/tower_$p.json" > /dev/null
  "$BIN_XC" --p $p --tower "$OUT/tower_$p.json" --configs m2t3,m3t2 --curves 3 --null 2 --planted 2 --abscissa 2 --ladder 1 \
      --dense-every 2 --dense-maxcols 2000 --f4-budget-s 120 --out "$OUT/rows_$p.jsonl" 2> "$OUT/log_$p.txt" > "$OUT/summary_$p.json"
done
cat "$OUT"/summary_*.json
