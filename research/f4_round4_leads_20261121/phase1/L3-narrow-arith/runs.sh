#!/bin/bash
cd /tmp/claude-0/-home-user/557dc6d7-a322-5dce-a371-122a5cef1780/scratchpad/round4/L3-narrow-arith
for v in "FL0 F4_FP_FL_MIN_COLS=0" "SIMD0 F4_FP_LAZY_SIMD=0" "NARROW1 F4_FP_NARROW=1" "NARROW0 F4_FP_NARROW=0"; do
  set -- $v; tag=$1; kv=$2
  s=$(date +%s)
  env RAYON_NUM_THREADS=1 $kv timeout 900 valgrind --tool=callgrind --dump-instr=yes --toggle-collect='*f4_with_arithmetic*' --callgrind-out-file=$tag.cg ../../new3/jv_cover --exp ccov --sizes 53 --seeds 1 --residuals 100 --constructed 4 --oracle-base 0 --json $tag.json > $tag.out 2> $tag.err
  echo "$tag rc=$? secs=$(( $(date +%s)-s ))" >> runs.log
done
