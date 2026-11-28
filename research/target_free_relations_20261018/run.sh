#!/usr/bin/env bash
# L1 measurement (PREREGISTRATION.md): per cell, gp (1 draw: deterministic) and random (4 draws),
# twists plain and [0,1,2,3], each with the m = 3 target arm on the same basis.
set -u
BIN=${1:-target/release/examples/dreg_target_free}
HERE=$(cd "$(dirname "$0")" && pwd); REPO=$(cd "$HERE/../.." && pwd); OUT=$HERE/runs
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 RAYON_NUM_THREADS=2
for cell in 16:4 18:4 20:5 24:6; do
  for basis in gp random; do
    draws=4; [ "$basis" = gp ] && draws=1
    for tw in 0,0,0,0 0,1,2,3; do
      name="$OUT/cell-${cell/:/-}.$basis.tw${tw//,/}"
      if [ -s "$name.jsonl" ]; then echo "skip $name"; continue; fi
      echo "$(date -u +%FT%TZ) start $name" | tee -a "$OUT/queue.log"
      (cd "$REPO" && prlimit --as=12000000000 timeout 7200 "$BIN" --cells "$cell" --basis "$basis" --twists "$tw" --draws "$draws" --seed 20261018 --budget-secs 1200 --target --no-count) > "$name.jsonl" 2> "$name.log"
  rc=$?  # read before any command substitution: the queue logs of run-20261018T1645Z record date's status, not this one
      echo "$(date -u +%FT%TZ) exit $rc $name" | tee -a "$OUT/queue.log"
    done
  done
done
