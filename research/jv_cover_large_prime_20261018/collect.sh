#!/usr/bin/env bash
# Collect real decompositions with their six factor-base indices (JV_LP_LOG=1,
# opt-in, read-only) by running the unmodified dlp path.  2 threads.
#   collect.sh <jv_cover binary> <raw dir>
set -uo pipefail
BIN=$1; RAW=$2
mkdir -p "$RAW"
export RAYON_NUM_THREADS=2 JV_LP_LOG=1
now_ms() { echo $(( $(date +%s%N) / 1000000 )); }
run() { # p seed
  local p=$1 s=$2 a; a=$(now_ms)
  "$BIN" --exp dlp --sizes "$p" --seeds 1 --first-seed "$s" --rho-runs 0 --rho-ref 1.3 \
     --check-every 0 --json "$RAW/dlp_p${p}_s${s}.json" \
     > "$RAW/dlp_p${p}_s${s}.out" 2> "$RAW/dlp_p${p}_s${s}.log"
  echo "p=$p seed=$s exit $? wall_ms $(( $(now_ms) - a ))" >> "$RAW/walls.txt"
}
for s in 1 2 3 4 5 6; do run 101 "$s"; done
for s in 1 2 3; do run 251 "$s"; done
