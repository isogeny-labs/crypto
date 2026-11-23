#!/bin/bash
D=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/lift-height
ulimit -v 5000000
export PYTHONPATH=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/pylib
cd $D/code
for r in 3 4 5 6 7; do
  s=20261118; [ $r = 3 ] && s=20261121
  timeout 2400 python3 lambda_calib.py --r $r --e 32 --n 10 --tlimit 60 --seed $s --out $D/out/lambda_calib_e32b.jsonl >> $D/out/calib2.log 2>&1
  echo "r=$r exit=$?" >> $D/out/calib2.log
done
echo CALIB-DONE >> $D/out/calib2.log
