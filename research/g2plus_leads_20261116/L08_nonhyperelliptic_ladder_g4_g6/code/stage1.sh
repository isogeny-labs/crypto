#!/bin/bash
cd /home/user/crypto/research/g2plus_leads_20261116/L08_nonhyperelliptic_ladder_g4_g6
for p in 257 521 1031 2053; do
  code/runlog.sh raw/run_d5_p$p.jsonl python3 code/l08.py run --d 5 --m 2 --p $p --curves 0,1,2,3 --out raw/run_d5_p$p.jsonl
  code/runlog.sh raw/run_d4_p$p.jsonl python3 code/l08.py run --d 4 --m 0 --p $p --curves 0,1,2,3 --out raw/run_d4_p$p.jsonl
done
for p in 4099 8209; do for c in 0 1 2 3; do
  code/runlog.sh raw/run_d5_p${p}_c$c.jsonl python3 code/l08.py run --d 5 --m 2 --p $p --curves $c --out raw/run_d5_p${p}_c$c.jsonl
  code/runlog.sh raw/run_d4_p${p}_c$c.jsonl python3 code/l08.py run --d 4 --m 0 --p $p --curves $c --out raw/run_d4_p${p}_c$c.jsonl
done; done
echo DONE > raw/stage1.done
