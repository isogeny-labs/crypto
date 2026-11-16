#!/bin/bash
f=$1; b=${f%.ms}
/usr/bin/time -f "wall=%e user=%U sys=%S rss_kb=%M" -o $b.msolve.time timeout -s KILL 120 msolve -g 2 -v 2 -t 1 -f $b.ms -o $b.msolve.out > $b.msolve.log 2>&1; echo "rc=$?" >> $b.msolve.time
/usr/bin/time -f "wall=%e user=%U sys=%S rss_kb=%M" -o $b.cms.time timeout -s KILL 120 cryptominisat5 --verb 0 --threads 1 --maxsol 1 $b.dimacs > $b.cms.out 2>&1; echo "rc=$?" >> $b.cms.time
