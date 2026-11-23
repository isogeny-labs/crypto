#!/bin/bash
while [ ! -f /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/runs/L_p2013265921_m8_N12_s1k4_r1.I1m5000.res.json ]; do sleep 20; done
sleep 5
cd /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/i4 && for f in L_p2013265921_m10_N12_s1k2_r0.c7.log L_p2013265921_m10_N12_s1k2_r0.c7.res.json; do [ -f $f ] && mv $f oom1_$f; done
echo "$(date -u +%FT%TZ) launch I4 m10 N12 r0 c7 retry with -m 5000, AS=12e9 wall=7200" >> /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/logs/progress.txt
I4T=3 I4AS=12e9 I4WALL=7200 I4EXTRA="-m 5000" /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/scripts/run_i4.sh /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/i4/cells_batch3b.txt
