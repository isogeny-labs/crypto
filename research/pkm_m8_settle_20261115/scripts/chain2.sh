#!/bin/bash
while ! grep -q "BATCH DONE.*cells_batch1" /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/i4/i4_progress.txt; do sleep 20; done
/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/scripts/run_i4.sh /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/i4/cells_batch2.txt
