#!/bin/bash
# Amendment A1 (PROTOCOL.md §"Amendment A1"): run.sh with msolve multithreaded.
# The only differences from the frozen run.sh: `-t $THREADS` and
# OMP_NUM_THREADS=$THREADS in place of 1, records in runs-mt/ and
# progress-mt.txt (so the first run's records are never touched or skipped
# into), and the wall seconds computed with awk when bc is absent.
#   THREADS=8 CAP=… WALL=… ./run_mt.sh NAME [NAME ...]     # NAME = systems/NAME.ms
set -u
cd "$(dirname "$0")"; SYS=${SYS:-../ic_next10_20261014/03-pkm-degree/systems}; mkdir -p runs-mt
THREADS=${THREADS:?THREADS}
export OMP_NUM_THREADS=$THREADS RAYON_NUM_THREADS=1
CAP=${CAP:-5000000000}
WALL=${WALL:-2700}
for name in "$@"; do
  if grep -q " end $name exit" progress-mt.txt 2>/dev/null; then continue; fi
  echo "$(date -u +%FT%TZ) start $name cap=$CAP wall=$WALL threads=$THREADS" >> progress-mt.txt
  t0=$(date +%s.%N)
  nice prlimit --as=$CAP -- timeout $WALL \
    msolve -t $THREADS -v 2 -f $SYS/$name.ms -o runs-mt/$name.out > runs-mt/$name.log 2>&1 &
  pid=$!
  hwm=0; vpk=0
  while kill -0 $pid 2>/dev/null; do
    for c in $(pgrep -P $pid) $pid; do
      for g in $(pgrep -P $c) $c; do
        if [ -r /proc/$g/status ] && grep -q '^Name:.*msolve' /proc/$g/status 2>/dev/null; then
          h=$(awk '/^VmHWM/{print $2}' /proc/$g/status 2>/dev/null); v=$(awk '/^VmPeak/{print $2}' /proc/$g/status 2>/dev/null)
          [ -n "$h" ] && [ "$h" -gt "$hwm" ] && hwm=$h
          [ -n "$v" ] && [ "$v" -gt "$vpk" ] && vpk=$v
        fi
      done
    done
    sleep 1
  done
  wait $pid; rc=$?
  t1=$(date +%s.%N)
  echo "VmHWM_kB=$hwm VmPeak_kB=$vpk wall_s=$(awk -v a="$t0" -v b="$t1" 'BEGIN{printf "%.1f", b-a}') exit=$rc threads=$THREADS" > runs-mt/$name.mem
  echo "$(date -u +%FT%TZ) end $name exit $rc" >> progress-mt.txt
done
