#!/usr/bin/env bash
set -euo pipefail

session=${1:?usage: analyze.sh SESSION_DIR ECBENCH}
ecbench=${2:?usage: analyze.sh SESSION_DIR ECBENCH}

"$ecbench" verify --dir "$session" --replay-all
"$ecbench" table --dir "$session" --reference coordinate-glv
for pair in \
  coordinate-glv:multiplier \
  plain-equal-points:multiplier \
  plain-same-seeds:multiplier \
  rho-neg:multiplier \
  multiplier:multiplier-aa
do
  a=${pair%%:*}
  b=${pair##*:}
  "$ecbench" compare --dir "$session" --a "$a" --b "$b" \
    --resamples 20000 --seed 2026110201 --save --json
done
