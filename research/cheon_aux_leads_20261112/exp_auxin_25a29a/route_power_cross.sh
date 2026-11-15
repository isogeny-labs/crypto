#!/usr/bin/env bash
# EXP-AUXIN-25a29a route A (power_cross): integer cross-multiplication in bash 64-bit arithmetic.
# Independent of route_reduced_fraction.sh (no shared code, no source/import).
set -euo pipefail
s2=$((2*256)); p9=$((1<<9))
p5=1; for i in $(seq 10); do p5=$((p5*5)); done
p4=$((1<<20))
pex=$((p5-8*p4)); psh=$((16*p4-p5))
g84o2=$((9984-2*4375)); g84u=$((12*4375/5-9984)); g84r=$((12*4375%5))
g7o24=$((119808-24*4375)); g7o27=$((119808-27*4375)); g7u28=$((28*4375-119808))
n13=$((4380480000-3234765625)); nh=$((19140625-1752192))
printf '{"route":"power_cross","stage2_steps":%d,"stage2_is_2pow9":%s,"pow_left":%d,"pow_right_low":%d,"pow_excess":%d,"pow_right_high":%d,"pow_short":%d,"g84_over_2_num":%d,"g84_under_12_5_num":%d,"twelve_fifths_exact":%s,"g7_over_24_num":%d,"g7_over_27_num":%d,"g7_under_28_num":%d,"null_excess_13":%d,"null_short_half":%d,"den":4375}\n' \
  $s2 $([ $s2 -eq $p9 ] && echo true || echo false) $p5 $((8*p4)) $pex $((16*p4)) $psh $g84o2 $g84u $([ $g84r -eq 0 ] && echo true || echo false) $g7o24 $g7o27 $g7u28 $n13 $nh
