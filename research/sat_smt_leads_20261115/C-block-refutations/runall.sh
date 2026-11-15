#!/bin/sh
# All measured runs, one process at a time (about 35 minutes on one core).  Usage: sh runall.sh
# Appends to raw/*.jsonl.gz: start from an empty raw/ directory.
cd "$(dirname "$0")"
for l in 6 7 8 9; do
  nice python3 run.py raw/sweep_l$l.jsonl.gz 540 $l sweep,rand,planted 2>> raw/log.txt
done
for l in 6 7 8 9; do
  nice python3 run.py raw/alpha_l$l.jsonl.gz 540 $l alpha 2>> raw/log.txt
done
