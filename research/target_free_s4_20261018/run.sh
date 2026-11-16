#!/usr/bin/env bash
# Eliminated target-free S4 study (PREREGISTRATION.md): per cell, gp plain, gp twisted and random plain,
# each with the S4 target arm on the same basis.  Stops at the first cell whose target-free records are all unfinished.
set -u
BIN=${1:-target/release/examples/dreg_eliminated_s4}
HERE=$(cd "$(dirname "$0")" && pwd); REPO=$(cd "$HERE/../.." && pwd); OUT=$HERE/runs
mkdir -p "$OUT"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 F4_F2_MAX_MATRIX_WORDS=$((1<<29)) RAYON_NUM_THREADS=3
for cell in 13:3 16:4 17:4 18:4 19:4 20:5 24:6; do
  for spec in "gp 0,0,0,0" "gp 0,1,2,3" "random 0,0,0,0"; do
    set -- $spec; basis=$1; tw=$2
    name="$OUT/cell-${cell/:/-}.$basis.tw${tw//,/}"
    if [ -s "$name.jsonl" ]; then echo "skip $name"; continue; fi
    echo "$(date -u +%FT%TZ) start $name" >> "$OUT/queue.log"
    (cd "$REPO" && prlimit --as=12000000000 timeout 10800 "$BIN" --cells "$cell" --basis "$basis" --twists "$tw" --draws 4 --seed 20261018 --budget-secs 1800) > "$name.jsonl" 2> "$name.log"
    rc=$?
    echo "$(date -u +%FT%TZ) exit $rc $name" >> "$OUT/queue.log"
  done
  # Stop condition: every target-free record of this cell unfinished (or none written).
  c=${cell/:/-}
  if ! grep -h '"arm":"target_free_s4"' "$OUT"/cell-$c.*.jsonl 2>/dev/null | grep -q '"timed_out":false,"oversize":false'; then
    echo "$(date -u +%FT%TZ) stop: no finished target-free record at ($cell); larger cells not run" >> "$OUT/queue.log"
    break
  fi
done
echo "$(date -u +%FT%TZ) queue done" >> "$OUT/queue.log"
