#!/usr/bin/env bash
# Frozen driver for PROTOCOL.md (prospective re-run of research/jv_sieve_strong_rho_20261019
# on the repaired rho source). Usage, from the repository root:
#   bash research/jv_sieve_prospective_20261023/run.sh [check|ic|rho|all]
set -euo pipefail
BIN=./target/release/examples/jv_cover
CHK=./target/release/examples/jv_endo_check
OUT=research/jv_sieve_prospective_20261023/raw
mkdir -p $OUT
dpb() { case $1 in 251) echo 10;; 503) echo 12;; 1009) echo 14;; *) echo 15;; esac; }
job() { # p arm neg batch run seed
  f=$OUT/rho_${1}_${2}_s${6}
  [ -f $f.done ] && return 0
  $BIN --exp rhostrong --sizes $1 --seeds 1 --first-seed $6 --neg $3 --batch $4 --r 256 \
       --dp-bits $(dpb $1) --run $5 --json $f.json > $f.log && touch $f.done
}
export -f job dpb; export BIN OUT
jobs_for() { for s in $(seq 1 $6); do echo "$1 $2 $3 $4 $5 $s"; done; }
what=${1:-all}
if [ $what = check ] || [ $what = all ]; then
  [ -s $OUT/endo_check.jsonl ] || $CHK --sizes 251,503,1009,1511,1823 --seeds 16 \
    > $OUT/endo_check.jsonl 2> $OUT/endo_check.log
fi
if [ $what = ic ] || [ $what = all ]; then
  for p in 251 503 1009 1511 1823; do
    [ -s $OUT/ic_${p}_sparse.json ] && continue
    JV_LIN=sparse $BIN --exp sieve --stop 64 --sizes $p --seeds 16 --rho-runs 0 \
      --json $OUT/ic_${p}_sparse.json > $OUT/ic_${p}_sparse.log
  done
  [ -s $OUT/ic_1009_sparse_aa.json ] || JV_LIN=sparse $BIN --exp sieve --stop 64 --sizes 1009 \
    --seeds 16 --rho-runs 0 --json $OUT/ic_1009_sparse_aa.json > $OUT/ic_1009_sparse_aa.log
fi
if [ $what = rho ] || [ $what = all ]; then
  {
    for p in 251 503 1009 1511 1823; do jobs_for $p R3 1 16 0 16; done
    for p in 251 503 1009; do jobs_for $p R0 0 1 0 16; done
    jobs_for 1511 R0 0 1 0 8; jobs_for 1823 R0 0 1 0 4
    jobs_for 1009 R3_aa 1 16 1 16
  } | xargs -P 2 -L 1 bash -c 'job "$@"' _
fi
