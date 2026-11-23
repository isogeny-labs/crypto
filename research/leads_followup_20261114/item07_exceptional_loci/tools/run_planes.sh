#!/bin/bash
# usage: run_planes.sh worker ; runs primes 23 29 31 37 split 2 ways
w=$1
cd /home/user/crypto/research/leads_followup_20261114/item07_exceptional_loci/tools
for p in 23 29 31 37; do
  (ulimit -v 6000000; OMP_NUM_THREADS=1 python3 plane.py $p 1 $w 2 ../data/plane${p}_t1_w$w.jsonl)
done
touch ../data/planes_train.done$w
