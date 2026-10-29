#!/bin/bash
# The timing batch of the extension study. Each session runs under its own exclusive lock so the lock
# is released between sessions; /dev/shm/timing.want (raised by the caller) is what holds the other fork off.
# No new session starts once DEADLINE (epoch seconds) has passed.
OUT=${OUT:-/dev/shm/out-l2}
DEADLINE=${DEADLINE:?set DEADLINE}
cd "$(dirname "$0")"
mkdir -p "$OUT"
run() {  # run <binary-tag> <spec> <name> [diag]
  [ "$(date +%s)" -gt "$DEADLINE" ] && { echo "CUT $3 (deadline)"; return; }
  [ -d "$OUT/$3" ] && return
  [ "$4" = diag ] && touch /dev/shm/lane_diag.on
  flock -x /dev/shm/cpu.lock /dev/shm/bins/ecbench-l2-$1 run --spec "specs/$2.json" --out "$OUT/$3" --allow-busy > "$OUT/$3.log" 2>&1
  rm -f /dev/shm/lane_diag.on
  echo "$3 $(tail -1 "$OUT/$3.log" | cut -c1-90)"
}
run split diag-28 split-diag-28-1 diag
for rep in 1 2 3; do run diag abfast-28 ab-old-$rep; run split abfast-28 ab-new-$rep; done
run split diag-32 split-diag-32-1 diag
for rep in 1 2; do run split grid-28 grid-28-$rep; done
for rep in 1 2; do run split grid-32 grid-32-$rep; done
echo BATCH-DONE
