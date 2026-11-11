#!/bin/bash
# Round 2 smoke: UNPLANTED targets at p < 2^16 only. Instrument validation, not claim data.
# usage: BIN=/path/to/p2coset smoke/round2/run_round2.sh   (writes into smoke/round2/)
set -u
BIN=${BIN:?set BIN to the p2coset binary}
D=$(cd "$(dirname "$0")" && pwd)
cd "$D"
TMP=${TMPDIR:-/tmp}
F4() { # kind p t m C seed select tseed
  local tag="$1_p$2_t$3_m$4_$7_ts$8"
  $BIN emit --kind $1 --p $2 --t $3 --m $4 --C $5 --seed $6 --select $7 --tseed $8 --target unplanted --out $TMP/$tag.ms > $tag.emit.json || return 1
  ( ulimit -v 1572864; nice -n 10 msolve -g 2 -t 1 -v 2 -f $TMP/$tag.ms -o $TMP/$tag.msolve.out > $tag.log 2>&1 )
  echo "{\"tag\":\"$tag\",\"c_E\":$(python3 -c "import json;print(json.load(open('$tag.emit.json'))['c_E'])"),\"decomposes\":$(python3 -c "import json;print(json.dumps(json.load(open('$tag.emit.json'))['decomposes']))"),\"f4\":$($BIN parse --log $tag.log)}" >> f4_unplanted.jsonl
  rm -f $TMP/$tag.ms $TMP/$tag.msolve.out
}
: > f4_unplanted.jsonl
for spec in "kummer 12289" "dickson 65519"; do
  set -- $spec; K=$1; P=$2
  for TS in $(seq 1 12); do
    for SEL in max med; do
      F4 $K $P 3 2 300 1 $SEL $TS; F4 $K $P 4 2 300 1 $SEL $TS; F4 $K $P 2 3 300 1 $SEL $TS; F4 $K $P 3 3 300 1 $SEL $TS
    done
  done
done
