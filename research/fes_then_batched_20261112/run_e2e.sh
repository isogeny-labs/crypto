#!/usr/bin/env bash
# The end-to-end launcher: `ic bench --sweep` per instance (each writes its
# own JSON, so a killed run keeps the finished instances), smallest first.
# Instances are those of research/m4gb_e2e_ic_ladder_20261110; the algebraic
# oracle's solver is inherited-f4, fes-then-inherited and fes-then-batched, beside the
# pair-table reference row and the matched counted rho.  Run under
# tools/benchrec.py run.
#
#   [IC_BIN=<ic binary>] research/fes_then_batched_20261112/run_e2e.sh [results-dir]
#
# IC_BIN (default target/release/ic) lets the run use a frozen copy of the
# binary while the checkout is rebuilt; its SHA-256 is written to the results.
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:-$here/results}"
mkdir -p "$out"
ic="${IC_BIN:-$repo/target/release/ic}"
sha256sum "$ic" >"$out/ic-sha256.txt"
for sweep in ${SWEEPS:-koblitz-n13 char2-n15 koblitz-n17 koblitz-n19 char2-n21 koblitz-n23}; do
  f="$out/$sweep.json"
  if [ -s "$f" ]; then echo "skip $sweep (exists)"; continue; fi
  echo "run $sweep"
  "$ic" bench --json --sweep "$here/sweeps/$sweep.json" --out "$f" \
    --solver-budget-seconds "${BUDGET:-60}" --rho-runs "${RHO_RUNS:-16}" --max-trials "${MAX_TRIALS:-2000000}" \
    >"$out/$sweep.log" 2>&1 || echo "sweep $sweep failed (exit $?)"
done
ls -la "$out"
