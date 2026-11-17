#!/usr/bin/env bash
# EXP-AUXIN-25a29a route B (reduced_fraction): reduce each fraction by gcd in awk, then subtract
# reduced fractions with bc. Independent of route_power_cross.sh (no shared code, no source/import).
set -euo pipefail
red() { awk -v a="$1" -v b="$2" 'function g(x,y){while(y){t=x%y;x=y;y=t}return x} BEGIN{d=g(a,b); printf "%d %d", a/d, b/d}'; }
sub() { read n1 d1 <<<"$(red $1 $2)"; read n2 d2 <<<"$(red $3 $4)"; n=$(echo "$n1*$d2-$n2*$d1" | bc); d=$(echo "$d1*$d2" | bc); red $n $d; }
s2=$(echo "256+256" | bc); l2=$(echo "l($s2)/l(2)" | bc -l | awk '{printf "%.12f",$1}')
pex=$(echo "5^10-8*4^10" | bc); psh=$(echo "16*4^10-5^10" | bc)
g84o2=$(sub 9984 4375 2 1); g84u=$(sub 12 5 9984 4375)
g7o24=$(sub 119808 4375 24 1); g7o27=$(sub 119808 4375 27 1); g7u28=$(sub 28 1 119808 4375)
n13=$(echo "4380480000-3234765625" | bc); nh=$(echo "19140625-1752192" | bc)
printf '{"route":"reduced_fraction","stage2_steps":%s,"log2_stage2":"%s","pow_excess":%s,"pow_short":%s,"g84_minus_2":"%s","twelve_fifths_minus_g84":"%s","g7_minus_24":"%s","g7_minus_27":"%s","28_minus_g7":"%s","null_excess_13":%s,"null_short_half":%s}\n' \
  $s2 $l2 $pex $psh "${g84o2/ //}" "${g84u/ //}" "${g7o24/ //}" "${g7o27/ //}" "${g7u28/ //}" $n13 $nh
