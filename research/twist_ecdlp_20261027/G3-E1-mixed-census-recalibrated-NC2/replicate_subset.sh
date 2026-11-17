#!/bin/bash
# replicate_subset.sh -- reduced replication (about 12-15 min on a loaded 4-core box). Re-runs, with the preregistered seed, into raw_sub/:
#   (1) stage 0 oracle gate (G0); (2) positive controls at 12 bits (PC1-3); (3) 8 grid jobs (12- and 16-bit, m=2 and 3, all 4 FB types) and
#   (4) 4 NC2 jobs (12/16 bits), then compares the deterministic lines (V0*, PC*, TGT, NCT) byte-for-byte against the full run in raw/.
# Wall-time columns are not compared (contention-dependent). Prints SUBSET_OK or SUBSET_MISMATCH lines. Does not touch raw/.
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G3-E1-mixed-census-recalibrated-NC2
cd $D && mkdir -p raw_sub && export G3_SEED=20261026
G3_STAGE=oracle gp -q g3_census.gp < /dev/null > raw_sub/stage0_oracle.out 2> /dev/null
if cmp -s <(grep -E '^V0' raw_sub/stage0_oracle.out) <(grep -E '^V0' raw/stage0_oracle.out); then echo "SUBSET_OK stage0"; else echo "SUBSET_MISMATCH stage0"; fi
G3_STAGE=controls G3_CPART=pc G3_CBITS=12 gp -q g3_census.gp < /dev/null > raw_sub/stage1_pc_12.out 2> /dev/null
if cmp -s <(grep -E '^(PC|CONTROLS_RESULT)' raw_sub/stage1_pc_12.out) <(grep -E '^(PC|CONTROLS_RESULT)' raw/stage1_pc_12.out); then echo "SUBSET_OK pc12"; else echo "SUBSET_MISMATCH pc12"; fi
cat > raw_sub/joblist.txt <<'EOJ'
12 0 7 3 interval
12 1 10 3 random
12 3 1 3 farey
12 2 6 2 twistmult
16 1 7 3 interval
16 2 3 2 farey
16 0 4 3 twistmult
16 3 6 2 random
EOJ
xargs -P 2 -L 1 sh -c 'f=raw_sub/grid_$0_$1_$2_m$3_$4.out; G3_STAGE=grid G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=$3 G3_FB=$4 G3_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) G3_GUARD_S=120 gp -q g3_census.gp < /dev/null > $f 2> /dev/null' < raw_sub/joblist.txt
for f in raw_sub/grid_*.out; do if cmp -s <(grep '^TGT' $f) <(grep '^TGT' raw/$(basename $f)); then echo "SUBSET_OK $f"; else echo "SUBSET_MISMATCH $f"; fi; done
cat > raw_sub/nc2list.txt <<'EOJ'
12 0 2 3 random
12 1 4 2 farey
16 0 1 3 interval
16 1 3 3 twistmult
EOJ
xargs -P 2 -L 1 sh -c 'f=raw_sub/nc2_$0_$1_$2_m$3_$4.out; G3_STAGE=nc2 G3_BITS=$0 G3_PIDX=$1 G3_CURVE=$2 G3_M=$3 G3_FB=$4 G3_TARGETS=$(if [ $3 = 2 ]; then echo 200; else echo 100; fi) gp -q g3_census.gp < /dev/null > $f 2> /dev/null' < raw_sub/nc2list.txt
for f in raw_sub/nc2_*.out; do if cmp -s <(grep '^NCT' $f) <(grep '^NCT' raw/$(basename $f)); then echo "SUBSET_OK $f"; else echo "SUBSET_MISMATCH $f"; fi; done
