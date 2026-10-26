#!/bin/bash
# Replays of the nonce-drawn cells (plus one supplementary validator-chosen cell).
set -u
V=/tmp/claude-0/-home-user/829a5db7-e7b3-5b24-827a-595408da4b3a/scratchpad/validator
BIN=$V/bin/n2
R=$V/replay
mkdir -p $R
run() { # name mode env...
  local name=$1 mode=$2; shift 2
  local out=$R/$name
  rm -rf $out; mkdir -p $out
  local t0=$(date +%s)
  env "$@" N2_THREADS=4 N2_DUMP=$out/dump.jsonl $BIN $out $mode > $out/stdout.log 2> $out/stderr.log
  local rc=$?
  echo "$name mode=$mode env=$* rc=$rc wall=$(( $(date +%s) - t0 ))s" | tee -a $R/replay_log.txt
}
run itemF__b24 control N2_SIZES=24
run itemF__b20 control N2_SIZES=20
run itemOJ__b20 run N2_SIZES=20
run itemOJ__b28 run N2_SIZES=28
run itemOJ__b32 run N2_SIZES=32
run H4__b32 memsweep N2_SIZES=32
run H4__b36 memsweep N2_SIZES=36
run H2__b32_h1 glue N2_SIZES=32 N2_HS=1
run H2__b36_h1 glue N2_SIZES=36 N2_HS=1
run H2__b40_h4 glue N2_SIZES=40 N2_HS=4
run H2__gluecheck gluecheck
run H1__b32_h96 hsweep N2_SIZES=32 N2_HS=96
run H1__b36_h16 hsweep N2_SIZES=36 N2_HS=16
run H1__b40_h12 hsweep N2_SIZES=40 N2_HS=12
run H6__b44_c2 run N2_SIZES=44 N2_CS=2
run SUPP_itemOJ__b40 run N2_SIZES=40
echo ALLDONE | tee -a $R/replay_log.txt
