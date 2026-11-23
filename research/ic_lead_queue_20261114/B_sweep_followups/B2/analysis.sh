#!/usr/bin/env bash
# B2 analysis: thin orchestration of the ecbench commands of PROTOCOL.md section 8. No statistics are computed here.
# usage: analysis.sh K1_DIR PC_DIR OUT_DIR      (frozen before data; bootstrap seed 275055567897761)
set -euo pipefail
K=${1:?K1 session dir}; P=${2:?PC session dir}; OUT=${3:?out dir}; E=${ECBENCH:-target/release/ecbench}; B=275055567897761
mkdir -p "$OUT"
"$E" verify --dir "$K" --replay-all --exit-code --out "$OUT/verify-K1.json"
"$E" verify --dir "$P" --replay-all --exit-code --out "$OUT/verify-PC.json"
c() { "$E" compare --seed "$B" --resamples 4000 --price-table-work --json "$@"; }
c --dir "$K" --a rho-plain --b kang --weight 0 --save > "$OUT/R-plain-w0.json"
c --dir "$K" --a rho-neg   --b kang --weight 0 --save > "$OUT/R-neg-w0.json"
c --dir "$K" --a rho-neg   --b rho-plain --weight 0 --save > "$OUT/g3a.json"
c --dir "$K" --a rho-plain --b kang --weight 0.1 --save > "$OUT/R-plain-w0.1.json"
c --dir "$K" --a rho-plain --b kang --weight 1   --save > "$OUT/R-plain-w1.json"
c --dir "$K" --a kang    --b kang-aa --weight 0 > "$OUT/g2-kang.json"
c --dir "$K" --a rho-neg --b rho-neg-aa --weight 0 > "$OUT/g2-rho.json"
c --dir "$P" --a bsgs-neg --b bsgs-tb --weight 0 --save > "$OUT/g3b.json"
c --dir "$P" --a bsgs-neg --b bsgs-neg-aa --weight 0 > "$OUT/g2-bsgs.json"
"$E" band  --dir "$K" --weights 0,0.1,1 --ranks --json "$OUT/band-K1.json" > "$OUT/band-K1.txt"
"$E" table --dir "$K" --reference rho-neg > "$OUT/table-K1.md"
