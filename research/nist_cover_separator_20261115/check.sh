#!/bin/sh
set -eu
cd "$(dirname "$0")"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
c++ -std=c++17 -O2 -Wall -Wextra -pedantic audit.cpp -o "$tmp/audit"
"$tmp/audit" data/scores.csv data/cover-metrics.csv
awk 'BEGIN {FS=",";OFS=","} NR==2 {$13=9999} {print}' data/scores.csv > "$tmp/bad-scores.csv"
if "$tmp/audit" "$tmp/bad-scores.csv" data/cover-metrics.csv > "$tmp/control.out" 2>&1; then
    echo 'ERROR: corrupted score was accepted' >&2; exit 1
fi
printf '%s\n' 'Corrupted-score control rejected:'
cat "$tmp/control.out"
sed '2d' data/cover-metrics.csv > "$tmp/missing-cover.csv"
if "$tmp/audit" data/scores.csv "$tmp/missing-cover.csv" > "$tmp/control.out" 2>&1; then
    echo 'ERROR: incomplete metric table was accepted' >&2; exit 1
fi
printf '%s\n' 'Missing-cover control rejected:'
tail -n 1 "$tmp/control.out"
