#!/bin/sh
# Thin build-and-run orchestration for the lin21 wall-gap pilot (no computation here).
#   ./run.sh OUTDIR [CELLS] [PROCS]     e.g.  ./run.sh /tmp/l21fast-l5 5      (smallest cell)
# Builds l21fast.rs and summary.rs with rustc -O, runs the selftest, then for every target of
# the lin21 pilot's target files (same targets, same seed) starts one process that runs the
# arms scan, lin21-fast, lin21 (original solver), lin21-fast again, in that order, and appends
# the binary's JSON lines to OUTDIR/raw/rows.jsonl. The profiling copy of the original (Gauss
# work counters, not a timed arm) is added on every target at l <= 9 and on the first four
# targets of each larger cell. PROCS processes at a time (default 1), 600 s cap per process.
# Extension cells: OUTDIR/raw/targets_l<l>.jsonl is used when the pilot has no file for l.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${1:?usage: run.sh OUTDIR [CELLS] [PROCS]}
CELLS=${2:-"5 6 7 8 9 10 11 12"}
PROCS=${3:-1}
TDIR=${TDIR:-"$HERE/../sat_native_lin21_20261116/raw"}   # override only for dry runs on other seeds
CAP=600
mkdir -p "$OUT/raw" "$OUT/bin"
# the shared field/curve/oracle/scan code is lines 16..862 of the pilot's lin21.rs, verbatim
sed -n '16,862p' "$HERE/../sat_native_lin21_20261116/lin21.rs" | cmp - "$HERE/base_lin21_verbatim.rs"
sed -f "$HERE/prof.sed" "$HERE/../../examples/pdp_lin_cdcl.rs" | cmp - "$HERE/lincdcl_prof.rs"
rustc -O -o "$OUT/bin/l21fast" "$HERE/l21fast.rs" 2>/dev/null
rustc -O -o "$OUT/bin/summary" "$HERE/summary.rs" 2>/dev/null
"$OUT/bin/l21fast" selftest > "$OUT/raw/selftest.jsonl"
[ -f "$OUT/raw/rows.jsonl" ] || : > "$OUT/raw/rows.jsonl"
for l in $CELLS; do
  T="$TDIR/targets_l$l.jsonl"; [ -f "$T" ] || T="$OUT/raw/targets_l$l.jsonl"
  sed -n 's/.*"idx":\([0-9]*\),.*"x_r":\([0-9]*\),.*/\1 \2/p' "$T" | while read -r idx xr; do
    prof=0; if [ "$l" -le 9 ] || [ "$idx" -lt 4 ]; then prof=1; fi
    echo "$l $xr $prof"
  done > "$OUT/raw/jobs_l$l.txt"
  EXE="$OUT/bin/l21fast" ROWS="$OUT/raw/rows.jsonl" CAP=$CAP xargs -P "$PROCS" -L 1 sh -c '
    lines=$(timeout "$CAP" "$EXE" run --l "$0" --xr "$1" --arms scan,fast,lin21,fast --prof "$2") ||
      lines="{\"arm\":\"lin21\",\"l\":$0,\"x_r\":$1,\"status\":\"censored\",\"cap_s\":$CAP}"
    echo "$lines" >> "$ROWS"' < "$OUT/raw/jobs_l$l.txt"
  echo "l=$l done: $(wc -l < "$OUT/raw/jobs_l$l.txt") targets"
done
"$OUT/bin/summary" "$OUT" "$TDIR"
echo "wrote $OUT/tables.md $OUT/results.json $OUT/fig_wall_vs_l.svg"
