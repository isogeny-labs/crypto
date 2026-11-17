#!/usr/bin/env bash
# Frozen driver for the study in PROTOCOL.md. Usage: run.sh [ic|rho]
set -euo pipefail
cd "$(dirname "$0")/../.."
BIN=./target/release/examples/jv_cover
OUT=research/jv_sieve_strong_rho_20261019/raw
mkdir -p $OUT
dpb() { case $1 in 251) echo 10;; 503) echo 12;; 1009) echo 14;; *) echo 15;; esac; }
# Rho runs are per-instance jobs (one file per p, arm, seed) so a lost session costs one run, not an arm.
job() { # p arm neg batch run seed
  f=$OUT/rho_${1}_${2}_s${6}
  [ -f $f.done ] && return 0
  $BIN --exp rhostrong --sizes $1 --seeds 1 --first-seed $6 --neg $3 --batch $4 --r 256 \
       --dp-bits $(dpb $1) --run $5 --json $f.json > $f.log && touch $f.done
}
export -f job dpb; export BIN OUT
jobs_for() { # p arm neg batch run nseeds
  for s in $(seq 1 $6); do echo "$1 $2 $3 $4 $5 $s"; done
}
if [ "${1:-all}" = ic ] || [ "${1:-all}" = all ]; then
  for p in 251 503 1009 1511 1823; do
    for m in dense sparse; do
      [ -s $OUT/ic_${p}_$m.json ] && [ $(grep -c "^p=" $OUT/ic_${p}_$m.log) -ge 16 ] && continue
      JV_LIN=$m $BIN --exp sieve --stop 64 --sizes $p --seeds 16 --rho-runs 0 \
        --json $OUT/ic_${p}_$m.json > $OUT/ic_${p}_$m.log
    done
  done
  [ -s $OUT/ic_1009_sparse_aa.json ] || JV_LIN=sparse $BIN --exp sieve --stop 64 --sizes 1009 --seeds 16 --rho-runs 0 \
    --json $OUT/ic_1009_sparse_aa.json > $OUT/ic_1009_sparse_aa.log
fi
if [ "${1:-all}" = rho ] || [ "${1:-all}" = all ]; then
  { # decisive arms first (strong rho at every size, then the plain control), then the factorial and A/A
    for p in 251 503 1009 1511 1823; do jobs_for $p R3 1 16 0 16; done
    for p in 251 503 1009; do jobs_for $p R0 0 1 0 16; done
    jobs_for 1511 R0 0 1 0 8; jobs_for 1823 R0 0 1 0 4
    for p in 251 503 1009; do jobs_for $p R1 0 16 0 16; jobs_for $p R2 1 1 0 16; done
    jobs_for 1009 R0_aa 0 1 1 16; jobs_for 1009 R3_aa 1 16 1 16
  } | xargs -P 4 -L 1 bash -c 'job "$@"' _
fi
