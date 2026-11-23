#!/bin/bash
# Development smoke tests at p < 2^16 only. Instrument validation, not claim data.
# usage: BIN=/path/to/p2coset smoke/run_smoke.sh   (writes into smoke/)
set -u
BIN=${BIN:?set BIN to the p2coset binary}
D=$(cd "$(dirname "$0")" && pwd)
cd "$D"
F4() { # kind p t m C seed select tseed
  local tag="$1_p$2_t$3_m$4_$7_ts$8"
  $BIN emit --kind $1 --p $2 --t $3 --m $4 --C $5 --seed $6 --select $7 --tseed $8 --out $D/$tag.ms > $tag.emit.json || return 1
  ( ulimit -v 1572864; nice -n 10 msolve -g 2 -t 1 -v 2 -f $D/$tag.ms -o $D/$tag.msolve.out > $tag.log 2>&1 )
  echo "{\"tag\":\"$tag\",\"c_E\":$(python3 -c "import json;print(json.load(open('$tag.emit.json'))['c_E'])"),\"f4\":$($BIN parse --log $tag.log)}" >> f4_smoke.jsonl
  rm -f $D/$tag.ms $D/$tag.msolve.out
}
: > f4_smoke.jsonl
: > cosets_coverage.jsonl
for spec in "kummer 12289" "dickson 65519"; do
  set -- $spec; K=$1; P=$2
  for T in 3 4; do
    $BIN cosets --kind $K --p $P --t $T --C 300 --seed 1 >> cosets_coverage.jsonl
    for SEL in max med min; do $BIN coverage --kind $K --p $P --t $T --C 300 --seed 1 --select $SEL >> cosets_coverage.jsonl; done
  done
  for TS in 1 2 3; do
    for SEL in max med; do F4 $K $P 3 2 300 1 $SEL $TS; F4 $K $P 4 2 300 1 $SEL $TS; F4 $K $P 2 3 300 1 $SEL $TS; done
  done
  F4 $K $P 3 3 300 1 max 1; F4 $K $P 3 3 300 1 med 1
done
