#!/usr/bin/env bash
# usage: falsify_cell.sh W T TARGET BUDGET ENTRIES
set -u
cd /home/user/crypto
W=$1; T=$2; K=$3; B=$4; E=$5
OUT=research/prime_ic_leads_20261110/growth_law/falsify
mkdir -p $OUT
NAME=nullw${W}_m2_t${T}_k${K}
echo "== $NAME: --controls null --null-width $W --m 2 --t-min $T --max-t $T --targets $K --hky-cmax ${CMAX:-7} --hky-max-entries $E --budget $B ($(date -u +%FT%TZ))" >> $OUT/commands.log
CHECKPOINT_SKIP=devcheck target/release/examples/pkm_tower_pilot --engine hky --kinds kummer --controls null --planted 0 --random 2 --ladder-t none --p 2013265921 --seed 1347106132 --null-width $W --m 2 --t-min $T --max-t $T --max-t-m3 $T --targets $K --hky-cmax ${CMAX:-7} --hky-max-entries $E --budget $B --out $OUT/$NAME.jsonl > $OUT/run_$NAME.log 2>&1
echo "exit=$?" >> $OUT/run_$NAME.log
