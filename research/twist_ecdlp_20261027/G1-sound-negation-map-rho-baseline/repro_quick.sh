#!/bin/bash
# Reduced replication of G1 (target: under 20 minutes on a contended machine).  Re-runs 7 of the 66 cells byte-for-byte (the walk code is deterministic under the
# fixed seeds; only the timer fields may differ) into repro/, compares them with raw/, then re-runs the frozen analysis and both addenda on the existing raw data
# in a scratch copy and compares summary.txt, solve.csv, ratios.csv, opcounts.txt and results.json byte-for-byte with out/.  Full run: bash run.sh.
set -u
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G1-sound-negation-map-rho-baseline
cd $D
sha256sum -c FROZEN_SHA256.txt > /dev/null 2>&1 || { echo "FROZEN FILE MISMATCH"; exit 2; }
rm -rf repro; mkdir -p repro/cells
n=0; bad=0
run() { # file-name ENV... ; runs the cell script and diffs against raw/ ignoring the done_ms header and the per-trial wall-ms field (field 19 of each rec, a timer)
  f=$1; shift; scr=$1; shift
  env "$@" timeout 840 gp -q -f $scr < /dev/null > repro/cells/$f 2> /dev/null
  if diff <(grep -v done_ms raw/$f | sed -E 's/, [0-9]+, ([0-9]+)\]\);$/, MS, \1]);/') <(grep -v done_ms repro/cells/$f | sed -E 's/, [0-9]+, ([0-9]+)\]\);$/, MS, \1]);/') > /dev/null; then echo "IDENTICAL $f"; else echo "DIFFERS   $f"; bad=$((bad+1)); fi
}
run cell_n1_b24_i1_c1.txt     g1_run_cell.gp ARM=n1    B=24 INST=1 TLO=1 THI=80 TOTAL=80
run cell_n1_b28_i1_c1.txt     g1_run_cell.gp ARM=n1    B=28 INST=1 TLO=1 THI=80 TOTAL=80
run cell_n1_b32_i1_c1.txt     g1_run_cell.gp ARM=n1    B=32 INST=1 TLO=1 THI=60 TOTAL=60
run cell_n1sab_b24_i1_c1.txt  g1_run_cell.gp ARM=n1sab B=24 INST=1 TLO=1 THI=30 TOTAL=30
run cell_n1c2_b28_i1_c1.txt   g1_run_cell.gp ARM=n1c2  B=28 INST=1 TLO=1 THI=30 TOTAL=30
run cell_kmix_b24_i3.txt      run_cell.gp    ARM=kmix  B=24 INST=3 TRIALS=40
run cell_a0_b28_i3.txt        run_cell.gp    ARM=a0    B=28 INST=3 TRIALS=40
# analysis re-run on the existing raw data in a scratch copy
mkdir -p repro/an/out; cp *.gp repro/an/; ln -s $D/raw repro/an/raw; ln -s $D/e5_inputs repro/an/e5_inputs
( cd repro/an; : > out/solve.csv; : > out/ratios.csv
  gp -q -f analyze_g1.gp < /dev/null > out/summary.txt 2> /dev/null
  gp -q -f opcounts_g1.gp < /dev/null > out/opcounts.txt 2> /dev/null
  cp $D/out/RC.txt $D/PREREG_SHA256.txt . 2>/dev/null; cp $D/out/RC.txt out/RC.txt; cp $D/PREREG_SHA256.txt .
  gp -q -f make_results_g1.gp < /dev/null > /dev/null 2>&1 )
for f in out/summary.txt out/solve.csv out/ratios.csv out/opcounts.txt out/opcounts.csv; do
  if cmp -s $f repro/an/$f; then echo "IDENTICAL $f"; else echo "DIFFERS   $f"; bad=$((bad+1)); fi; done
if cmp -s results.json repro/an/results.json; then echo "IDENTICAL results.json"; else echo "DIFFERS   results.json"; bad=$((bad+1)); fi
echo "quick replication: $bad differences"
