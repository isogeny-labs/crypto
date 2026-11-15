#!/bin/bash
# usage: job.sh B SEED   (b=3s diagonal, m=3, school, cap 100k)
B=$1; SEED=$2; S=$((B/3)); cd "$(dirname "$0")"
BB=$(printf %02d $B)
[ -d raw/sweep/b$BB-s$SEED ] && exit 0
if [ "$B" = 9 ]; then export KEEP_CNF_DIR=$PWD/cnf; fi
export MULMODE=school KISSAT_CONFLICTS=100000 TMPDIR=$PWD/tmp
exec timeout 5400 nice -n 19 sweeper/target/release/prime_pdp_sat_sweep sweep --m 3 --bits $B --two-adicity 2 --families small-x --encodings bits-grouplaw --ls $S --planted 8 --uniform 8 --timeout 120 --seed $SEED --out raw/sweep/b$BB-s$SEED > raw/sweep/b$BB-s$SEED.log 2>&1
