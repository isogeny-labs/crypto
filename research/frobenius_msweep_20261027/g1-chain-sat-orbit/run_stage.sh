#!/usr/bin/env bash
# One stage of cell G1. Usage:
#   run_stage.sh STAGE SEED CONFLICT_CAP NATURAL PLANTED COSET "n:m:k n:m:k ..."
# Needs RUN (UTC stamp) in the environment; writes reports/run-$RUN/. Never run
# before RUN_CARD.md step 0 (resume checkpoint) has passed.
set -euo pipefail
cd "$(dirname "$0")/../../.."
STAGE=$1; SEED=$2; CAP=$3; NAT=$4; PL=$5; CO=$6; CELLS=$7
: "${RUN:?set RUN to the UTC stamp, e.g. 20261028T000000Z}"
G1=research/frobenius_msweep_20261027/g1-chain-sat-orbit
BIN=target/release/examples/frobenius_msweep_g1_chain_sat_orbit
OUT=$G1/reports/run-$RUN/$STAGE
mkdir -p "$OUT"
COMMON="--a 0 --seed $SEED --natural $NAT --planted $PL --coset $CO"

# 1. exact oracle, once per cell, shared by every arm
for c in $CELLS; do IFS=: read -r n m k <<<"$c"; echo "$n $m $k"; done |
  xargs -P4 -L1 bash -c 'timeout 7200 '"$BIN"' oracle --n $0 --m $1 --k $2 '"$COMMON"' --out '"$OUT"'/oracle'

# 2. one job per (cell, arm, conj); order shuffled with a fixed seed so arms interleave
for c in $CELLS; do IFS=: read -r n m k <<<"$c"
  for arm in P O R Q; do echo "$n $m $k $arm 0"; done
  echo "$n $m $k P 1"   # A/A arm: plain on the sigma-conjugate target
done | shuf --random-source=<(yes "$SEED") |
  xargs -P4 -L1 bash -c 'ID=$(printf "a0-n%s-m%s-k%s-s%s" $0 $1 $2 '"$SEED"');
    timeout 14400 '"$BIN"' run --n $0 --m $1 --k $2 --arm $3 --conj $4 '"$COMMON"' \
      --conflicts '"$CAP"' --tieout 3 --run-id '"$RUN"'-'"$STAGE"' \
      --oracle '"$OUT"'/oracle/oracle-$ID.jsonl --out '"$OUT"'/rows || echo "TIMEOUT-OR-FAIL $0 $1 $2 $3 $4" >> '"$OUT"'/failures.txt'

# 3. analysis
"$BIN" analyze --rows "$OUT/rows" --out "$OUT/analysis"
