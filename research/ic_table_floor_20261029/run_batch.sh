#!/bin/bash
# The timing batch: for each size and repetition the old and the new binary run the same spec
# back to back, each under its own exclusive CPU lock (released between sessions).
# Binaries: /dev/shm/bins/ecbench-tf-old (the base of this branch) and ecbench-tf-new.
# No new session starts once DEADLINE (epoch seconds) has passed.
OUT=${OUT:-/dev/shm/out-tf}
DEADLINE=${DEADLINE:?set DEADLINE}
cd "$(dirname "$0")"
mkdir -p "$OUT"
run() {  # run <old|new> <size> <rep>
  local name=ab-$1-$2-$3
  [ "$(date +%s)" -gt "$DEADLINE" ] && { echo "CUT $name (deadline)"; return; }
  [ -d "$OUT/$name" ] && return
  flock -x /dev/shm/cpu.lock /dev/shm/bins/ecbench-tf-$1 run --spec "specs/ab-$2.json" --out "$OUT/$name" --allow-busy > "$OUT/$name.log" 2>&1
  echo "$name $(tail -1 "$OUT/$name.log" | cut -c1-90)"
}
for rep in 1 2 3; do
  for bits in 28 24 32; do
    run old $bits $rep
    run new $bits $rep
  done
done
echo BATCH-DONE
