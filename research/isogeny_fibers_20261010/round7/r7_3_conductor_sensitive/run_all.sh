#!/bin/sh
# Reproduces every raw record of lead R7.3.  Usage: run_all.sh <example-binary> <out-dir>
BIN=${1:?binary}; OUT=${2:?out dir}
mkdir -p "$OUT"
for d in 3 4; do for l in 5 7 11 13; do
  "$BIN" expA --ell $l --d $d --instances 8 --seed 1 --runs 3000 --out "$OUT"
done; done
for l in 5 7 11 13; do
  "$BIN" expB --ell $l --instances 10 --seed 1 --runs 600 --census-floors 3 --out "$OUT"
done
# (D, l) pairs with l split in K, l non-principal, and 3 not split in K (3 split forces 3 | N);
# the pairs 35/13, 23/13, 31/5, 31/7, 59/5, 59/7, 83/7, 83/11, 107/11, 107/13 were tried first
# and produced no instance (N never prime), see RESULTS.md.
for pair in "51 5" "51 11" "91 5" "115 7" "123 11" "187 7" "235 13" "403 11" "139 5" "139 7" "211 5" "283 7" "307 7" "331 5"; do
  set -- $pair
  "$BIN" expH --ell $2 --d $1 --max 4 --seed 1 --sizes 100,200 --out "$OUT"
done
"$BIN" summarise --dir "$OUT" --out "$OUT/summary.json"
