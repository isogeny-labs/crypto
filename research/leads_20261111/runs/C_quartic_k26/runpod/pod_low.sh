#!/usr/bin/env bash
# Addendum A: k = 17..21 on the same pod, same pattern, after the main sequence. Strictly sequential.
set -u
cd /work
B=./v2w2
QS=(131071 262111 524287 1048507 2097091)
cal() { r=$1
  for i in 0 1 2 3 4; do q=${QS[$(( (i + r) % 5 ))]}
    for c in 1 2 3; do python3 runjob.py out/cal${r}_${q}_${c} 1200 $B calib $q $c >/dev/null; done; done
  echo "low cal $r done $(date -u +%FT%TZ)" >> out/progress.txt; }
unit() { V2W_COMPACT=1 python3 runjob.py out/ic_$1_$2_D 3600 $B ic $1 $2 D 3.0 2.0 4 4 >/dev/null; echo "unit $1 $2 done $(date -u +%FT%TZ)" >> out/progress.txt; }
until grep -q "MAIN DONE" out/progress.txt 2>/dev/null; do sleep 20; done
cal 1; for q in ${QS[@]}; do unit $q 1; done
cal 2; for q in 2097091 1048507 524287 262111 131071; do unit $q 2; done
cal 3; for q in ${QS[@]}; do unit $q 3; done
cal 4
echo "LOW DONE $(date -u +%FT%TZ)" >> out/progress.txt
# measured negation rho, parallel, only after every timed job above
mkdir -p rho
for q in 4194271 8388571; do for i in 0 1 2 3 4 5 6 7; do
  python3 runjob.py rho/rho_${q}_$i 14400 $B rho $q 1 $i $((i+1)) neg >/dev/null &
done; done
wait
echo "RHO DONE $(date -u +%FT%TZ)" >> out/progress.txt
