#!/bin/bash
# E5-kummer-multiplicative-walk driver.  Executes PREREGISTRATION.md section 9 (controls first).  Reproduces every number.
set -u
D=${D:-/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk}
JOBS=${JOBS:-2}   # parallel cells (DEVIATIONS item 18); output is independent of JOBS because every cell is seeded and the skip rule is the frozen skip.list
cd $D
PY=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python
mkdir -p raw out
rm -f out/MISSING.txt
[ -z "${REPLICATE:-}" ] && { sha256sum lib.gp gen_curves.gp check_identity.gp check_exhaustive.gp chainsearch.gp run_cell.gp analyze.gp plot.py cellrun.sh run.sh > FROZEN_SHA256.txt; date -u > FROZEN_AT.txt; }

gp -q -f gen_curves.gp        < /dev/null > raw/curves.txt     2> raw/curves.err
# positive controls first: PC1 (exhaustive small instance, orbit/solvability vs +-H, walks), PC2 (orbit sizes), class partition
if [ -n "${PARALLEL_CONTROLS:-}" ]; then   # fast replication (DEVIATIONS item 18): controls and the a1c supplement run beside the cell pool; they are independent and seeded, so output is unchanged
  { gp -q -f check_exhaustive.gp < /dev/null > raw/exhaustive.txt 2> raw/exhaustive.err; gp -q -f check_identity.gp < /dev/null > raw/identity.txt 2> raw/identity.err; gp -q -f chainsearch.gp < /dev/null > raw/chains.txt 2> raw/chains.err; } &
  CTRL_PID=$!
else
gp -q -f check_exhaustive.gp  < /dev/null > raw/exhaustive.txt 2> raw/exhaustive.err
gp -q -f check_identity.gp    < /dev/null > raw/identity.txt   2> raw/identity.err
gp -q -f chainsearch.gp       < /dev/null > raw/chains.txt     2> raw/chains.err
fi

SUPP=$(grep -o 'SUPPLEMENTARY_INST=[0-9]*' raw/curves.txt | head -1 | cut -d= -f2)
echo "supplementary proper-H instance: b=24 inst=$SUPP" > out/SUPPLEMENTARY.txt
# PC3 positive control cell (pc1: K-pure, a in +-H, proper-H set) before every main cell
printf "pc1 24 1 40\npc1 24 2 40\npc1 24 $SUPP 40\n" | xargs -P $JOBS -L 1 ./cellrun.sh

trials_for_b() { case $1 in 20) echo 80;; 24) echo 80;; 28) echo 60;; 32) echo 40;; 36) echo 24;; 40) echo 12;; esac; }
: > cells.list
for B in 20 24 28 32 36 40; do
  T=$(trials_for_b $B)
  : > cells_b$B.list
  for arm in kpure kmix kpureT a0 a1; do for inst in 1 2; do echo "$arm $B $inst $T" >> cells_b$B.list; done; done
  if [ $B -le 28 ]; then for inst in 1 2; do echo "cross $B $inst 24" >> cells_b$B.list; done; fi
  if [ $B -eq 24 ]; then
    for inst in 1 2 $SUPP; do echo "nc1 24 $inst 40" >> cells_b$B.list; echo "kmixidx 24 $inst 40" >> cells_b$B.list; done
  fi
  if [ $B -eq 28 ]; then for arm in kpure_sa kpure_sc kmix_sa kmix_sc; do echo "$arm 28 1 60" >> cells_b$B.list; done; fi
  cat cells_b$B.list >> cells.list
  # 4 cores shared with another agent: 2 cells at a time, hard limit 840 s per cell (cellrun.sh)
done
# all cells in one pool, longest first (no per-b barrier is needed with the frozen skip.list)
[ -n "${PARALLEL_CONTROLS:-}" ] && { ./run_supp.sh & SUPP_PID=$!; }
sort -s -k2,2nr cells.list | xargs -P $JOBS -L 1 ./cellrun.sh

# supplementary arm a1c (added after the main run, DEVIATIONS.md item 11), then the final analysis (analyze.gp v2, DEVIATIONS.md item 12)
if [ -n "${PARALLEL_CONTROLS:-}" ]; then wait $CTRL_PID; wait $SUPP_PID; else ./run_supp.sh; fi
gp -q -f analyze.gp < /dev/null > out/summary.txt 2> out/analyze.err
$PY plot.py out/summary.csv out/costs.csv
# operation-count table from the raw records and final results.json (DEVIATIONS.md item 15)
gp -q -f ops_table.gp < /dev/null > out/opcounts_pooled.txt 2> out/opcounts.err
gp -q -f ops_ratio.gp < /dev/null 2>&1 | grep -v Warn > out/opcounts_ratios.txt
./build_results_json.sh
ls raw out > out/FILES.txt
rm -rf target
echo done
