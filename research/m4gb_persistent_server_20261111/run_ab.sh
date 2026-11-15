#!/usr/bin/env bash
# The one launcher of this study: the one-shot M4GB adapter (`m4gb-f2`, one
# process per call) against the persistent server adapter (`m4gb-f2-server`,
# one `--serve` process per binary), with `inherited-f4` as the in-house
# scale, on the same seeded Weil-descent targets of every cell, interleaved
# per target, engine order rotating per repetition (ic descent --solver).
# One ic invocation per cell and family.  Run under tools/benchrec.py run.
#
#   M4GB_BINARY=<dir of solver_m4gb_n<N>_gf2 built with patch 0003> research/m4gb_persistent_server_20261111/run_ab.sh [results-dir]
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:-$here/results}"
mkdir -p "$out"
: "${M4GB_BINARY:?set M4GB_BINARY to a directory of solver_m4gb_n<N>_gf2 binaries}"
export M4GB_THREADS="${M4GB_THREADS:-1}"
TARGETS="${TARGETS:-6}"; REPEATS="${REPEATS:-3}"; BUDGET="${BUDGET:-60}"
CELLS_M2="${CELLS_M2:-11:6:2 13:7:2 15:8:2 17:9:2 19:10:2 21:11:2 23:12:2}"
CELLS_M3="${CELLS_M3:-11:4:3 13:5:3 15:5:3}"
"$repo/target/release/ic" --version >"$out/ic-version.txt" 2>&1 || true
for b in "$M4GB_BINARY"/solver_m4gb_n*_gf2; do printf '%s  %s\n' "$(sha256sum "$b" | cut -c1-16)" "$(basename "$b")"; done >"$out/m4gb-binaries.txt"
run_cell() {  # family cell
  local fam=$1 cell=$2 tag
  tag="${fam}-$(echo "$cell" | tr ':' '-')"
  local f="$out/cell-$tag.json"
  if [ -s "$f" ]; then echo "skip $tag (exists)"; return 0; fi
  echo "run $tag $(date -u +%FT%TZ)"
  "$repo/target/release/ic" descent --json --out "$f" --families "$fam" --cells "$cell" \
    --targets "$TARGETS" --repeats "$REPEATS" --budget-seconds "$BUDGET" \
    --solver inherited-f4 --solver m4gb-f2 --solver m4gb-f2-server >"$out/cell-$tag.log" 2>&1 \
    || echo "cell $tag failed (exit $?)"
}
for fam in K R; do for cell in $CELLS_M2; do run_cell "$fam" "$cell"; done; done
for fam in K R; do for cell in $CELLS_M3; do run_cell "$fam" "$cell"; done; done
ls -la "$out"
