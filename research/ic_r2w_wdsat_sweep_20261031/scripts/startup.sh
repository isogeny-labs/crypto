#!/bin/bash
# start-up Ir of each first-mode WDSat binary on a contradictory one-line instance
for l in 3 4 5 6 7; do for b in bins/first/l$l/wdsat_id*; do
 /usr/bin/time -l -o /tmp/claude-501/su.txt $b -i evid/t2.anf -x -b -n 13 -l $l -m 3 >/dev/null 2>&1
 echo "$l $(basename $b) $(grep 'instructions retired' /tmp/claude-501/su.txt | awk '{print $1}')"
done; done
