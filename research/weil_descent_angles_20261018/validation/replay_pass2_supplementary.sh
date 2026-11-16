#!/bin/bash
# Supplementary full-coverage replays (validator-chosen, NOT nonce-drawn).
set -u
V=/tmp/claude-0/-home-user/829a5db7-e7b3-5b24-827a-595408da4b3a/scratchpad/validator
BIN=$V/bin/n2
R=$V/replay
until grep -q ALLDONE $R/replay_log.txt 2>/dev/null; do sleep 15; done
run() {
  local name=$1 mode=$2; shift 2
  local out=$R/$name
  rm -rf $out; mkdir -p $out
  local t0=$(date +%s)
  env "$@" N2_THREADS=4 N2_DUMP=$out/dump.jsonl $BIN $out $mode > $out/stdout.log 2> $out/stderr.log
  echo "$name mode=$mode env=$* rc=$? wall=$(( $(date +%s) - t0 ))s" | tee -a $R/replay_log.txt
}
for b in 28 32 36 40; do run SUPP_itemF__b$b control N2_SIZES=$b; done
for b in 24 36; do run SUPP_itemOJ__b$b run N2_SIZES=$b; done
for cell in 28:1 28:4 32:4 36:4 40:1; do b=${cell%:*}; h=${cell#*:}; run SUPP_H2__b${b}_h$h glue N2_SIZES=$b N2_HS=$h; done
for b in 32 36 40; do
  hs=""
  for h in 4 8 12 16 24 32 48 64 96 128; do
    case "$b:$h" in 32:96|36:16|40:12) continue;; esac
    hs="$hs,$h"
  done
  run SUPP_H1__b${b}_rest hsweep N2_SIZES=$b N2_HS=${hs#,}
done
run SUPP_H6__b48_c2 run N2_SIZES=48 N2_CS=2
echo ALLDONE2 | tee -a $R/replay_log.txt
