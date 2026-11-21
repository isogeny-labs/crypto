#!/usr/bin/env bash
# The one launcher of this study: every engine on the same seeded Weil-descent
# targets of every cell, interleaved per target, engine order rotating per
# repetition (ic descent --solver).  One ic invocation per cell and family so
# each cell's JSON survives on its own; the m = 2 ladder stops for a family
# once m4gb-f2 exhausted the budget on every target of a cell (the larger
# cells would only repeat that).  Run under tools/benchrec.py run.
#
#   M4GB_BINARY=<dir of solver_m4gb_n<N>_gf2> research/m4gb_descent_ladder_20261110/run_ladder.sh [results-dir]
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:-$here/results}"
mkdir -p "$out"
: "${M4GB_BINARY:?set M4GB_BINARY to a directory of solver_m4gb_n<N>_gf2 binaries}"
export M4GB_THREADS="${M4GB_THREADS:-1}"
TARGETS="${TARGETS:-6}"; REPEATS="${REPEATS:-2}"; BUDGET="${BUDGET:-60}"
m2_cells="11:6:2 13:7:2 15:8:2 17:9:2 19:10:2 21:11:2 23:12:2 25:13:2 27:14:2 29:15:2 31:16:2"
m3_cells="11:4:3 13:5:3 15:5:3"
run_cell() {  # family cell
  local fam=$1 cell=$2 tag
  tag="${fam}-$(echo "$cell" | tr ':' '-')"
  local f="$out/cell-$tag.json"
  if [ -s "$f" ]; then echo "skip $tag (exists)"; return 0; fi
  echo "run $tag"
  "$repo/target/release/ic" descent --json --out "$f" --families "$fam" --cells "$cell" \
    --targets "$TARGETS" --repeats "$REPEATS" --budget-seconds "$BUDGET" \
    --solver fes-f2 --solver f4-f2 --solver inherited-f4 --solver m4gb-f2 >"$out/cell-$tag.log" 2>&1 \
    || echo "cell $tag failed (exit $?)"
}
m4gb_exhausted() {  # 1 if m4gb-f2 hit the budget on every target of the cell in this file
  python3 - "$1" <<'PY'
import json, sys
try:
    d = json.load(open(sys.argv[1]))
except Exception:
    sys.exit(1)
for c in d.get('cells', []):
    for e in c['engines']:
        if e['engine'] == 'm4gb-f2' and e['over_budget'] >= c['targets']:
            sys.exit(0)
sys.exit(1)
PY
}
for fam in K R; do
  for cell in $m2_cells; do
    run_cell "$fam" "$cell"
    if m4gb_exhausted "$out/cell-${fam}-$(echo "$cell" | tr ':' '-').json"; then
      echo "m4gb-f2 over budget on every target of $fam $cell: stopping the m = 2 ladder for $fam"; break
    fi
  done
done
for fam in K R; do for cell in $m3_cells; do run_cell "$fam" "$cell"; done; done
ls -la "$out"
