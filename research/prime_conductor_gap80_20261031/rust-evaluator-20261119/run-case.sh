#!/bin/bash
# Existing Rust ecbench entry point; backend hash checked by Rust before dlopen.
set -eu
study=research/prime_conductor_gap80_20261031/rust-evaluator-20261119
label=$1
input=$2
scale=$3
window=$4
block=$5
cap=$6
result="$study/results/$label"
test ! -e "$result"
mkdir -p "$result"
read -r bridge_sha bridge_path < "$study/native-library.sha256"
ulimit -v 12582912
TIMEFORMAT='wall_s=%3R user_s=%3U sys_s=%3S'
set +e
{ time timeout --signal=INT --kill-after=5s "${cap}s" \
 /tmp/conductor-rust-evaluator-20261119/ecbench isogeny-evaluate \
 --input "$input" --out "$result/result.json" \
 --native-library "$bridge_path" --native-sha256 "$bridge_sha" \
 --twist-scale "$scale" --max-window "$window" --block "$block" \
 --projective-batch 64 > "$result/stdout" 2> "$result/stderr"; } \
 2> "$result/resources.txt"
code=$?
set -e
printf '%s\n' "$code" > "$result/exit"
exit "$code"
