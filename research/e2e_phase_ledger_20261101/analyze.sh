#!/usr/bin/env bash
set -euo pipefail

session=${1:?usage: analyze.sh SESSION_DIR ECBENCH}
ecbench=${2:?usage: analyze.sh SESSION_DIR ECBENCH}

"$ecbench" verify --dir "$session" --replay-all
"$ecbench" table --dir "$session" --reference ic-inherited-f4-m3
"$ecbench" resources --dir "$session"
"$ecbench" band --dir "$session" --weights 0,0.1,1 --ranks

for candidate in \
  ic-matrix-f4-m3 \
  ic-matrix-f5-m3 \
  ic-f6-m3 \
  ic-f6-pair-m3 \
  ic-rootfind-m3 \
  ic-rootfind-window-m3 \
  ic-rootfind-negfold-m3 \
  ic-rootfind-window-negfold-m3
do
  "$ecbench" compare --dir "$session" \
    --a ic-inherited-f4-m3 --b "$candidate" \
    --resamples 20000 --seed 202611010131 --save --json
done

"$ecbench" compare --dir "$session" \
  --a rho-strong-v2 --b rho-strong-v2-aa \
  --resamples 20000 --seed 202611010132 --save --json
