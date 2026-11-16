#!/bin/sh
# run.sh -- driver for E1-mixed-solution-census (PREREGISTRATION.md section 9, verbatim commands)
D=${E1_DIR:-/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E1-mixed-solution-census}   # E1_DIR overrides (reduced replication)
cd $D
mkdir -p raw
ln -sfn raw out
sha256sum e1_census.gp e1_aggregate.gp plot_e1.py > raw/script_hashes.txt        # before any run
# Stage 0: oracle agreement V0
E1_STAGE=oracle E1_SEED=20261021 gp -q e1_census.gp < /dev/null > raw/stage0_oracle.out 2> raw/stage0_oracle.err || { echo "STAGE0 FAILED"; exit 1; }
# Stage 1: positive and negative controls
E1_STAGE=controls E1_SEED=20261021 gp -q e1_census.gp < /dev/null > raw/stage1_controls.out 2> raw/stage1_controls.err || { echo "STAGE1 FAILED"; exit 1; }
# Stage 2: grid (<=2 concurrent jobs; each job <=3 configs x ~85 s)
for bits in ${E1_BITS_LIST:-12 16 20 24}; do for pidx in 0 1; do
  if [ $bits = 24 ] && [ $pidx = 1 ]; then continue; fi
  for curve in 0 1 2 3; do for m in 2 3; do for fb in interval random twistmult farey; do
    echo "$bits $pidx $curve $m $fb"; done; done; done; done; done > raw/joblist.txt
cat raw/joblist.txt | xargs -P 2 -L 1 sh -c 'E1_STAGE=grid E1_SEED=20261021 E1_BITS=$0 E1_PIDX=$1 E1_CURVE=$2 E1_M=$3 E1_FB=$4 E1_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) E1_GUARD_S=60 gp -q e1_census.gp < /dev/null > raw/grid_$0_$1_$2_m$3_$4.out 2> raw/grid_$0_$1_$2_m$3_$4.err'
# (E1_REDUCED=1: stop after stage 2; run_reduced.sh compares against the original run)
[ -n "$E1_REDUCED" ] && exit 0
# Stage 3: aggregation and plots (plot only recorded values)
rm -f pooled.tsv controls_summary.tsv verdict.txt results.json
gp -q e1_aggregate.gp < /dev/null > summary.tsv 2> raw/aggregate.err
/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python plot_e1.py summary.tsv
# Stage 4 (added 2026-10-26, not part of the frozen protocol; descriptive re-tabulations, traceability check, operation counts)
sh ./extra_stats.sh summary.tsv > extra_stats.txt
sh ./opcounts.sh summary.tsv > opcounts.tsv
{ for cfg in "3 100 12 0 0 random 1" "2 200 12 0 0 random 1" "3 100 16 0 2 random 1"; do set -- $cfg
  echo "# config m=$1 targets=$2 bits=$3 pidx=$4 curve=$5 fb=$6 alpha_idx=$7"
  E1_STAGE=none E1_M=$1 E1_TARGETS=$2 E1_BITS=$3 E1_PIDX=$4 E1_CURVE=$5 E1_FB=$6 E1_ALPHA_IDX=$7 gp -q e1_census.gp opcount_verify.gp < /dev/null 2>/dev/null | grep '^OPC'; done; } > opcounts_verify.txt
sh ./validate_results.sh $D > validate_results.txt
