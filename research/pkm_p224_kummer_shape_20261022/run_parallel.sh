#!/bin/bash
# R16n, R17 and R19 of PROTOCOL.md 2.3 with run_f4.sh's exact flags, started in
# parallel with the main batch to use idle cores (the main script is stopped
# before it reaches them; see README "Deviations"). Usage: BIN=... run_parallel.sh R16n|R17|R19
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=${BIN:?set BIN}
cd "$here/runs"
ulimit -v 4000000
export RAYON_NUM_THREADS=1
SL26=67104769; SL31=2147352577
case $1 in
  R17) name=R17-SL31-m3-N15; args=(--p $SL31 --m 3 --controls tower --t-min 5 --max-t-m3 5 --targets 0);;
  R16n) name=R16n-SL24-m2-N22-null; args=(--p 16760833 --m 2 --controls null --t-min 11 --max-t 11 --targets 0);;
  R19) name=R19-SL26-m2-N22; args=(--p $SL26 --m 2 --controls tower --t-min 11 --max-t 11 --targets 0);;
esac
echo "$(date -u +%FT%TZ) start $name (parallel): ${args[*]}" >> ../progress.txt
nice -n 10 "$bin" --engine generic --budget 7200 --ladder-t none --planted 0 --random 2 \
  --seed 1347112276 --kinds kummer "${args[@]}" --out "$name.jsonl" 2> "$name.log"
rc=$?
echo "$(date -u +%FT%TZ) end $name exit $rc (parallel)" >> ../progress.txt
