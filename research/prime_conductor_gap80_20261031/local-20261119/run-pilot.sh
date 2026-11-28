#!/bin/bash
set -eu
study=research/prime_conductor_gap80_20261031/local-20261119
mode=${1:?affinity mode}
label=${2:?fresh label}
binary=/workspace/tools/conductor-local-20261119/ecbench
test ! -e "$study/results/$label"
mkdir -p "$study/results"
ulimit -v 12582912
if timeout --signal=INT --kill-after=30s 660s "$binary" run --spec "$study/pilot-spec.json" --out "$study/results/$label" --cpus "$mode" --wait --quiet > "$study/$label.stdout" 2> "$study/$label.stderr"; then code=0; else code=$?; fi
printf '%s\n' "$code" > "$study/$label.exit"
exit "$code"
