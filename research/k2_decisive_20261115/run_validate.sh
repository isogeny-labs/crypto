#!/usr/bin/env bash
# Reproduction check: the blocked engine (m4) against the original dense closure (ref, k2close.rs semantics)
# on the small cells of research/k2_flatness_20261115. Usage: run_validate.sh <k2m4r binary> <out dir>
set -euo pipefail
B=$1; O=$2; mkdir -p "$O"
cells=(
 "P 2 16 8 2 5" "P 2 20 10 2 5"
 "A 3 11 4 3 5" "A 3 12 4 3 5" "A 3 13 4 3 5" "A 3 14 5 3 5" "B 3 15 3 3 5" "C 3 4 8 2 3 5"
)
for eng in ref m4; do
  : > "$O/val_$eng.jsonl"
  for s in 1 2; do
    "$B" --engine $eng --cell P --m 2 --n 16 --k 8 --targets 2 --dmin 2 --dmax 5 --seed $s >> "$O/val_$eng.jsonl"
    "$B" --engine $eng --cell P --m 2 --n 20 --k 10 --targets 2 --dmin 2 --dmax 5 --seed $s >> "$O/val_$eng.jsonl"
    for nk in 11:4 12:4 13:4 14:5; do n=${nk%:*}; k=${nk#*:}
      "$B" --engine $eng --cell A --m 3 --n $n --k $k --targets 2 --dmin 3 --dmax 5 --maxcols 80000 --seed $s >> "$O/val_$eng.jsonl"; done
    "$B" --engine $eng --cell B --m 3 --n 15 --k 3 --targets 2 --dmin 3 --dmax 5 --maxcols 80000 --seed $s >> "$O/val_$eng.jsonl"
    for nk in 8:2 9:2; do n=${nk%:*}; k=${nk#*:}
      "$B" --engine $eng --cell C --m 4 --n $n --k $k --targets 2 --dmin 3 --dmax 5 --maxcols 80000 --seed $s >> "$O/val_$eng.jsonl"; done
  done
  "$B" --engine $eng --cell N --m 3 --n 12 --k 4 --targets 1 --dmin 3 --dmax 4 --pres random --seed 1 >> "$O/val_$eng.jsonl"
  "$B" --engine $eng --cell N --m 4 --n 8 --k 2 --targets 1 --dmin 3 --dmax 4 --pres random --seed 1 >> "$O/val_$eng.jsonl"
done
