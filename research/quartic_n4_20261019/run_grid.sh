#!/bin/bash
# N4 grid driver. usage: run_grid.sh <n4 binary> <outdir>   (4 parallel single-thread processes)
set -u
BIN=$1; OUT=$2; mkdir -p "$OUT/units"
KS="223:5:30 499:5:30 991:5:30 2011:5:30 4051:5:30 8191:5:24 16363:5:24 32719:5:16 65479:5:16 131071:3:8 262111:3:6"
jobs=()
for spec in $KS; do IFS=: read q ns nr <<<"$spec"; for sd in $(seq 1 $ns); do jobs+=("B0 $q $sd 3.0 $nr"); done; done
for q in 223 499 991 2011 4051; do for sd in 1 2 3; do jobs+=("A0 $q $sd 0 0"); done; done
# largest first
printf '%s\n' "${jobs[@]}" | sort -k2 -n -r | xargs -P 4 -L 1 bash -c 'arm=$0; q=$1; sd=$2; c=$3; nr=$4; '"$BIN"' unit $q $sd $c $nr 1.2 > '"$OUT"'/units/${arm}_${q}_${sd}.jsonl'
