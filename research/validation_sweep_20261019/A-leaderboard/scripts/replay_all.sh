#!/bin/sh
# ecbench verify --replay-all on every session whose rows feed an inherited-F4 / F6-IC counter on the board,
# plus the root-finder session that holds the published 120/168. Receipts are written beside, tree untouched.
# usage: replay_all.sh <ecbench> <outdir>
EB="$1"; OUT="$2"
for s in research/ecbench_rootfind_engine_20261012/sessions/claim-n13 \
         research/f6_ic_ecbench_ladder_20261005/sessions/n7 research/f6_ic_ecbench_ladder_20261005/sessions/n13 \
         research/f6_ic_ecbench_ladder_20261005/sessions/n19 research/f6_ic_ecbench_ladder_20261005/sessions/n23 \
         research/f4_f5_f6_cross_family_20261011/sessions/K13 research/f4_f5_f6_cross_family_20261011/sessions/K19 \
         research/f4_f5_f6_cross_family_20261011/sessions/R13 research/f4_f5_f6_cross_family_20261011/sessions/R19; do
  tag=$(echo "$s" | sed 's#research/##; s#/sessions/#__#')
  "$EB" verify --dir "$s" --replay-all --out "$OUT/$tag.receipt.json" > "$OUT/$tag.stdout" 2> "$OUT/$tag.stderr"
  echo "$tag exit=$?"
done
