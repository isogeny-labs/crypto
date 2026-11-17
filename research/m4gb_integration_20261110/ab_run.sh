#!/usr/bin/env bash
# usage: run.sh <config> <input-basename> <repeats> <timeout-s>
AB=$(dirname "$0"); cfg=$1; in=$2; reps=$3; to=$4
ulimit -v 6000000
TIMEFORMAT='%R'
for r in $(seq 1 $reps); do
  # interleave: odd repeats base first, even repeats new first
  if [ $((r % 2)) = 1 ]; then arms="base new"; else arms="new base"; fi
  for arm in $arms; do
    out=$AB/out/$in.$cfg.$arm.r$r.out; rm -f "$out"
    t=$( { time timeout $to $AB/bin/$arm/solver_m4gb_$cfg --solve -i $AB/in/$in.in -o "$out" --nrthreads 1 --loglevel 3 >$AB/out/$in.$cfg.$arm.r$r.log 2>&1; } 2>&1 )
    rc=$(grep -c 'contradiction: I=(1)' $AB/out/$in.$cfg.$arm.r$r.log)
    deg=$(grep -o 'lcmdegree=[0-9]*' $AB/out/$in.$cfg.$arm.r$r.log | cut -d= -f2 | sort -n | tail -1)
    mem=$(grep -o 'Peak memory *: *[0-9]*' $AB/out/$in.$cfg.$arm.r$r.log | grep -o '[0-9]*$')
    h=$( [ -f "$out" ] && sha256sum "$out" | cut -c1-12 || echo "unit=$rc" )
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$in" "$cfg" "$arm" "$r" "$t" "${deg:-NA}" "${mem:-NA}" "$h"
  done
done
