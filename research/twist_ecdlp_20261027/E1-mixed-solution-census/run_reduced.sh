#!/bin/bash
# run_reduced.sh -- reduced replication subset (about 10-15 min on 2 free cores): stage 0 + stage 1 + the 12-bit grid (64 jobs),
# written to ./reduced/ and compared byte-for-byte (counts, no timings) against the original run in this directory.
O=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E1-mixed-solution-census
R=${E1_RED_DIR:-$O/reduced}
rm -rf $R; mkdir -p $R
cp $O/e1_census.gp $O/e1_aggregate.gp $O/plot_e1.py $O/run.sh $O/opcount_verify.gp $R/
E1_GUARD_S_OVERRIDE=1000 E1_DIR=$R E1_REDUCED=1 E1_BITS_LIST=12 E1_PAR01=${E1_PAR01-1} E1_JOBS=${E1_JOBS:-4} sh $R/run.sh || exit 1
R=$R bash $O/reduced_compare.sh
