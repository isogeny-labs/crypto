#!/usr/bin/env bash
# LEAD-C-jv-nagao-hilbert-at-scale, stage 1 (counts only), run from the repo root.
# A/A + drift control: line_p1301_s1 re-run with the frozen fbline command on today's binary.
# Arm B: the 9 fbline instances with --stop hilbert:16 (JV_FB_LINE=65536 as in the frozen arm).
set -u
D=research/jv_nagao_hilbert_20261112; R=$D/results
B=${JV_BIN:-target/release/examples/jv_cover}
run() { label=$1; shift; s0=$(date +%s)
  [ -s $R/$label.json ] && { echo "$label skip (done)" >> $R/walls.txt; return; }
  env JV_FB_LINE=65536 timeout 3600 $B "$@" --json $R/$label.json.tmp > $R/$label.log 2>&1; rc=$?
  [ $rc = 0 ] && mv $R/$label.json.tmp $R/$label.json
  echo "$label rc=$rc wall=$(( $(date +%s)-s0 ))s" >> $R/walls.txt; }
COMMON=(--exp dlp --rho-runs 0 --rho-ref 1.3 --check-every 64)
run aa_line_p1301_s1 "${COMMON[@]}" --sizes 1301 --first-seed 1 --seeds 1
for p in 1301 1451 1601; do for s in 1 2 3; do
  run hilbert_p${p}_s$s "${COMMON[@]}" --stop hilbert:16 --sizes $p --first-seed $s --seeds 1
done; done
echo ALL-DONE >> $R/walls.txt
