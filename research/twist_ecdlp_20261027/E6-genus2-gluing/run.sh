#!/bin/bash
# E6 single driver (v2).  Same commands, seeds and grid as the frozen run_all.sh; the ORDER differs
# (controls, incl. the positive control PC1, first; main runs after).  Each p is seeded independently
# (setrand(SEED+p) / setrand(SEED+p+7)) so order does not change any number.
#   bash run.sh                      full run (~30 min wall under contention; the p=101 main run ends at its 900 s timeout, see RESULTS.md)
#   E6_REDUCED=1 E6_OUT=out_reduced bash run.sh   reduced replication (~10-15 min): controls p=101,211,409,1009; main p=409,1009,4001,10007,20011
#   E6_REDUCED=2 E6_OUT=out_quick bash run.sh     quick subset: controls p=101,211,409,1009; main p=409,1009,4001 (about 6 min wall under load)
#   E6_POSTONLY=1 [E6_OUT=...] bash run.sh   re-derive summary, totals, Wilson, opcounts, cross-checks from existing raw files (seconds)
# After either run: bash run.sh writes out/summary.txt, totals.txt, wilson_totals.txt, opcounts.txt, SHA256SUMS_v2.txt;
# reduced output is byte-identical to the corresponding full-run files (compare with cmp, see RESULTS.md).
set -u
HERE="$(dirname "$(readlink -f "$0")")"; cd "$HERE"
OUT=${E6_OUT:-out}; RED=${E6_REDUCED:-0}; SEED=20261021
mkdir -p $OUT
run_main() { # p N BRUTE
  E6_P=$1 E6_N=$2 E6_BRUTE=$3 E6_SEED=$SEED timeout 900 gp -q e6_main.gp </dev/null > $OUT/main_p$1.txt 2> $OUT/main_p$1.err
  echo "main p=$1 rc=$?" >> $OUT/rc.txt
}
run_ctrl() { # p NPOS NNEG NNULL
  E6_P=$1 E6_NPOS=$2 E6_NNEG=$3 E6_NNULL=$4 E6_SEED=$SEED timeout 900 gp -q e6_controls.gp </dev/null > $OUT/ctrl_p$1.txt 2> $OUT/ctrl_p$1.err
  echo "ctrl p=$1 rc=$?" >> $OUT/rc.txt
}
if [ "${E6_POSTONLY:-0}" != 1 ]; then
: > $OUT/rc.txt
run_ctrl 101  300 300 3000
run_ctrl 211  300 300 3000
run_ctrl 409  300 300 2000
run_ctrl 1009 300 300 1000
if [ "$RED" = 0 ]; then
  run_ctrl 4001 100 100 200
  run_main 101   20 4     # 16 distinct j exist for pattern [1,1,1]: sampler loops until the 900 s timeout (rc=124), all records already printed
  run_main 211   20 3
fi
run_main 409   15 0
run_main 1009  10 0
run_main 4001   5 0
[ "$RED" = 2 ] || run_main 10007  4 0
[ "$RED" = 2 ] || run_main 20011  1 0
fi   # end of experiment runs (E6_POSTONLY=1 skips them and only re-derives summary/totals/opcounts/xcheck from the raw files)
# summary + helpers (read only the raw files)
E6_FILES=$(ls $OUT/main_p*.txt | paste -sd,) E6_CFILES=$(ls $OUT/ctrl_p*.txt | paste -sd,) \
  timeout 600 gp -q e6_summary.gp </dev/null > $OUT/summary.txt 2> $OUT/summary.err
SRC=$?; [ "${E6_POSTONLY:-0}" = 1 ] || echo "summary rc=$SRC" >> $OUT/rc.txt
PS=$(ls $OUT/main_p*.txt | sed 's/.*main_p//; s/\.txt//' | sort -n | paste -sd,)
E6_OUT=$OUT E6_PS=$PS bash e6_opcounts.sh > $OUT/opcounts.txt
# post-hoc independent cross-checks (own Legendre-symbol counts; exhaustive distinct-j counts; mu-scan with seed 777)
XD=$OUT/xcheck; mkdir -p $XD
for p in 4001 10007 20011; do [ -f $OUT/main_p$p.txt ] && awk '$1=="A"{for(i=1;i<=NF;i++){split($i,k,"=");v[k[1]]=k[2]} print v["A"],v["B"],v["nE"],v["nEt"],v["t"]}' $OUT/main_p$p.txt > $XD/raw_$p.txt; done
(cd $XD && for i in 1 2 3; do gp -q "$HERE/e6_xcheck$i.gp" </dev/null > xcheck$i.out 2> xcheck$i.err; done)

sha256sum e6lib.gp e6_main.gp e6_controls.gp e6_summary.gp e6_opcounts.sh run.sh $OUT/main_p*.txt $OUT/ctrl_p*.txt $OUT/summary.txt $OUT/opcounts.txt > $OUT/SHA256SUMS_v2.txt
# pooled totals and Wilson intervals (derived from summary.txt only)
E6_OUT=$OUT bash e6_totals.sh
g() { grep "^TOTAL $1=" $OUT/totals.txt | cut -d= -f2; }
PAIRS="$(g curves)/$(g A_all_ok),$(g C_target_n)/$(g match_target),$(g N1_eq_p+1_target)/$(g N1_eq_p+1_target),$(g C2_n)/$(g brute_eq_pari),$(grep -o 'POS_n=[0-9]*' $OUT/totals.txt | cut -d= -f2)/$(grep -o 'POS=[0-9]*' $OUT/totals.txt | head -1 | cut -d= -f2)"
E6_PAIRS=$PAIRS gp -q e6_wilson.gp </dev/null > $OUT/wilson_totals.txt 2>&1
