#!/bin/bash
# What was actually run at q = 131071 (PLAN Amendment 2): `run_ladder.sh` started the three instances in one
# process; after instance 0 had finished (its group-order search alone took 26 minutes under load) the process
# was stopped and instances 1 and 2 were run in two parallel processes with QDS_INST0. Records of instance 0:
# results/raw/ic_q131071.jsonl; instances 1 and 2: results/raw/ic_q131071_inst{1,2}.jsonl.
# usage: run_q131071_parallel.sh <binary> <out-dir>
BIN=${1:?binary}; OUT=${2:?out dir}; mkdir -p "$OUT"
export QDS_GIT=$(git rev-parse HEAD)
for i in 1 2; do
  QDS_INST0=$i "$BIN" run "$OUT/ic_q131071_inst$i.jsonl" 131071 1 7 100,200,400 none 800 2> "$OUT/ic_q131071_inst$i.log" &
done
wait
