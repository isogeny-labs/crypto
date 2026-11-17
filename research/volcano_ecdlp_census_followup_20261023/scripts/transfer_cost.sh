#!/bin/sh
# PLAN 2 "Transport cost is measured per path with ca volcano --transport-bench" (Phase B, in its own lock window).
# For each census source walk of V5 and V6, rerun ca rooted at the source with --transport-bench (same flags and seed as census.sh).
# For Tcr: ca rooted at each non-crater V5 vertex v (its census model); the line of the level-1 vertex is the path v -> w. The last
# edge (w -> j = 0) is not a ca edge (ca does not step onto j = 0); it is counted in the analysis as 2 Velu point maps (G and Q).
. "$(dirname "$0")/env.sh"
OUT=$STUDY/runs/transfer_cost.jsonl; : > $OUT
SEED=20261023; REPS=${REPS:-200}
for V in V5 V6; do
  case $V in V5) ELL=2;; V6) ELL=3;; esac
  for S in D floor-a floor-b crater-b; do
    W=$STUDY/census/$V/walks/$S/R.json; [ -f $W ] || continue
    P=$(jq -r .p $W); A=$(jq -r .a $W); B=$(jq -r .b $W)
    T=$(mktemp)
    $CA volcano --p $P --a $A --b $B --ell $ELL --seed $SEED --trials 1 --per-level 0 --controls 0 --radius 40 --max-vertices 20000 --transport-bench $T --transport-reps $REPS > /dev/null 2>$T.err || { echo "FAIL $V $S" >&2; continue; }
    jq -c --arg v $V --arg s $S --argjson ell $ELL --argjson p $P '. + {volcano:$v,src:$s,ell:$ell,p:$p}' $T >> $OUT
    rm -f $T $T.err
  done
done
# Tcr paths
P=$(jq -r .p $STUDY/census/V5/walks/D/R.json)
jq -c '{id,a,b}' $STUDY/census/V5/vertices.jsonl | while read -r row; do
  id=$(echo "$row" | jq -r .id); [ "$id" = 0 ] && continue
  A=$(echo "$row" | jq -r .a); B=$(echo "$row" | jq -r .b)
  T=$(mktemp)
  $CA volcano --p $P --a $A --b $B --ell 2 --seed $SEED --trials 1 --per-level 0 --controls 0 --radius 40 --max-vertices 20000 --transport-bench $T --transport-reps $REPS > /dev/null 2>$T.err || { echo "FAIL Tcr $id" >&2; continue; }
  jq -c --argjson src $id --argjson p $P 'select(.level==1) + {volcano:"V5",src:("tcr-v"+($src|tostring)),ell:2,p:$p,tcr_source_id:$src,note:"path v -> level-1 vertex w; add 2 Velu point maps for the last edge w -> j=0"}' $T >> $OUT
  rm -f $T $T.err
done
wc -l $OUT
