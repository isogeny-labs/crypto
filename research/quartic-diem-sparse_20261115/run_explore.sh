#!/bin/bash
# Exploratory arms of PLAN Amendment 1 (bfs rows; lightest rows with 6 b cycles), same instances and seeds.
# usage: run_explore.sh <binary> <out-dir>
BIN=${1:?binary}; OUT=${2:?out dir}; mkdir -p "$OUT"
export QDS_GIT=$(git rev-parse HEAD)
for q in 1021 4093 16381 65521 131071; do
  QDS_ROWMODE=bfs "$BIN" run "$OUT/xbfs_q$q.jsonl" $q 3 7 200,400 none 800 2> "$OUT/xbfs_q$q.log"
  QDS_ROWMODE=light QDS_LIGHTMULT=6 "$BIN" run "$OUT/xlight_q$q.jsonl" $q 3 7 200,400 none 800 2> "$OUT/xlight_q$q.log"
done
