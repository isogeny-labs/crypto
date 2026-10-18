#!/bin/sh
# Repair-pass smoke test of the item 05 instrument (not a measurement; nothing here is a card).
# Builds b05 into a throwaway directory, runs the revision cards' arm list (as ONE comma-separated
# argument, the way the -r2 cards pass it) once on target 0 of each of the four full cells so a
# panic or a nonzero exit shows here and not in a card, and checks that grevlex on target 0 still
# gives the first-run touches (the engine is deterministic, so the first-run grevlex records
# remain the pairing baseline for the -r2 arms). Run from the repository root.
set -eu
D=research/blindspots_20261018/05-elimination-orders
T=$(mktemp -d target/blindspots-05-smoke-XXXXXX)
trap 'rm -rf "$T"' EXIT
rustc --edition 2021 -C opt-level=3 -C target-cpu=native -o "$T/b05" examples/blindspots_05_elimination_orders.rs 2>/dev/null
A15=grevlex,wdeg,blkIL,mindeg,tw,mindeg-rev,tw-rev,rand0,rand1,rand2,rand3,rand4,rand5,rand6,rand7
A13=grevlex,wdeg,blkIL,mindeg,tw,rand0,rand1,rand2,rand3,rand4,rand5,rand6,rand7
for cell in c17-4-3 c19-4-3 c13-3-4 c15-3-4; do
  case $cell in c13-3-4) A=$A13;; *) A=$A15;; esac
  prlimit --as=4294967296 "$T/b05" run --draws $D/draws.tsv --cell $cell --targets 0 --arms "$A" \
    --time-limit 60 --total-limit 400 > "$T/$cell.out" 2> "$T/$cell.err"
  n=$(grep -c '"status":"refuted"' "$T/$cell.out")
  want=$(echo "$A" | tr ',' '\n' | wc -l)
  [ "$n" = "$want" ] || { echo "FAIL $cell: $n of $want arms refuted"; exit 1; }
  new=$(grep '"arm":"grevlex"' "$T/$cell.out" | sed 's/.*"touches":\([0-9]*\),.*/\1/')
  old=$(grep '"target":0,"arm":"grevlex"' $(ls $D/raw/card-0[2-6]-main-$cell.stdout.log) | sed 's/.*"touches":\([0-9]*\),.*/\1/')
  [ "$new" = "$old" ] || { echo "FAIL $cell: grevlex touches $new differ from first run $old"; exit 1; }
  echo "$cell: $n/$want arms refuted, grevlex touches identical to first run ($new)"
done
echo SMOKE-R2 OK
