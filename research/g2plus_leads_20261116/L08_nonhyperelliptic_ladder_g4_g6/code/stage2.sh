#!/bin/bash
cd /home/user/crypto/research/g2plus_leads_20261116/L08_nonhyperelliptic_ladder_g4_g6
for p in 131 257 521 1031 2053; do
  code/runlog.sh raw/opt_d6m5_p$p.jsonl python3 code/l08.py run --d 6 --m 5 --p $p --curves 0,1,2,3 --out raw/opt_d6m5_p$p.jsonl
  code/runlog.sh raw/opt_d6m4_p$p.jsonl python3 code/l08.py run --d 6 --m 4 --p $p --curves 0,1,2,3 --out raw/opt_d6m4_p$p.jsonl
done
echo DONE > raw/stage2.done
