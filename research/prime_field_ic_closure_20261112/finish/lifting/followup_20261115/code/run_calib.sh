#!/bin/bash
cd /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/code
PYTHONPATH=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/../../pylib python3 lambda_calib.py --r 2,3,4,5,6,7 --e 4,8,16,32 --n 15 --tlimit 90 --out /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/lambda_calib.jsonl > /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/lambda_calib.log 2>&1
echo done >> /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/lambda_calib.log
