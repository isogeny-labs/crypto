#!/bin/bash
# usage: cellrun.sh ARM B INST TRIALS   (cwd = experiment directory; RAWDIR defaults to raw)
# DEVIATIONS.md item 18: the preregistered "skip if the same arm at the next-smaller b exceeded 200 s wall time" rule (PREREGISTRATION 9) is applied
# DETERMINISTICALLY: the cells that rule skipped in the original run are frozen in skip.list (a1 b40 i1/i2, after a1 b36 took 429 s / 512 s); the wall-clock
# observation is only logged. The hard per-cell limit is CELL_TIMEOUT (default 3000 s, a safety net far above any observed cell time; the original used 840 s and no cell hit it).
ARM=$1; B=$2; INST=$3; TRIALS=$4
R=${RAWDIR:-raw}
MISS=${OUTDIR:-out}/MISSING.txt
PREV=$((B-4))
if grep -qxF "$ARM $B $INST" skip.list 2>/dev/null; then
  echo "$ARM b$B i$INST SKIPPED: frozen skip.list (original run: prior cell b$PREV took >200s)" >> $MISS
  exit 0
fi
S=$(date +%s)
ARM=$ARM B=$B INST=$INST TRIALS=$TRIALS timeout ${CELL_TIMEOUT:-3000} gp -q -f run_cell.gp < /dev/null > $R/cell_${ARM}_b${B}_i${INST}.txt 2> $R/cell_${ARM}_b${B}_i${INST}.err
RC=$?
E=$(date +%s)
echo $((E-S)) > $R/time_${ARM}_b${B}_i${INST}.sec
if [ $RC -ne 0 ]; then echo "$ARM b$B i$INST rc=$RC after $((E-S))s (timeout 124 = killed at ${CELL_TIMEOUT:-3000} s; records written before the kill are kept)" >> $MISS; fi
exit 0
