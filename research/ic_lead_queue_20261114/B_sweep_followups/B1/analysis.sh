#!/usr/bin/env bash
# B1 analysis: thin orchestration of the ecbench commands of PROTOCOL.md section 9. No statistics are computed here.
# usage: analysis.sh SESSION_DIR OUT_DIR       (frozen before data; bootstrap seed 171294647853004)
set -euo pipefail
D=${1:?session dir}; OUT=${2:?out dir}; E=${ECBENCH:-target/release/ecbench}; B=171294647853004
mkdir -p "$OUT"
"$E" verify --dir "$D" --replay-all --exit-code --out "$OUT/verify.json"
for N in n56 n64 n80 n91; do for W in 0 0.1 1; do
  "$E" compare --dir "$D" --a n45 --b "$N" --weight "$W" --price-table-work    --seed "$B" --resamples 4000 --save --json > "$OUT/cmp-$N-w$W-priced.json"
  "$E" compare --dir "$D" --a n45 --b "$N" --weight "$W" --no-price-table-work --seed "$B" --resamples 4000 --save --json > "$OUT/cmp-$N-w$W-unpriced.json"
done; done
for W in 0 0.1 1; do
  "$E" compare --dir "$D" --a inc-n32 --b n45 --weight "$W" --no-price-table-work --seed "$B" --resamples 4000 --save --json > "$OUT/g3-w$W-unpriced.json"
  "$E" compare --dir "$D" --a inc-n32 --b n45 --weight "$W" --price-table-work    --seed "$B" --resamples 4000 --save --json > "$OUT/g3-w$W-priced.json"
done
"$E" compare --dir "$D" --a n45 --b n45-aa --weight 0 --price-table-work --seed "$B" --json > "$OUT/g2-n45.json"
"$E" compare --dir "$D" --a rho-neg --b rho-neg-aa --weight 0 --seed "$B" --json > "$OUT/g2-rho.json"
"$E" band  --dir "$D" --weights 0,0.1,1 --ranks --price-table-work    --json "$OUT/band-priced.json"   > "$OUT/band-priced.txt"
"$E" band  --dir "$D" --weights 0,0.1,1 --ranks --no-price-table-work --json "$OUT/band-unpriced.json" > "$OUT/band-unpriced.txt"
"$E" table --dir "$D" --reference rho-neg > "$OUT/table.md"
