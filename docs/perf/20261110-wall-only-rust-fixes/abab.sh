#!/bin/bash
# usage: abab.sh ROUNDS 'cmd A' 'cmd B'   -- alternates A,B; prints min/median seconds of each.
# Local, unisolated wall time; RAYON_NUM_THREADS=1 is set for both.
R=$1; A=$2; B=$3; export RAYON_NUM_THREADS=1
ta=(); tb=()
for ((i=0;i<R;i++)); do
  for arm in A B; do
    cmd=$([ $arm = A ] && echo "$A" || echo "$B")
    s=$(date +%s%N); bash -c "$cmd" >/dev/null 2>&1; rc=$?; e=$(date +%s%N)
    [ $rc -ne 0 ] && echo "arm $arm exit $rc" >&2
    t=$(( (e-s)/1000000 ))
    [ $arm = A ] && ta+=($t) || tb+=($t)
  done
done
stat() { printf '%s\n' "$@" | sort -n | awk '{a[NR]=$1} END{m=(NR%2)?a[(NR+1)/2]:(a[NR/2]+a[NR/2+1])/2; printf "min %d ms, median %s ms (n=%d) [%s]", a[1], m, NR, ""}'; }
echo "A: $(stat "${ta[@]}") all: ${ta[*]}"
echo "B: $(stat "${tb[@]}") all: ${tb[*]}"
