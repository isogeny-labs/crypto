#!/bin/bash
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1
cd "$(dirname "$0")"; L=../raw
lane1(){ for c in random1:1 random2:1 random1:0 koblitz:0; do python3 arm_A.py 13 $c 3; done
  for c in random1:1 random2:1 random1:0; do python3 arm_A.py 15 $c 2; done
  python3 arm_A.py 17 random1:1 1; }
lane2(){ for nc in "13 random1:1" "13 random2:1" "13 random1:0" "13 koblitz:0" "15 random1:1" "15 random2:1" "15 random1:0" "17 random1:1" "17 random2:1" "17 random1:0" "17 koblitz:1" "19 random1:1" "19 random2:1" "19 random1:0" "19 koblitz:0" "19 koblitz:1"; do set -- $nc; python3 arm_B.py $1 $2 P auto; done
  for n in 13 15 17 19; do python3 arm_B.py $n random1:1 E q; done; }
lane3(){ python3 arm_B.py 21 random1:1 P 2,q,t; python3 arm_B.py 21 random2:1 P 2,q; python3 arm_B.py 23 random1:1 P 2,q; }
lane1 > $L/lane1.log 2>&1 & lane2 > $L/lane2.log 2>&1 & lane3 > $L/lane3.log 2>&1 & wait; echo ALLDONE >> $L/lane3.log
