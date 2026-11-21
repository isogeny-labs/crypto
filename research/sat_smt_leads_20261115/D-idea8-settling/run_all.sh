#!/bin/sh
# Measured runs, one process at a time, one core, each process under 10 minutes.
set -e
cd "$(dirname "$0")"
for s in 0 1 2; do nice python3 run.py 5 $s A raw_l5_s${s}_A.jsonl; done
N=$(python3 nstar.py 5 .); echo "Nstar l=5: $N"
for s in 0 1 2; do nice python3 run.py 5 $s BL raw_l5_s${s}_BL.jsonl 1 $N; done
for s in 0 1 2; do nice python3 run.py 5 $s BE raw_l5_s${s}_BE.jsonl 1 $N; done
for s in 0 1 2; do nice python3 run.py 4 $s ALL raw_l4_s${s}_ALL.jsonl; done
echo ALLDONE
