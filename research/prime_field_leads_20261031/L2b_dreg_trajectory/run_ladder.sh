#!/bin/bash
# Frozen ladder, as executed: p = largest prime <= 2^k.
#  k=10,12,14,16 : 2 curves x 4 targets x 2 arms (full design)
#  k=18          : curve 1 x targets 1-2 x 2 arms (reduced: cost, see REPORT)
#  k=20          : curve 1 x target 1 x 2 arms (reduced)
# Long cells first. Cap per run: CAP seconds (30 min maximum permitted).
D=$(cd "$(dirname "$0")" && pwd)
{
 for t in 1; do for a in real ctrl; do echo "1048573 1 $t $a"; done; done
 for t in 1 2; do for a in real ctrl; do echo "262139 1 $t $a"; done; done
 for p in 65521 16381 4093 1021; do
  for c in 1 2; do for t in 1 2 3 4; do for a in real ctrl; do echo "$p $c $t $a"; done; done; done
 done
} | xargs -P ${PAR:-8} -L 1 sh -c '"'$D'/run_cell.sh" $0 $1 $2 $3 ${CAP:-1500}'
