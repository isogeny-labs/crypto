#!/bin/bash
# Reduced replication (~8-12 min on a quiet machine): Stage 0 / 0b for all m, plus the full m=11 grid (6 l-values x 2 populations),
# written to rep_reduced/ and compared with the recorded out/ files on every deterministic column (all of stage 1; stage 2 columns 1-40, i.e. everything
# except the 12 CPU-time columns). Prints "REDUCED REPLICATION: IDENTICAL" or the diffs. Does not touch out/.
D=$(cd "$(dirname "$0")" && pwd)
cd $D || exit 1
R=rep_reduced; rm -rf $R; mkdir -p $R
( for M in 11 13 17; do M=$M timeout ${TO:-900} gp -q -f scripts/stage0b_fq2.gp </dev/null > $R/stage0b_m$M.txt 2>&1; done
  for M in 11 13 17 19; do M=$M timeout ${TO:-900} gp -q -f scripts/stage0_identities.gp </dev/null > $R/stage0_m$M.txt 2>&1; done ) &
M=11
for L in 4 5 6 7 8 9; do for POP in 1 2; do
  M=$M L=$L POP=$POP timeout ${TO:-900} gp -q -f scripts/stage1_exhaustive.gp </dev/null >> $R/stage1_m$M.csv 2>>$R/stage1_m$M.err
  M=$M L=$L POP=$POP timeout ${TO:-900} gp -q -f scripts/stage2_sieve.gp      </dev/null >> $R/stage2_m$M.csv 2>>$R/stage2_m$M.err
done; done
wait
bad=0
for f in stage0_m11 stage0_m13 stage0_m17 stage0_m19 stage0b_m11 stage0b_m13 stage0b_m17; do cmp -s out/$f.txt $R/$f.txt || { echo "DIFF $f"; bad=1; }; done
diff <(grep -v '^#' out/stage1_m11.csv) <(grep -v '^#' $R/stage1_m11.csv) >/dev/null || { echo "DIFF stage1_m11"; bad=1; }
diff <(grep -v '^#' out/stage2_m11.csv | cut -d, -f1-40) <(grep -v '^#' $R/stage2_m11.csv | cut -d, -f1-40) >/dev/null || { echo "DIFF stage2_m11 (deterministic columns)"; bad=1; }
[ $bad = 0 ] && echo "REDUCED REPLICATION: IDENTICAL"
