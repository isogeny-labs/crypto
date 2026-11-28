#!/bin/bash
# Preregistered local native constructor; no concurrent build/audit.
set -eu
study=research/prime_conductor_gap80_20261031/twist-transport-20261119
label=$1
input=$2
scale=$3
result="$study/results/$label"
test ! -e "$result"
mkdir -p "$result"
exec 9>/tmp/crypto-bench.lock
flock -x 9
ulimit -v 12582912
set +e
TIMEFORMAT='wall_s=%3R user_s=%3U sys_s=%3S'
{ time timeout --signal=INT --kill-after=5s 120s \
 /tmp/conductor-twist-20261119/complete_twisted_pipeline --evaluate 4096 4096 \
 --twist-scale "$scale" --projective-batch 64 --prepared-norm \
 < "$input" > "$result/stdout" 2> "$result/stderr"; } 2> "$result/resources.txt"
code=$?
set -e
printf '%s\n' "$code" > "$result/exit"
exit "$code"
