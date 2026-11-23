#!/bin/bash
cd /home/user/crypto/research/leads_followup_20261114/item07_exceptional_loci/tools
for p in "$@"; do (ulimit -v 6000000; OMP_NUM_THREADS=1 python3 sample_cubic.py $p 14 $((p*p)) ../data/samp${p}.jsonl); done
touch ../data/samp_$1.alldone
