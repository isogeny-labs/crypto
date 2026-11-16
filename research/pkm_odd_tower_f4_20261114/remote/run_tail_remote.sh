#!/usr/bin/env bash
# EXP-PKM-ODD-001 Amendment 4: the censored m = 3 cells on a high-memory host.
# Runs in the pod's /work with the two uploaded binaries and censor_rows.py.
# 1. Host-equivalence: three committed cells rerun; their counts must equal the committed
#    ones (checked locally after download, and here by expected values) before step 2 counts.
# 2. The six cells censored for memory on the 15 GiB host, same arguments as run_cells.sh.
set -uo pipefail
cd /work
export RAYON_NUM_THREADS=4
P1=1399680001; SEED=20261114; BUDGET=1800; DMAX=64
mkdir -p runs
sha256sum pkm_odd_tower_f4 pkm_algebraic_oracle > runs/binary.sha256
nproc > runs/host.txt; grep -E "MemTotal|model name" /proc/meminfo /proc/cpuinfo | head -3 >> runs/host.txt
echo "start $(date -u +%FT%TZ)" >> runs/progress.txt

run() {  # name rows args...
  local name="$1" rows="$2"; shift 2; local f="runs/$name.jsonl" t0; t0=$(date +%s)
  ./pkm_odd_tower_f4 --seed $SEED --budget $BUDGET --dmax $DMAX --cell "$name" --out "$f" "$@" > /dev/null
  local rc=$?
  if [ $rc -ne 0 ]; then
    python3 censor_rows.py A "$f" "$name" "$rows" "instrument exit $rc on the high-memory host" -- "$@"
    echo "censored $name (exit $rc)" >> runs/progress.txt
  fi
  echo "done $name rows=$(wc -l < "$f") secs=$(( $(date +%s) - t0 ))" >> runs/progress.txt
}
runb() {  # name args...
  local name="$1"; shift; local f="runs/B-$name.jsonl" t0; t0=$(date +%s)
  ./pkm_algebraic_oracle --seed $SEED --budget $BUDGET --dmax $DMAX --cell "B-$name" --out "$f" --planted 2 --random 2 "$@" > /dev/null
  local rc=$?
  if [ $rc -ne 0 ]; then
    python3 censor_rows.py B "$f" "B-$name" 4 "instrument exit $rc on the high-memory host" -- "$@"
    echo "censored B-$name (exit $rc)" >> runs/progress.txt
  fi
  echo "done B-$name rows=$(wc -l < "$f") secs=$(( $(date +%s) - t0 ))" >> runs/progress.txt
}

# 1. host-equivalence cells (names prefixed HE-, never read as study rows)
run HE-M2-A3-k5-tower 3 --p $P1 --m 2 --degrees 3,3,3,3,3 --control tower --kind refute --draws 3
run HE-M3-A2-t4-tower 3 --p $P1 --m 3 --degrees 2,2,2,2 --control tower --kind refute --draws 3
runb HE-M3-A2-t4 --p $P1 --m 3 --form kummer --degrees 2,2,2,2

# 2. the censored cells
run M3-A2-t5-tower 3 --p $P1 --m 3 --degrees 2,2,2,2,2 --control tower --kind refute --draws 3
run M3-A2-t5-null 3 --p $P1 --m 3 --degrees 2,2,2,2,2 --control null --kind refute --draws 3
run M3-A23-n4-tower 3 --p $P1 --m 3 --degrees 2,3,2,3 --control tower --kind refute --draws 3
run M3-A23-n4-null 3 --p $P1 --m 3 --degrees 2,3,2,3 --control null --kind refute --draws 3
runb M3-A23-n4 --p $P1 --m 3 --form kummer --degrees 2,3,2,3
runb M3-A2-t5 --p $P1 --m 3 --form kummer --degrees 2,2,2,2,2
echo "end $(date -u +%FT%TZ)" >> runs/progress.txt
