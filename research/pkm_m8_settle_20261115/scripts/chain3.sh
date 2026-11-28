#!/bin/bash
R=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/runs/L_p2013265921_m8_N12_s1k4_r0.I1.res.json
while [ ! -f $R ]; do sleep 20; done
sleep 5
if python3 -c "import json,sys;r=json.load(open('$R'));sys.exit(0 if r['stop'] is None else 1)" && grep -q '\[1\]' /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/runs/L_p2013265921_m8_N12_s1k4_r0.I1.out; then
  S=L_p2013265921_m8_N12_s1k4_r1
  echo "$(date -u +%FT%TZ) launch I1 $S T=3 AS=12e9 wall=7200" >> /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/logs/progress.txt
  cd /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/repo && python3 tools/benchrec.py run benchmarks/records/20261115T2340Z-pkm-kummer-m8-settle.json -- python3 ../scripts/cap.py 12e9 7200 ../r2/runs/$S.I1.log ../r2/runs/$S.I1.res.json -- msolve -t 3 -v 2 -g 2 -f ../systems/$S.ms -o ../r2/runs/$S.I1.out < /dev/null > /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/logs/benchrec_I1_$S.txt 2>&1
fi
echo "$(date -u +%FT%TZ) launch I4 m10 N12 r0 c7 AS=11e9 wall=7200" >> /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/logs/progress.txt
I4AS=11e9 I4WALL=7200 /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/scripts/run_i4.sh /tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/m8/r2/i4/cells_batch3b.txt
