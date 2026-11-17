#!/bin/bash
# F2 measured run (PROTOCOL.md §4). Usage: ./run.sh BIN
# Every process on core 3 only, 2 GB address-space cap, timeout.
set -u
B=$1; cd "$(dirname "$0")"
mkdir -p raw/measure raw/linz
CELLS=("11 1 small" "13 0 small" "19 0 small" "23 0 small" "29 1 small" "37 0 small" "41 0 small" "47 1 small" "83 0 wide" "83 0 w3" "131 0 w3")
for rep in 0 1 2; do
  # seeded shuffle of the cell order for this rep (deterministic: sort by sha256 of rep+cell)
  mapfile -t ORDER < <(for c in "${CELLS[@]}"; do printf '%s\t%s\n' "$(printf '%s' "20261116-$rep-$c" | sha256sum | cut -c1-16)" "$c"; done | sort | cut -f2)
  for c in "${ORDER[@]}"; do
    set -- $c
    out=raw/measure/n$1_$3_rep$rep.json
    timeout 600 prlimit --as=2000000000 taskset -c 3 $B measure --n $1 --a $2 --impl $3 --rounds 101 --warmup 10 --seed 20261116 --rep $rep > $out 2> ${out%.json}.log
    echo "rep=$rep n=$1 impl=$3 exit=$?"
  done
done
# L2 linz cells: the seed each L2 raw file records.
L2=../../ecc2k130_open7_20261114/L2-refutation-filters/raw
for c in "11 5 200" "11 6 200" "13 6 200" "13 7 200" "19 9 200" "19 10 200"; do
  set -- $c
  seed=$(grep -o '"seed":[0-9]*' $L2/linz_n$1_m2_d$2.json | head -1 | cut -d: -f2)
  out=raw/linz/linz_n$1_m2_d$2.json
  timeout 3000 prlimit --as=2000000000 taskset -c 3 $B linz --n $1 --m 2 --d $2 --targets $3 --reps 3 --seed $seed > $out 2> ${out%.json}.log
  echo "linz n=$1 d=$2 seed=$seed exit=$?"
done
