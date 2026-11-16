#!/bin/bash
# LEADS.md L4: the m = 3, N = 21 PKM Kummer tower systems of ic_next10 item 3, rerun under a 14 GB address-space cap and a 6 h wall (the 2026-10-14 runs stopped at the 45 min cap inside the second degree-7 step).
#   ./run.sh NAME [NAME ...]     # NAME = systems/NAME.ms
# Each run: nice, one thread, -v 2, a 5 GB address-space cap (prlimit --as),
# a 45 min wall cap (timeout). Writes runs/NAME.log (msolve's -v 2 trace),
# runs/NAME.out (msolve's answer) and runs/NAME.mem (peak VmHWM/VmPeak in kB,
# sampled from /proc every second, plus the wall seconds and exit status).
# progress.txt records start, end and exit status. A run that ended is not repeated.
set -u
cd "$(dirname "$0")"
export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1
CAP=${CAP:-14000000000}
WALL=${WALL:-21600}
for name in "$@"; do
  if grep -q " end $name exit" progress.txt 2>/dev/null; then continue; fi
  echo "$(date -u +%FT%TZ) start $name cap=$CAP wall=$WALL" >> progress.txt
  t0=$(date +%s.%N)
  nice prlimit --as=$CAP -- timeout $WALL \
    ${MSOLVE:-msolve} -t 1 -v 2 -f systems/$name.ms -o runs/$name.out > runs/$name.log 2>&1 &
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
  echo "VmHWM_kB=$hwm VmPeak_kB=$vpk wall_s=$(echo "$t1 - $t0" | bc) exit=$rc" > runs/$name.mem
  echo "$(date -u +%FT%TZ) end $name exit $rc" >> progress.txt
done
