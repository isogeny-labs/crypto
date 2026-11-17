#!/usr/bin/env bash
# Frozen grid of PROTOCOL.md. Usage: flock <cpu lock> run_grid.sh <k2close binary> <cpu lock> <out.jsonl>
set -euo pipefail
B=$1; L=$2; OUT=$3
run() { "$B" "$@" >> "$OUT"; }  # the caller holds the CPU lock for the whole grid
for s in 1 2; do run --cell P --m 2 --n 16 --k 8 --targets 2 --dmin 2 --dmax 5 --seed $s; done
for s in 1 2; do run --cell P --m 2 --n 20 --k 10 --targets 2 --dmin 2 --dmax 5 --seed $s; done
run --cell N --m 3 --n 12 --k 4 --targets 1 --dmin 3 --dmax 4 --pres random --seed 1
run --cell N --m 4 --n 8 --k 2 --targets 1 --dmin 3 --dmax 4 --pres random --seed 1
for nk in 11:4 12:4 13:4 14:5 15:5 16:5 17:6 18:6; do
  n=${nk%:*}; k=${nk#*:}
  for s in 1 2; do run --cell A --m 3 --n $n --k $k --targets 2 --dmin 3 --dmax 5 --maxcols 80000 --seed $s; done
done
for nk in 15:3 15:4 15:6; do
  n=${nk%:*}; k=${nk#*:}
  for s in 1 2; do run --cell B --m 3 --n $n --k $k --targets 2 --dmin 3 --dmax 5 --maxcols 80000 --seed $s; done
done
for nk in 8:2 9:2 10:2 11:3 12:3; do
  n=${nk%:*}; k=${nk#*:}
  for s in 1 2; do run --cell C --m 4 --n $n --k $k --targets 2 --dmin 3 --dmax 5 --maxcols 80000 --seed $s; done
done
for s in 1 2; do run --cell A-ext --m 3 --n 19 --k 6 --targets 1 --dmin 4 --dmax 5 --maxcols 80000 --seed $s; done
