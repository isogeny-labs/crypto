#!/usr/bin/env bash
# The one launcher: `ic bench --sweep` per instance (each writes its own JSON,
# so a killed run keeps the finished instances), smallest first.  Run under
# tools/benchrec.py run with M4GB_BINARY set to the directory of solver_m4gb_n<N>_gf2.
#
#   M4GB_BINARY=<dir> research/m4gb_e2e_ic_ladder_20261110/run_e2e.sh [results-dir]
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:-$here/results}"
mkdir -p "$out"
: "${M4GB_BINARY:?set M4GB_BINARY to a directory of solver_m4gb_n<N>_gf2 binaries}"
export M4GB_THREADS="${M4GB_THREADS:-1}"
for sweep in ${SWEEPS:-koblitz-n13 char2-n15 koblitz-n17 koblitz-n19 char2-n21 koblitz-n23}; do
  f="$out/$sweep.json"
  if [ -s "$f" ]; then echo "skip $sweep (exists)"; continue; fi
  echo "run $sweep"
  "$repo/target/release/ic" bench --json --sweep "$here/sweeps/$sweep.json" --out "$f" \
    --solver-budget-seconds "${BUDGET:-60}" --rho-runs "${RHO_RUNS:-16}" --max-trials "${MAX_TRIALS:-2000000}" \
    >"$out/$sweep.log" 2>&1 || echo "sweep $sweep failed (exit $?)"
done
ls -la "$out"
