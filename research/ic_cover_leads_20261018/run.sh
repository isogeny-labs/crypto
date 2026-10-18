#!/usr/bin/env bash
# Cost breakdown of one Nagao decomposition test of examples/jv_cover.rs on
# main after #1717, to rank the next optimisation leads.
#
#   run.sh <outdir>
#
# 1. ccov with and without --trace (F4 trace replay), p = 53, 101, seed 1:
#    stock release build.
# 2. The linear stage (solve_zero_dim) split into its parts, and the number of
#    non-unit columns of each multiplication matrix: release build with
#    raw/instrument.patch applied (eprintln only, LEADS_DBG=1), p = 101.
# 3. F4 rows, pivots and zero reductions per degree: F4_DEBUG=1, p = 101,
#    100 random residuals.
# Aggregation is awk over the recorded lines; nothing is recomputed.
set -euo pipefail
OUT=${1:?outdir}
mkdir -p "$OUT/raw"
A=(--exp ccov --seeds 1 --residuals 100 --max-degree 24 --budget 300)
cargo build --release --example jv_cover
BIN=target/release/examples/jv_cover
"$BIN" "${A[@]}" --sizes 53,101 --constructed 20 --json "$OUT/raw/ccov.json"
"$BIN" "${A[@]}" --sizes 53,101 --constructed 20 --trace --json "$OUT/raw/ccov_trace.json"
F4_DEBUG=1 "$BIN" "${A[@]}" --sizes 101 --constructed 0 2>&1 | grep '^f4 step [0-9]*:' > "$OUT/raw/f4steps.txt"
git apply "$OUT/raw/instrument.patch"
cargo build --release --example jv_cover
LEADS_DBG=1 "$BIN" "${A[@]}" --sizes 101 --constructed 20 > "$OUT/raw/lin_breakdown_p101.log" 2>&1
git apply -R "$OUT/raw/instrument.patch"
"$(dirname "$0")/aggregate.sh" "$OUT"
