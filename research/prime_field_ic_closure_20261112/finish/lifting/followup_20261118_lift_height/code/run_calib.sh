#!/bin/bash
# arm E: lambda_MW vs lambda_G calibration at e = 16, 32 (waits for arm D to finish)
D=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/lift-height
while ! grep -q GRID-DONE $D/out/gridD.log; do sleep 30; done
ulimit -v 5000000
export PYTHONPATH=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/pylib
cd $D/code
timeout 9000 python3 lambda_calib.py --r 2,3,4,5,6,7 --e 16,32 --n 10 --tlimit 60 --seed 20261118 --out $D/out/lambda_calib_e16e32.jsonl > $D/out/calib.log 2>&1
echo CALIB-DONE >> $D/out/calib.log
