#!/bin/sh
# PLAN 3 "Transfer cost": for each census source walk, rerun ca volcano rooted at the source
# with --transport-bench (same flags and seed as census.sh), tag every line with volcano/source.
. "$(dirname "$0")/env.sh"
CA=${CA:-/home/user/cryptanalysis/build/ca}
OUT=$STUDY/runs/transfer_cost.jsonl; : > $OUT
SEED=20261017; REPS=${REPS:-200}
for V in V1 V2 V3; do
  case $V in V1) ELL=2;; V2) ELL=3;; V3) ELL=2;; esac
  for S in D floor-a floor-b crater-b; do
    W=$STUDY/census/$V/walks/$S/R.json; [ -f $W ] || continue
    P=$(jq -r .p $W); A=$(jq -r .a $W); B=$(jq -r .b $W)
    T=$(mktemp)
    $CA volcano --p $P --a $A --b $B --ell $ELL --seed $SEED --trials 1 --per-level 0 --controls 0 --radius 40 --max-vertices 20000 --transport-bench $T --transport-reps $REPS > /dev/null 2>$T.err || { echo "FAIL $V $S" >&2; continue; }
    jq -c --arg v $V --arg s $S --argjson ell $ELL --argjson p $P '. + {volcano:$v,src:$s,ell:$ell,p:$p}' $T >> $OUT
    rm -f $T $T.err
  done
done
wc -l $OUT
