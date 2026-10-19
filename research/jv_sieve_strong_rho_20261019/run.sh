#!/usr/bin/env bash
# Frozen driver for the study in PROTOCOL.md. Usage: run.sh [ic|rho]
set -euo pipefail
cd "$(dirname "$0")/../.."
BIN=./target/release/examples/jv_cover
OUT=research/jv_sieve_strong_rho_20261019/raw
mkdir -p $OUT
dpb() { case $1 in 251) echo 10;; 503) echo 12;; 1009) echo 14;; *) echo 15;; esac; }
rho() { # p seeds neg batch tag [run]; idempotent: a finished piece is skipped (restart after a lost session)
  [ -s $OUT/rho_${1}_${5}.json ] && [ -f $OUT/rho_${1}_${5}.done ] && return 0
  $BIN --exp rhostrong --sizes $1 --seeds $2 --neg $3 --batch $4 --r 256 --dp-bits $(dpb $1) \
       --run ${6:-0} --json $OUT/rho_${1}_${5}.json > $OUT/rho_${1}_${5}.log
  touch $OUT/rho_${1}_${5}.done
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
  for p in 251 503 1009; do
    rho $p 16 0 1 R0; rho $p 16 0 16 R1; rho $p 16 1 1 R2; rho $p 16 1 16 R3
  done
  rho 1009 16 0 1 R0_aa 1; rho 1009 16 1 16 R3_aa 1
  rho 1511 16 1 16 R3; rho 1511 8 0 1 R0
  rho 1823 16 1 16 R3; rho 1823 4 0 1 R0
fi
