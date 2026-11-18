#!/bin/sh
# Thin build-and-run orchestration for the lin21 pilot (no computation here).
#   ./run.sh OUTDIR [CELLS]      e.g.  ./run.sh /tmp/lin21-l5 5      (smallest cell)
# Builds the two Rust files with rustc -O, runs the selftest, then for every cell writes the
# binary's target lines to OUTDIR/raw/targets_l<l>.jsonl and appends one binary-printed JSON
# line per (target, arm) to OUTDIR/raw/rows.jsonl, 2 processes at a time, 600 s cap each
# (a capped process leaves a "censored" line). `summary` then writes the tables and the SVG.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${1:?usage: run.sh OUTDIR [CELLS]}
CELLS=${2:-"5 6 7 8 9 10 11 12"}
SEED=${SEED:-20261116}
CAP=600
mkdir -p "$OUT/raw" "$OUT/bin"
rustc -O -o "$OUT/bin/lin21" "$HERE/lin21.rs" 2>/dev/null
rustc -O -o "$OUT/bin/summary" "$HERE/summary.rs" 2>/dev/null
"$OUT/bin/lin21" selftest > "$OUT/raw/selftest.jsonl"
: > "$OUT/raw/rows.jsonl"
# cell plan: "arm:k" = first k non-decomposable uniform targets plus all planted; "arm" = all
plan() {
  case $1 in
    5|6|7) echo "scan lin lin-nolex lin21 lin21-pu" ;;
    8) echo "scan lin lin-nolex:4 lin21 lin21-pu" ;;
    9) echo "scan lin:8 lin21 lin21-pu" ;;
    10|11) echo "scan lin21 lin21-pu" ;;
    *) echo "scan lin21" ;;
  esac
}
for l in $CELLS; do
  T="$OUT/raw/targets_l$l.jsonl"
  "$OUT/bin/lin21" targets --l "$l" --seed "$SEED" --uniform 16 --planted 4 > "$T"
  : > "$OUT/raw/jobs_l$l.txt"
  for spec in $(plan "$l"); do
    arm=${spec%%:*}; k=${spec#*:}; [ "$k" = "$spec" ] && k=all
    sed -n 's/.*"kind":"\([a-z-]*\)".*"x_r":\([0-9]*\),.*/\1 \2/p' "$T" | {
      u=0
      while read -r kind xr; do
        if [ "$k" != all ]; then
          [ "$kind" = uniform-dec ] && continue
          if [ "$kind" = uniform ]; then u=$((u + 1)); [ "$u" -gt "$k" ] && continue; fi
        fi
        echo "$l $xr $arm" >> "$OUT/raw/jobs_l$l.txt"
      done
    }
  done
  # one process per job; sort by target so no arm owns a time window
  sort -k2,2n -k3,3 "$OUT/raw/jobs_l$l.txt" | EXE="$OUT/bin/lin21" ROWS="$OUT/raw/rows.jsonl" CAP=$CAP xargs -P 2 -L 1 sh -c '
    line=$(timeout "$CAP" "$EXE" run --l "$0" --xr "$1" --arm "$2") ||
      line="{\"arm\":\"$2\",\"l\":$0,\"x_r\":$1,\"status\":\"censored\",\"cap_s\":$CAP}"
    echo "$line" >> "$ROWS"'
  echo "l=$l done: $(wc -l < "$OUT/raw/jobs_l$l.txt") jobs"
done
"$OUT/bin/summary" "$OUT" > /dev/null
echo "wrote $OUT/tables.md $OUT/summary.json $OUT/results.json $OUT/fig_conflicts_vs_l.svg"
