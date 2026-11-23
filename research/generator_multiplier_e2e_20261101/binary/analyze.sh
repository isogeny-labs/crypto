#!/usr/bin/env bash
set -euo pipefail

session=${1:?usage: analyze.sh SESSION_DIR ECBENCH}
ecbench=${2:?usage: analyze.sh SESSION_DIR ECBENCH}

"$ecbench" verify --dir "$session" --replay-all
"$ecbench" table --dir "$session" --reference coordinate-subspace
for pair in \
  coordinate-subspace:multiplier \
  plain-best:multiplier \
  plain-same-seeds:multiplier \
  rho-neg:multiplier \
  multiplier:multiplier-aa
do
  a=${pair%%:*}
  b=${pair##*:}
  "$ecbench" compare --dir "$session" --a "$a" --b "$b" \
    --resamples 20000 --seed 2026110202 --save --json
done
