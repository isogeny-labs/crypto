#!/bin/bash
# usage: run_grid.sh <n4 binary> <outdir>   (4 parallel single-thread processes)
set -u
BIN=$1; OUT=$2; mkdir -p "$OUT/units"
jobs=()
# B2: all sizes; rho for the two new sizes only
for spec in 1048507:3:4 524287:3:6 262111:3:0 131071:3:0 65479:5:0 32719:5:0 16363:5:0 8191:5:0 4051:5:0 2011:5:0 991:5:0 499:5:0 223:5:0; do
  IFS=: read q ns nr <<<"$spec"; for sd in $(seq 1 $ns); do jobs+=("B2 $q $sd 2.0 $nr 1.2 12"); done; done
for spec in 262111:3 131071:3 65479:5 32719:5 16363:5 8191:5 4051:5; do
  IFS=: read q ns <<<"$spec"; for sd in $(seq 1 $ns); do jobs+=("B1 $q $sd 3.0 0 1.2 12"); done; done
for spec in 65479:3 32719:3 16363:3 8191:3 4051:3; do
  IFS=: read q ns <<<"$spec"; for sd in $(seq 1 $ns); do jobs+=("B0n $q $sd 3.0 0 1.2 12"); done; done
printf '%s\n' "${jobs[@]}" | sort -k2 -n -r | xargs -P 4 -L 1 bash -c 'arm=$0; q=$1; sd=$2; c=$3; nr=$4; rm=$5; mw=$6; '"$BIN"' unit $arm $q $sd $c $nr $rm $mw > '"$OUT"'/units/${arm}_${q}_${sd}.jsonl'
