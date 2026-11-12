#!/usr/bin/env bash
# The end-to-end launcher: `ic bench --sweep` per instance (each writes its
# own JSON, so a killed run keeps the finished instances), smallest first.
# Instances are those of research/m4gb_e2e_ic_ladder_20261110; the algebraic
# oracle's solver is inherited-f4 (folded), inherited-f4-legacy and fes-then-inherited, beside the
# pair-table reference row and the matched counted rho.  Run under
# tools/benchrec.py run.
#
#   research/size_rule_fold_20261112/run_e2e.sh [results-dir]
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:-$here/results}"
mkdir -p "$out"
for sweep in ${SWEEPS:-koblitz-n13 char2-n15 koblitz-n17 koblitz-n19 char2-n21 koblitz-n23}; do
  f="$out/$sweep.json"
  if [ -s "$f" ]; then echo "skip $sweep (exists)"; continue; fi
  echo "run $sweep"
  "$repo/target/release/ic" bench --json --sweep "$here/sweeps/$sweep.json" --out "$f" \
    --solver-budget-seconds "${BUDGET:-60}" --rho-runs "${RHO_RUNS:-16}" --max-trials "${MAX_TRIALS:-2000000}" \
    >"$out/$sweep.log" 2>&1 || echo "sweep $sweep failed (exit $?)"
done
ls -la "$out"
