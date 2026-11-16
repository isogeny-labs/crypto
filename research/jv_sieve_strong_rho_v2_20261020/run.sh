#!/usr/bin/env bash
# Frozen driver for PROTOCOL.md (V2): seeds 17-32. Usage: bash run.sh [ic|rho|all]
# Idempotent: finished sieve sizes and finished rho jobs (.done markers) are skipped, so a lost
# session costs at most the interrupted runs, which are re-run from the start.
set -euo pipefail
cd "$(dirname "$0")/../.."
BIN=./target/release/examples/jv_cover
OUT=research/jv_sieve_strong_rho_v2_20261020/raw
FIRST=17; N=16
mkdir -p $OUT
# producer identity, recorded before the first run; refuse a dirty producer tree
if ! git diff --quiet HEAD -- src examples Cargo.toml Cargo.lock; then
  echo "producer tree (src, examples, Cargo.*) differs from HEAD: commit it first" >&2; exit 2
fi
[ -f $OUT/PROVENANCE.json ] || cat > $OUT/PROVENANCE.json <<P
{"producer_commit": "$(git rev-parse HEAD)", "jv_cover_sha256": "$(sha256sum $BIN | cut -d' ' -f1)",
 "recorded_utc": "$(date -u +%Y-%m-%dT%H:%M:%SZ)", "seeds": [$FIRST, $((FIRST+N-1))]}
P
dpb() { case $1 in 251) echo 10;; 503) echo 12;; 1009) echo 14;; *) echo 15;; esac; }
job() { # p arm neg batch run seed   (one file per p, arm, seed)
  f=$OUT/rho_${1}_${2}_s${6}
  [ -f $f.done ] && return 0
  $BIN --exp rhostrong --sizes $1 --seeds 1 --first-seed $6 --neg $3 --batch $4 --r 256 \
       --dp-bits $(dpb $1) --run $5 --json $f.json > $f.log && touch $f.done
}
export -f job dpb; export BIN OUT
jobs_for() { # p arm neg batch run nseeds
  for i in $(seq 0 $(($6 - 1))); do echo "$1 $2 $3 $4 $5 $((FIRST + i))"; done
}
ic_done() { [ -s $OUT/$1.json ] && [ "$(grep -c '^p=' $OUT/$1.log)" -ge $N ]; }
if [ "${1:-all}" = ic ] || [ "${1:-all}" = all ]; then
  for p in 251 503 1009 1511 1823; do
    for m in dense sparse; do
      ic_done ic_${p}_$m && continue
      JV_LIN=$m $BIN --exp sieve --stop 64 --sizes $p --first-seed $FIRST --seeds $N --rho-runs 0 \
        --json $OUT/ic_${p}_$m.json > $OUT/ic_${p}_$m.log
    done
  done
  ic_done ic_1009_sparse_aa || JV_LIN=sparse $BIN --exp sieve --stop 64 --sizes 1009 --first-seed $FIRST \
    --seeds $N --rho-runs 0 --json $OUT/ic_1009_sparse_aa.json > $OUT/ic_1009_sparse_aa.log
fi
if [ "${1:-all}" = rho ] || [ "${1:-all}" = all ]; then
  { # decisive arms first (strong rho at every size, then the plain control), then factorial and A/A
    for p in 251 503 1009 1511 1823; do jobs_for $p R3 1 16 0 16; done
    for p in 251 503 1009; do jobs_for $p R0 0 1 0 16; done
    jobs_for 1511 R0 0 1 0 8; jobs_for 1823 R0 0 1 0 4
    for p in 251 503 1009; do jobs_for $p R1 0 16 0 16; jobs_for $p R2 1 1 0 16; done
    jobs_for 1009 R0_aa 0 1 1 16; jobs_for 1009 R3_aa 1 16 1 16
  } | xargs -P 4 -L 1 bash -c 'job "$@"' _
fi
