#!/bin/bash
cd /tmp/claude-0/-home-user/eb02eff3-ee5a-5113-a25c-b6cc0a65fdff/scratchpad/wave2/KM-POPCODE-STEPS
r(){ n=$1; shift; echo "$(date +%T) START $n" >> raw/queue.log; python3 -u run_cell.py "$@" raw/$n.jsonl > raw/$n.log 2>&1; echo "$(date +%T) END $n rc=$?" >> raw/queue.log; }
r gen_W1_d1_D3 gen 1 1 40961 3 12
r n1_W1_D3 n1 1 0 40961 3 11
r n2_W1_D3 n2 1 0 40961 3 11
r gen_W1_p12289_D3 gen 1 2 12289 3 12
r gen_W2_d0_D3 gen 2 0 40961 3 12
r gen_W3_d0_D3 gen 3 0 40961 3 12
r gen_W2_d0_D4 gen 2 0 40961 4 12
r gen_W3_d0_D4 gen 3 0 40961 4 12
r gen_W1_d0_D4 gen 1 0 40961 4 13
r gen_W2_d1_D4 gen 2 1 40961 4 12
r gen_W3_d1_D4 gen 3 1 40961 4 12
r gen_W1_d1_D4 gen 1 1 40961 4 13
