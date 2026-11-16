#!/usr/bin/env bash
# Frozen run script (see PROTOCOL.md). Run from the repo root.
set -u
D=research/jv_cover_fbline_20261102; R=$D/results
B=${JV_BIN:-target/release/examples/jv_cover}
run() { label=$1; to=$2; shift 2; s0=$(date +%s)
  env "${ENVV[@]}" timeout $to $B "$@" --json $R/$label.json > $R/$label.log 2>&1; rc=$?
  echo "$label rc=$rc wall=$(( $(date +%s)-s0 ))s" >> $R/walls.txt; }
COMMON=(--exp dlp --rho-runs 0 --rho-ref 1.3 --check-every 64)
# C1: baseline unchanged (JV_FB_LINE unset)
ENVV=(-u JV_FB_LINE); run c1_plain_p1301_s3 3600 "${COMMON[@]}" --sizes 1301 --first-seed 3 --seeds 1
# C2: machinery null (only the plain line)
ENVV=(JV_FB_LINE=1); run c2_line1_p1301_s3 3600 "${COMMON[@]}" --sizes 1301 --first-seed 3 --seeds 1
# Arm B
ENVV=(JV_FB_LINE=65536)
for p in 1301 1451 1601; do for s in 1 2 3; do
  run line_p${p}_s$s 3600 "${COMMON[@]}" --sizes $p --first-seed $s --seeds 1
done; done
