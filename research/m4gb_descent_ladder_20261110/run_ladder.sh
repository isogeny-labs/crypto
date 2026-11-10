#!/usr/bin/env bash
# The one launcher of this study: every engine on the same seeded Weil-descent
# targets of every cell, interleaved per target, engine order rotating per
# repetition (ic descent --solver).  Run under tools/benchrec.py run.
#
#   M4GB_BINARY=<dir of solver_m4gb_n<N>_gf2> research/m4gb_descent_ladder_20261110/run_ladder.sh [out.json]
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:-$here/results/ladder.json}"
mkdir -p "$(dirname "$out")"
: "${M4GB_BINARY:?set M4GB_BINARY to a directory of solver_m4gb_n<N>_gf2 binaries}"
export M4GB_THREADS="${M4GB_THREADS:-1}"
# m = 2 square cells (m*n' boolean unknowns = 12 ... 32), both families, plus three m = 3 cells.
cells="11:6:2,13:7:2,15:8:2,17:9:2,19:10:2,21:11:2,23:12:2,25:13:2,27:14:2,29:15:2,31:16:2,11:4:3,13:5:3,15:5:3"
exec "$repo/target/release/ic" descent --json --out "$out" \
  --families K,R --cells "$cells" --targets "${TARGETS:-8}" --repeats "${REPEATS:-3}" \
  --budget-seconds "${BUDGET:-300}" \
  --solver fes-f2 --solver f4-f2 --solver inherited-f4 --solver m4gb-f2
