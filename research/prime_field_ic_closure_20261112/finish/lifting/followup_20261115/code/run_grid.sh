#!/bin/bash
cd /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/code
xargs -P 3 -L 1 python3 xedni_cubic.py < /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/grid.txt > /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/grid.log 2>&1
echo done >> /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lifting/out/grid.log
