#!/bin/bash
set -eu
study=research/prime_conductor_gap80_20261031/local-20261119
test ! -e "$study/results/target80-bounded"
mkdir -p "$study/results/target80-bounded"
ulimit -v 12582912
if timeout --signal=INT --kill-after=5s 60s /tmp/conductor-prepared-norm-20261118/complete_pipeline --evaluate 500000000000 64 --projective-batch 64 --prepared-norm < "$study/target80-33points.txt" > "$study/results/target80-bounded/stdout" 2> "$study/results/target80-bounded/stderr"; then code=0; else code=$?; fi
printf '%s\n' "$code" > "$study/results/target80-bounded/exit"
exit "$code"
