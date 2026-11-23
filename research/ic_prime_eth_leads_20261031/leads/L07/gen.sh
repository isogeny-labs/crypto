#!/bin/sh
cd "$(dirname "$0")"
cc -O2 -o dlog dlog.c || exit 1
i=0
while read b p a bb r gx gy; do
  i=$((i+1)); ./dlog $p $a $r $gx $gy tab_${b}_$i.bin || echo FAIL $b $i
done < curves.txt
ls -la tab_*.bin
