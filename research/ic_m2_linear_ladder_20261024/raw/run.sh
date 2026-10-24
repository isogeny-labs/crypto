#!/bin/sh
# Thin orchestration of the frozen ladder (PROTOCOL.md): one process per rung.
B=/home/user/bin/ic_m2_linear
for spec in "0 13 8" "1 17 8" "0 19 8" "1 19 8" "0 23 8" "1 23 8" "0 37 4" "0 41 4"; do
  set -- $spec
  echo "start a=$1 n=$2 $(date -u +%FT%TZ)"
  $B $1 $2 0 $3 20261024 rung_a$1_n$2.json 2> rung_a$1_n$2.log
  echo "done a=$1 n=$2 rc=$? $(date -u +%FT%TZ)"
done
echo LADDER_COMPLETE
