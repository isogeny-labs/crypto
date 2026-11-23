#!/usr/bin/env bash
# Native grid: Python dumps the frozen system (curve, base, target with exhaustive consistency check),
# closure/target/release/pc_closure computes the degree. Sequential; timeout 1200 s per cell.
# dmax per cell = largest D with C(n+D, D) <= COLMAX (dense u32 rows must fit in memory); beyond it a
# cell is reported censored.
set -u
cd "$(dirname "$0")/.."
BIN=closure/target/release/pc_closure
COLMAX=${COLMAX:-45000}
cell() { # arm p m t seed target
  local tag="${1}_p${2}_m${3}_t${4}_s${5}_${6}"
  local sys="runs/systems/${tag}.json"
  local out="runs/native/${tag}.jsonl"
  [ -s "$out" ] && grep -q -E "certified|censored" "$out" && return
  [ -s "$sys" ] || python3 -I scripts/raw_closure.py --arm $1 --p $2 --m $3 --t $4 --seed $5 --target $6 --dump "$sys" --dump-only --quiet >/dev/null 2>"runs/native/${tag}.gen.err" || { echo "$(date -u +%H:%M:%S) $tag GEN-FAIL" >> runs/native/progress.txt; return; }
  local n dmax
  n=$(python3 -c "import json;print(json.load(open('$sys'))['n'])")
  dmax=$(python3 -c "
from math import comb
n=$n; D=1
while comb(n+D+1,D+1) <= $COLMAX: D+=1
print(D)")
  : > "$out"
  timeout 1200 $BIN --system "$sys" --dmax $dmax --out "$out" --quiet >/dev/null 2>"runs/native/${tag}.err"
  echo "$(date -u +%H:%M:%S) $tag n=$n dmax=$dmax exit=$? $(grep -o -E '"(certified_D|censored_D_ge)": *[0-9]+' $out | tail -1)" >> runs/native/progress.txt
}
for s in 1 2; do
  for t in 1 2 3 4 5 6; do cell K 40961 2 $t $s random; cell S 40961 2 $t $s random; cell M 40961 2 $t $s random; cell T 8191 2 $t $s random; cell C 8191 2 $t $s random; done
  for t in 1 2 3; do cell K 40961 3 $t $s random; cell S 40961 3 $t $s random; cell M 40961 3 $t $s random; cell T 8191 3 $t $s random; done
  for t in 1 2; do cell C 8191 3 $t $s random; done
done
for a in K S M; do cell $a 40961 2 3 1 planted; done; cell T 8191 2 3 1 planted; cell C 8191 2 3 1 planted
for t in 2 3 4 5; do cell T 73727 2 $t 1 random; done
cell K 40961 2 7 1 random; cell T 8191 2 7 1 random; cell C 8191 2 6 1 random
echo "$(date -u +%H:%M:%S) NATIVE GRID DONE" >> runs/native/progress.txt
