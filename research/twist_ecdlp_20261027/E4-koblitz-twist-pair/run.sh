#!/bin/bash
# Driver for E4-koblitz-twist-pair: reproduces every number. Run from anywhere: bash run.sh
# Commands follow PREREGISTRATION.md section 9 (only addition: stdin from /dev/null so gp exits; scripts end with quit).
# Amended driver, see DEVIATIONS.md D1-D3 (the .gp scripts and the preregistration are untouched):
#   TO    per-process timeout in seconds (default 900 = preregistered; use TO=3600 on a contended machine so a slow
#         machine cannot truncate a cell; every scientific column is seed-deterministic, only CPU-time columns move).
#   LANES 2 = preregistered concurrency; 4 = stage0/0b lane + m=19 | m=17 | m=13,m=11 lanes (default, ~18-20 min quiet).
#   D is derived from the location of this script, so a copy of the directory runs (and rm -f out/*) in the copy only.
D=$(cd "$(dirname "$0")" && pwd)
TO=${TO:-900}
LANES=${LANES:-4}
PY=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python
cd $D || exit 1
rm -f out/* results.json
sha256sum PREREGISTRATION.md scripts/*.gp > out/FROZEN.sha256     # before any stage runs
grid() { case $1 in 11) echo 4 5 6 7 8 9;; 13) echo 5 6 7 8 9 10;; 17) echo 6 7 8 9 10;; 19) echo 7 8 9 10;; esac; }
runcell() { # $1=M ; runs all cells for one m, sequentially
  M=$1
  for L in $(grid $M); do for POP in 1 2; do
    M=$M L=$L POP=$POP timeout $TO gp -q -f scripts/stage1_exhaustive.gp </dev/null >> out/stage1_m$M.csv 2>>out/stage1_m$M.err
    echo "stage1 M=$M L=$L POP=$POP exit=$?" >> out/status_m$M.txt
    M=$M L=$L POP=$POP timeout $TO gp -q -f scripts/stage2_sieve.gp      </dev/null >> out/stage2_m$M.csv 2>>out/stage2_m$M.err
    echo "stage2 M=$M L=$L POP=$POP exit=$?" >> out/status_m$M.txt
  done; done
}
stage0() {
  # positive control PC3 (F_{q^2}, detector sees rational sums) first, then Stage 0 gates
  for M in 11 13 17; do M=$M timeout $TO gp -q -f scripts/stage0b_fq2.gp </dev/null > out/stage0b_m$M.txt 2>&1; done
  for M in 11 13 17 19; do M=$M timeout $TO gp -q -f scripts/stage0_identities.gp </dev/null > out/stage0_m$M.txt 2>&1; done
}
if [ "$LANES" = 2 ]; then
  stage0
  ( runcell 11; runcell 19 ) &
  ( runcell 13; runcell 17 ) &
  wait
else
  stage0 &
  runcell 19 &
  runcell 17 &
  ( runcell 13; runcell 11 ) &
  wait
fi
# completeness gate: a non-zero exit (e.g. 124 = timeout) means a cell is missing and the replication is NOT complete
if grep -v 'exit=0$' out/status_m*.txt | grep -q .; then
  echo "INCOMPLETE: non-zero exit in" ; grep -v 'exit=0$' out/status_m*.txt
  echo "Re-run with a larger TO (e.g. TO=3600 bash run.sh); the verdict below is NOT a replication."
  RC=1
fi
timeout $TO gp -q -f scripts/analyze.gp </dev/null > out/verdict.txt 2>&1
$PY scripts/plot_e4.py > out/plot.log 2>&1
sha256sum out/*.csv out/*.txt > out/RESULTS.sha256
bash $D/run_supp.sh                                   # exploratory supplement + operation counts (outputs out/supp_*.txt, out/opcounts.json)
echo "run.sh done (RC=${RC:-0})"
exit ${RC:-0}
