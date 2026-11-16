#!/usr/bin/env bash
# F4 early-stop study (lead L4 of #2216): measures --stop 64 and the Hilbert rule.
#
#   run.sh [outdir]      (default: raw/ beside this script; run from the repo root)
#
# 1. ccov (the #2216 workload) with no stop, --stop 64 and --stop hilbert:1,
#    p = 53, 101, 251, seed 1, 100 random + 20 planted residuals, 2 threads.
# 2. f4_stop_study: the same residual systems, every stop rule side by side
#    (per-step rows and multiplications, found sets compared with the full run,
#    regularity-check cost, hash of the leading forms).
# 3. F4_DEBUG per-step lines of the plain and stopped runs at p = 101.
# Counted field multiplications are the metric; no wall-clock claims.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${1:-$HERE/raw}
mkdir -p "$OUT"
export CARGO_BUILD_JOBS=4
cargo build --release --example jv_cover --example f4_stop_study
BIN=target/release/examples
A=(--exp ccov --seeds 1 --residuals 100 --constructed 20 --max-degree 24 --budget 300 --threads 2 --oracle-base 60)
for p in 53 101 251; do
  "$BIN/jv_cover" "${A[@]}" --sizes $p --json "$OUT/ccov_p${p}_full.json" > "$OUT/ccov_p${p}_full.txt"
  "$BIN/jv_cover" "${A[@]}" --sizes $p --stop 64 --json "$OUT/ccov_p${p}_stop64.json" > "$OUT/ccov_p${p}_stop64.txt"
  "$BIN/jv_cover" "${A[@]}" --sizes $p --stop hilbert:1 --json "$OUT/ccov_p${p}_hilbert1.json" > "$OUT/ccov_p${p}_hilbert1.txt"
  "$BIN/f4_stop_study" --sizes $p --residuals 100 --constructed 20 --threads 2 --json "$OUT/study_p${p}.json"
done
F4_DEBUG=1 "$BIN/jv_cover" "${A[@]}" --sizes 101 --constructed 0 2>&1 >/dev/null | grep '^f4 step [0-9]*:' > "$OUT/f4debug_p101_full.txt"
F4_DEBUG=1 "$BIN/jv_cover" "${A[@]}" --sizes 101 --constructed 0 --stop 64 2>&1 >/dev/null | grep '^f4 step [0-9]*:' > "$OUT/f4debug_p101_stop64.txt"
"$HERE/aggregate.sh" "$OUT"
