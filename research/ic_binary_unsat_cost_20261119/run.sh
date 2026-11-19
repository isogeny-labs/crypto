#!/usr/bin/env bash
# Registered run: 8 cells, sequential, single thread. Raw output under raw/ (gitignored).
set -u
cd "$(dirname "$0")/../.."
export RAYON_NUM_THREADS=1
BIN=target/release/examples/ic_unsat_relabel
OUT=research/ic_binary_unsat_cost_20261119/raw
for cell in "0 13" "0 15" "1 15" "1 17" "1 19" "0 19" "0 23" "1 23"; do
  set -- $cell
  dim=$(( ($2 + 2) / 3 ))
  $BIN --a $1 --n $2 --dim $dim --m 3 --targets 60 --reps 3 --seed 20261119 --budget 2000 \
    > $OUT/a$1_n$2.jsonl 2> $OUT/a$1_n$2.err
  echo "cell a=$1 n=$2 dim=$dim exit=$?"
done
