#!/usr/bin/env bash
# Raised-cap pass (deviation recorded in RESULTS.md): the registered 1 GiB F4
# step-matrix cap left every (16,4) target-free draw unmeasured at degree 5, so
# the same arms are rerun with the cap at 8 GiB (F4_F2_MAX_MATRIX_WORDS = 2^30)
# and a 3,600 s budget, under the same 12 GB address-space limit.  (16,4) only;
# gp plain, gp twisted, random plain (one draw, the same first draw as runs/).
set -u
BIN=${1:-target/release/examples/dreg_target_free}
HERE=$(cd "$(dirname "$0")" && pwd); REPO=$(cd "$HERE/../.." && pwd); OUT=$HERE/runs_highcap
mkdir -p "$OUT"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 F4_F2_MAX_MATRIX_WORDS=$((1<<30)) RAYON_NUM_THREADS=3
for spec in "gp 0,0,0,0" "gp 0,1,2,3" "random 0,0,0,0"; do
  set -- $spec; basis=$1; tw=$2
  name="$OUT/cell-16-4.$basis.tw${tw//,/}"
  if [ -s "$name.jsonl" ]; then echo "skip $name"; continue; fi
  echo "$(date -u +%FT%TZ) start $name" | tee -a "$OUT/queue.log"
  (cd "$REPO" && prlimit --as=12000000000 timeout 7200 "$BIN" --cells 16:4 --basis "$basis" --twists "$tw" --draws 1 --seed 20261018 --budget-secs 3600 --target --no-count) > "$name.jsonl" 2> "$name.log"
  rc=$?  # read before any command substitution: the queue logs of run-20261018T1645Z record date's status, not this one
  echo "$(date -u +%FT%TZ) exit $rc $name" | tee -a "$OUT/queue.log"
done
