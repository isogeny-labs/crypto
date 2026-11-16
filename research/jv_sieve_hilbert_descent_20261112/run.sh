#!/usr/bin/env bash
# LEAD-C-jv-sieve-hilbert-descent, local stage. Run from the repo root.
# Plain arm = the committed V2 command (jv_sieve_negrho_20261102 / fbline validator replay); must reproduce the committed records.
# Hilbert arm = the same command with JV_SIEVE_HILBERT=16 (descent F4 uses StopMode::Hilbert(16)).
set -u
D=research/jv_sieve_hilbert_descent_20261112; R=$D/results
B=${JV_BIN:-target/release/examples/jv_cover}
run() { label=$1; seed=$2; shift 2; s0=$(date +%s)
  [ -s $R/$label.json ] && return
  env -u JV_FB_LINE -u JV_FB_LINE_MODE -u JV_SIEVE_HILBERT JV_LIN=sparse "$@" timeout 1800 $B --exp sieve --stop 64 --sizes 1511 \
    --first-seed $seed --seeds 1 --rho-runs 0 --json $R/$label.json.tmp > $R/$label.log 2>&1; rc=$?
  [ $rc = 0 ] && mv $R/$label.json.tmp $R/$label.json
  echo "$label rc=$rc wall=$(( $(date +%s)-s0 ))s" >> $R/walls.txt; }
for s in 17 20 27 32; do run plain_s$s $s; done
for s in $(seq 17 32); do run hilbert_s$s $s JV_SIEVE_HILBERT=16; done
echo ALL-DONE >> $R/walls.txt
