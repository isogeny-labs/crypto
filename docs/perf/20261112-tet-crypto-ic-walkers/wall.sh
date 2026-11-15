#!/bin/bash
# usage: wall.sh BIN IN [N] -> per-run solve_wall_ns (ms) list
N=${3:-20}; export RAYON_NUM_THREADS=1
for i in $(seq $N); do $1 exec < $2 | jq '.report.solve_wall_ns'; done | sort -n | awk '{a[NR]=$1} END{printf "min %.2f med %.2f ms (n=%d)\n", a[1]/1e6, a[int((NR+1)/2)]/1e6, NR}'
