#!/usr/bin/env bash
# Percentile CI from bootstrap replicates (linear interpolation between order
# statistics at position (n-1)q), plus mean and sd of the replicates.
# Usage: quantiles.sh FILE   (lines "rep<TAB>value")
set -eu
grep '^rep' "$1" | cut -f2 | sort -g | awk '{v[NR]=$1; s+=$1; q+=$1*$1} END{n=NR
  split("0.025 0.975 0.05 0.95", P, " ")
  for (i=1;i<=4;i++){ h=(n-1)*P[i]+1; lo=int(h); f=h-lo; x=v[lo]+(lo<n ? f*(v[lo+1]-v[lo]) : 0); printf "q%s\t%.17g\n", P[i], x }
  m=s/n; printf "n\t%d\nmean\t%.17g\nsd\t%.17g\n", n, m, sqrt((q-n*m*m)/(n-1)) }'
