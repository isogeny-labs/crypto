#!/bin/bash
# usage: solveab.sh LABEL BIN_A IN_A BIN_B IN_B [N=20] -> 5 alternating rounds; per round the summed report.solve_wall_ns of N child runs; min/median ms
# S: work directory holding the base-*/ gate sessions (as run: the session scratchpad)
S=${S:-$PWD}; N=${6:-20}; export RAYON_NUM_THREADS=1
l0=$(cut -d' ' -f1-3 /proc/loadavg); ta=(); tb=()
one() { local t=0; for i in $(seq $N); do v=$($1 exec < $2 | jq '.report.solve_wall_ns'); t=$((t+v)); done; echo $((t/1000000)); }
for r in 1 2 3 4 5; do ta+=($(one $2 $3)); tb+=($(one $4 $5)); done
st() { printf '%s\n' "$@" | sort -n | awk '{a[NR]=$1} END{printf "min %d ms, median %d ms", a[1], a[3]}'; }
echo "== $1 | load before $l0 | after $(cut -d' ' -f1-3 /proc/loadavg)"; echo "A: $(st ${ta[@]}) all: ${ta[*]}"; echo "B: $(st ${tb[@]}) all: ${tb[*]}"
