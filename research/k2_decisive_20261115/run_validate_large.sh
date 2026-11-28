#!/usr/bin/env bash
# Larger reproduction rows (m4 engine only; compared with the original raw/grid.jsonl rows of the same seeds,
# which is deterministic) and the one speed calibration. Usage: run_validate_large.sh <binary> <out.jsonl>
set -euo pipefail
B=$1; OUT=$2; : > "$OUT"
for nk in 16:5 17:6 18:6; do n=${nk%:*}; k=${nk#*:}
  "$B" --engine m4 --cell A --m 3 --n $n --k $k --targets 1 --dmin 4 --dmax 4 --maxcols 80000 --seed 1 >> "$OUT"; done
"$B" --engine m4 --cell C --m 4 --n 11 --k 3 --targets 1 --dmin 4 --dmax 4 --maxcols 80000 --seed 1 >> "$OUT"
