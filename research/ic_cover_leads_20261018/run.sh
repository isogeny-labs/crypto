#!/usr/bin/env bash
# Cost breakdown of one Nagao decomposition test of examples/jv_cover.rs on
# main after #1717, to rank the next optimisation leads.
#
#   run.sh <outdir>
#
# 1. ccov with and without --trace (F4 trace replay), p = 53, 101, seed 1:
#    stock release build.
# 2. The linear stage (solve_zero_dim) split into its parts, and the number of
#    columns with more than one nonzero entry in each multiplication matrix: release build with
#    raw/instrument.patch applied (eprintln only, LEADS_DBG=1), p = 101.
# 3. F4 rows, pivots and zero reductions per degree: F4_DEBUG=1, p = 101,
#    100 random residuals.
# Aggregation is awk over the recorded lines; nothing is recomputed.
set -euo pipefail
OUT=${1:?outdir}
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"
PATCH="$SCRIPT_DIR/reports/run-20261018T1900Z/raw/instrument.patch"
git diff --quiet HEAD -- src examples/jv_cover.rs Cargo.toml Cargo.lock build.rs .cargo tools/volcano-cli isogeny_algos
git apply --check "$PATCH"
# Evidence directories are immutable: a fresh run must not overwrite an old one.
mkdir -p -- "$(dirname -- "$OUT")"
mkdir -- "$OUT"
OUT=$(cd -- "$OUT" && pwd)
mkdir -- "$OUT/raw"
cp -- "$PATCH" "$OUT/raw/instrument.patch"
git rev-parse HEAD > "$OUT/source-revision.txt"
patch_applied=0
cleanup() {
    status=$?
    trap - EXIT
    if ((patch_applied)); then
        if ! git apply -R "$OUT/raw/instrument.patch"; then
            echo "Failed to reverse instrumentation; inspect src/cryptanalysis/jv_cover.rs" >&2
            status=1
        fi
    fi
    exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
A=(--exp ccov --seeds 1 --residuals 100 --max-degree 24 --budget 300)
cargo build --release --locked --example jv_cover
BIN=${CARGO_TARGET_DIR:-target}/release/examples/jv_cover
shasum -a 256 "$BIN" > "$OUT/binary-stock.sha256"
"$BIN" "${A[@]}" --sizes 53,101 --constructed 20 --json "$OUT/raw/ccov.json"
"$BIN" "${A[@]}" --sizes 53,101 --constructed 20 --trace --json "$OUT/raw/ccov_trace.json"
F4_DEBUG=1 "$BIN" "${A[@]}" --sizes 101 --constructed 0 2>&1 | grep '^f4 step [0-9]*:' > "$OUT/raw/f4steps.txt"
git apply "$OUT/raw/instrument.patch"
patch_applied=1
cargo build --release --locked --example jv_cover
shasum -a 256 "$BIN" > "$OUT/binary-instrumented.sha256"
LEADS_DBG=1 "$BIN" "${A[@]}" --sizes 101 --constructed 20 > "$OUT/raw/lin_breakdown_p101.log" 2>&1
git apply -R "$OUT/raw/instrument.patch"
patch_applied=0
"$SCRIPT_DIR/aggregate.sh" "$OUT" 100
