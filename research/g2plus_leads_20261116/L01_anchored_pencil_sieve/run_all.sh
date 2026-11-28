#!/bin/bash
# Executes PROTOCOL.md section 10 (after the unit tests and gates, already run), logging each command.
cd /home/user/crypto/research/g2plus_leads_20261116/L01_anchored_pencil_sieve
for g in 2 3; do for p in 1031 4099 16411 65537; do for c in 1 2 3; do for s in 1 2; do
  ./logcmd.sh raw/rel_g${g}_p${p}_c${c}_s${s}.jsonl python3 run.py relphase --g $g --p $p --curve-index $c --seed $s \
    --arms P_full,P_naive1,B_lab --rows-dir raw/rows \
    --out raw/rel_g${g}_p${p}_c${c}_s${s}.jsonl
done; done; done; done
for s in 1 2; do
  ./logcmd.sh raw/aa_g2_p4099_c1_s${s}.jsonl python3 run.py relphase --g 2 --p 4099 --curve-index 1 --seed $s \
    --arms P_full,P_naive1,B_lab --out raw/aa_g2_p4099_c1_s${s}.jsonl
done
for g in 2 3; do for p in 4099 16411; do for c in 1 2 3; do
  ./logcmd.sh raw/lp2_g${g}_p${p}_c${c}.jsonl python3 run.py lp2 --g $g --p $p --curve-index $c --seed 1 \
    --out raw/lp2_g${g}_p${p}_c${c}.jsonl
done; done; done
for gp in "2 1031" "2 4099" "3 1031"; do set -- $gp; for c in 1 2 3; do
  ./logcmd.sh raw/la_g$1_p$2_c${c}.jsonl python3 run.py la --g $1 --p $2 --curve-index $c --seed 1 --rows-dir raw/rows \
    --out raw/la_g$1_p$2_c${c}.jsonl
done; done
echo ALLDONE > raw/ALLDONE
