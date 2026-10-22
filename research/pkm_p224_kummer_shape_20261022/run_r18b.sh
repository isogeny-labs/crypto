#!/bin/bash
# R18b of PROTOCOL.md section 10 (Amendment 2): the sizing run of the N = 24 system.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=${BIN:?set BIN}
cd "$here/runs"
ulimit -v 5000000
export RAYON_NUM_THREADS=2
echo "$(date -u +%FT%TZ) start R18b-SL24-m2-N24-sizing" >> ../progress.txt
nice -n 10 "$bin" --engine generic --budget 7200 --ladder-t none --planted 0 --random 2 --seed 1347112276 \
  --kinds kummer --p 16760833 --m 2 --controls tower --t-min 12 --max-t 12 --targets 0 \
  --max-nnz 600000000 --trace --out R18b-SL24-m2-N24-sizing.jsonl 2> R18b-SL24-m2-N24-sizing.log
rc=$?
echo "$(date -u +%FT%TZ) end R18b-SL24-m2-N24-sizing exit $rc" >> ../progress.txt
