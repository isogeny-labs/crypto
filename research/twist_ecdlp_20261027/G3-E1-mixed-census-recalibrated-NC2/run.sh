#!/bin/bash
# run.sh -- driver for G3-E1-mixed-census-recalibrated-NC2 (PREREGISTRATION.md section 10 commands; deviations in DEVIATIONS.md)
# Resumable: a job whose output already ends with its completion marker is skipped (same seeds => same bytes).
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G3-E1-mixed-census-recalibrated-NC2
PY=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python
cd $D && mkdir -p raw && ln -sfn raw out
# the preregistration must be unchanged since freezing
EXPECT=e8fc1a52d7fc73d8f27e9322ed5e586ac3bb64d0c9e739849ed73466b16773fd
[ "$(sha256sum PREREGISTRATION.md | cut -d' ' -f1)" = "$EXPECT" ] || { echo "PREREGISTRATION CHANGED"; exit 1; }
sha256sum PREREGISTRATION.md > PREREGISTRATION.sha256
if [ ! -f raw/script_hashes.txt ]; then
  sha256sum g3_census.gp g3_aggregate.gp plot_g3.py run.sh > SCRIPTS.sha256 ; cp SCRIPTS.sha256 raw/script_hashes.txt   # before any run
fi
export G3_SEED=20261026
# ---- Stage 0 (G0)
if ! grep -q '^V0_RESULT' raw/stage0_oracle.out 2>/dev/null; then
  G3_STAGE=oracle gp -q g3_census.gp < /dev/null > raw/stage0_oracle.out 2> raw/stage0_oracle.err || { echo "STAGE0 (G0) FAILED"; exit 1; }
fi
# ---- Stage 1 controls: positive controls first (PC1-PC3 per bit size), then NC1 per bit size, then NC3 (split by bit size only to keep each run < 15 min; same code, same seed)
for b in 12 16 20 24; do echo "$b"; done | xargs -P 2 -I{} sh -c 'f=raw/stage1_pc_{}.out; grep -q "^CONTROLS_RESULT" $f 2>/dev/null || G3_STAGE=controls G3_CPART=pc G3_CBITS={} gp -q g3_census.gp < /dev/null > $f 2> raw/stage1_pc_{}.err'
for b in 12 16 20 24; do grep -q "^CONTROLS_RESULT.*PCbad=0" raw/stage1_pc_$b.out || { echo "G1 (positive control) FAILED or missing at $b bits"; exit 1; }; done
for b in 12 16 20 24; do echo "nc1 $b"; done > raw/c1list.txt; echo "nc3 12" >> raw/c1list.txt
cat raw/c1list.txt | xargs -P 2 -L 1 sh -c 'f=raw/stage1_$0_$1.out; grep -q "^CONTROLS_RESULT" $f 2>/dev/null || G3_STAGE=controls G3_CPART=$0 G3_CBITS=$1 gp -q g3_census.gp < /dev/null > $f 2> raw/stage1_$0_$1.err'
cat raw/stage1_pc_12.out raw/stage1_pc_16.out raw/stage1_pc_20.out raw/stage1_pc_24.out raw/stage1_nc1_12.out raw/stage1_nc1_16.out raw/stage1_nc1_20.out raw/stage1_nc1_24.out raw/stage1_nc3_12.out > raw/stage1_controls.out
# ---- Stage 2 main grid, 2 concurrent workers (4 cores shared)
for b in 12 16 20 24; do K=$(case $b in 12|16) echo 4;; 20) echo 3;; 24) echo 2;; esac)
 for p in $(seq 0 $((K-1))); do for c in $(seq 0 11); do for m in 2 3; do for fb in interval random twistmult farey; do echo "$b $p $c $m $fb"; done; done; done; done; done > raw/joblist.txt
xargs -P 2 -L 1 sh -c 'f=raw/grid_$0_$1_$2_m$3_$4.out; grep -q "^JOBEND" $f 2>/dev/null || G3_STAGE=grid G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=$3 G3_FB=$4 G3_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) G3_GUARD_S=120 gp -q g3_census.gp < /dev/null > $f 2> raw/grid_$0_$1_$2_m$3_$4.err' < raw/joblist.txt
# ---- Stage 2b NC2-calibrated (cidx 0..5, alpha 0.65, same targets)
for b in 12 16 20 24; do K=$(case $b in 12|16) echo 4;; 20) echo 3;; 24) echo 2;; esac)
 for p in $(seq 0 $((K-1))); do for c in 0 1 2 3 4 5; do for m in 2 3; do for fb in interval random twistmult farey; do echo "$b $p $c $m $fb"; done; done; done; done; done > raw/nc2list.txt
xargs -P 2 -L 1 sh -c 'f=raw/nc2_$0_$1_$2_m$3_$4.out; grep -q "^NCEND" $f 2>/dev/null || G3_STAGE=nc2 G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=$3 G3_FB=$4 G3_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) gp -q g3_census.gp < /dev/null > $f 2> raw/nc2_$0_$1_$2_m$3_$4.err' < raw/nc2list.txt
# ---- Stage 2c resolution cells (m=3): "bits pidx cidx fb block ntargets guard"
( for p in 0 1; do for c in 2 5; do for fb in random farey twistmult; do for j in 0 1; do echo "20 $p $c $fb $j 10000 800"; done; done; done; done
  for c in 2 5; do for j in 0 1 2 3 4 5; do echo "24 0 $c random $j 60000 850"; done; done ) > raw/reslist.txt
xargs -P 2 -L 1 sh -c 'f=raw/res_$0_$1_$2_$3_b$4.out; grep -q "^REND" $f 2>/dev/null || G3_STAGE=res G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=3 G3_FB=$3 G3_BLOCK=$4 G3_TARGETS=$5 G3_GUARD_S=$6 gp -q g3_census.gp < /dev/null > $f 2> raw/res_$0_$1_$2_$3_b$4.err' < raw/reslist.txt
# ---- Stage G5 reproducibility: re-run jobs NR%60==1 of raw/joblist.txt into raw_rep/ and compare TGT lines
mkdir -p raw_rep; awk 'NR%60==1' raw/joblist.txt > raw_rep/joblist.txt
xargs -P 2 -L 1 sh -c 'f=raw_rep/grid_$0_$1_$2_m$3_$4.out; grep -q "^JOBEND" $f 2>/dev/null || G3_STAGE=grid G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=$3 G3_FB=$4 G3_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) G3_GUARD_S=120 gp -q g3_census.gp < /dev/null > $f 2> /dev/null' < raw_rep/joblist.txt
for f in raw_rep/grid_*.out; do if cmp -s <(grep '^TGT' $f) <(grep '^TGT' raw/$(basename $f)); then echo "G5 OK $f"; else echo "G5 MISMATCH $f"; fi; done > raw_rep/g5.txt
# ---- Stage 3 aggregation (all gates and the verdict come from files), then plots of recorded values only
rm -f pooled.tsv controls_summary.tsv nc2_cells.tsv f4_table.tsv verdict.txt results.json
gp -q g3_aggregate.gp < /dev/null > summary.tsv 2> raw/aggregate.err
$PY plot_g3.py summary.tsv
# ---- Stage 4 (added after the frozen run, DEVIATIONS.md B1): operation counts from recorded CFG/END lines; formula check; spliced into results.json
rm -f ops.json
G3_OPS_JSON=ops.json gp -q g3_opcounts.gp < /dev/null > ops.tsv 2> raw/opcounts.err
[ -s raw/opcount_verify.out ] || G3_STAGE=none gp -q g3_census.gp g3_opcount_verify.gp < /dev/null > raw/opcount_verify.out 2>&1
sed -i '$ d' results.json; printf ']\n,"ops":%s}\n' "$(cat ops.json)" >> results.json
