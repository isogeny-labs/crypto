#!/usr/bin/env bash
# LEAD-C-jv-sieve-emit-price, local stage. Run from the repo root.
# plain arm: committed V2 command (as research/jv_sieve_hilbert_descent_20261112/run.sh), single-threaded;
#   JV_SIEVE_ROUND_THREADS=4 keeps the round plan of the committed 4-thread runs (jv_sieve.rs: the plan
#   reads rayon::current_num_threads otherwise).
# emit arm: the same command with JV_SIEVE_EMIT=<sidecar> (jv_bitops counters on, one-thread pool).
set -u
D=research/leads_20261111/runs/C_sieve_emit_price; R=$D/results
B=${JV_BIN:?set JV_BIN to the patched jv_cover binary}
run() { label=$1; seed=$2; shift 2; s0=$(date +%s)
  [ -s $R/$label.json ] && return
  env -u JV_FB_LINE -u JV_FB_LINE_MODE -u JV_SIEVE_HILBERT -u JV_SIEVE_EMIT RAYON_NUM_THREADS=1 JV_SIEVE_ROUND_THREADS=4 JV_LIN=sparse "$@" \
    timeout 1800 $B --exp sieve --stop 64 --sizes 1511 \
    --first-seed $seed --seeds 1 --rho-runs 0 --json $R/$label.json.tmp > $R/$label.log 2>&1; rc=$?
  [ $rc = 0 ] && mv $R/$label.json.tmp $R/$label.json
  echo "$label rc=$rc wall=$(( $(date +%s)-s0 ))s" >> $R/walls.txt; }
for s in $(seq 17 32); do run emit_s$s $s JV_SIEVE_EMIT=$R/emit_s$s.side.json; done
for s in 17 24 27 32; do run plain_s$s $s; done
echo ALL-DONE >> $R/walls.txt
