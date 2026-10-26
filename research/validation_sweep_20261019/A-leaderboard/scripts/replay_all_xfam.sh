#!/bin/sh
# second half of replay_all.sh, run separately because the n23 session's replay-all takes hours
EB="$1"; OUT="$2"
for s in research/f4_f5_f6_cross_family_20261011/sessions/K13 research/f4_f5_f6_cross_family_20261011/sessions/K19 \
         research/f4_f5_f6_cross_family_20261011/sessions/R13 research/f4_f5_f6_cross_family_20261011/sessions/R19; do
  tag=$(echo "$s" | sed 's#research/##; s#/sessions/#__#')
  "$EB" verify --dir "$s" --replay-all --out "$OUT/$tag.receipt.json" > "$OUT/$tag.stdout" 2> "$OUT/$tag.stderr"
  echo "$tag exit=$?"
done
