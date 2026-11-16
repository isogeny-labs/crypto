#!/bin/bash
# Single driver for L1-descent-parity. Full run: ./run.sh   Reduced replication subset (a few minutes): SIZES="20 24" ./run.sh
# Everything is PARI/GP (gp -q), fixed seeds from PREREGISTRATION.md section 6; outputs in out/, then results.json, RESULTS.md.
set -u
D=$(cd "$(dirname "$0")" && pwd)
export GPDIR=$D/gp
export OUTDIR=${OUTDIR:-$D/out}
SIZES=${SIZES:-"20 24 28 32"}
BOOT=${BOOT:-10000}
JOBS=${JOBS:-3}
GATE_MS=1200000
mkdir -p "$OUTDIR"; rm -f "$OUTDIR"/STOP
reps_for() { # arm size -> number of attempt streams
  case "$1" in
    C) case "$2" in 20|24) echo 10;; 28) echo 5;; *) echo 2;; esac;;
    Dh|Dq) case "$2" in 32) echo 3;; *) echo 10;; esac;;
    *) echo 10;;
  esac
}
gp_() { gp -q "$@" </dev/null 2>&1 | grep -v "Warning: new maximum stack size"; }

echo "stage 0: curves and targets"; for s in $SIZES; do S=$s gp_ "$GPDIR/gen.gp"; done
echo "stage 1: unit tests, controls, stopping rule"; for s in $SIZES; do S=$s gp_ "$GPDIR/controls.gp"; done
if [ -e "$OUTDIR/STOP" ]; then echo "STOP file present: stopping rule fired"; cat "$OUTDIR/STOP"; exit 2; fi

echo "gates (pilot of 2 instances, 1 stream)"
RUN_SIZES=""
for s in $SIZES; do
  run=1
  if [ "$s" = 28 ] || [ "$s" = 32 ]; then
    mkdir -p "$OUTDIR/pilot_$s"; cp "$OUTDIR/curves_$s.csv" "$OUTDIR/pilot_$s/"
    tot=0
    arms="C"; [ "$s" = 32 ] && arms="A B D Dh Dq C"
    for a in $arms; do
      ms=$(OUTDIR="$OUTDIR/pilot_$s" S=$s ARM=$a REPS=1 NMAX=2 gp_ "$GPDIR/cell_ic.gp" | sed -n 's/.*total_ms=\([0-9]*\).*/\1/p')
      tot=$((tot + ${ms:-0})); echo "  s=$s arm=$a pilot_ms=${ms:-0}"
    done
    if [ "$tot" -le "$GATE_MS" ]; then echo "gate s=$s passed: $tot ms <= $GATE_MS" | tee "$OUTDIR/gate_$s.txt"; else echo "gate s=$s FAILED: $tot ms > $GATE_MS: cell not run (budget gate)" | tee "$OUTDIR/gate_$s.txt"; run=0; fi
  fi
  [ "$run" = 1 ] && RUN_SIZES="$RUN_SIZES $s"
done
echo "sizes run: $RUN_SIZES"

echo "stage 2: cells (parallel, $JOBS jobs)"
JOBLIST=$OUTDIR/joblist.txt; : > "$JOBLIST"
for s in $(echo $RUN_SIZES | tr ' ' '\n' | sort -rn); do
  for a in C Dq Dh D B Bp A; do
    echo "S=$s ARM=$a REPS=$(reps_for $a $s) gp -q $GPDIR/cell_ic.gp" >> "$JOBLIST"
  done
  echo "S=$s gp -q $GPDIR/cell_rho.gp" >> "$JOBLIST"
  echo "S=$s gp -q $GPDIR/cell_census.gp" >> "$JOBLIST"
done
cat "$JOBLIST" | xargs -P "$JOBS" -I{} bash -c 'export GPDIR OUTDIR; env {} </dev/null 2>&1 | grep -v "Warning: new maximum stack size"'

echo "stage 3: analysis"
SIZES_CSV=$(echo $RUN_SIZES | tr ' ' ',')
SIZES=$SIZES_CSV BOOT=$BOOT gp_ "$GPDIR/analysis.gp"
if [ "$OUTDIR" = "$D/out" ]; then cp "$OUTDIR/results.json" "$OUTDIR/RESULTS.md" "$D/"; fi
echo "done: $OUTDIR/results.json $OUTDIR/RESULTS.md (full runs also copy them next to run.sh)"
