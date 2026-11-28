#!/bin/bash
cd /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/code
PYTHONPATH=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/../../pylib python3 ss_coverage.py --bits 12,16,20,24,28,32,36 --per 3 --out /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/ss_coverage.jsonl > /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/ss_coverage.log 2>&1
echo done >> /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/ss_coverage.log
