#!/bin/sh
# All measured runs, one process at a time (about 40 minutes on one core).  Usage: sh runall.sh [steps]
# Appends to raw/*.jsonl.gz: start from an empty raw/ directory.
cd "$(dirname "$0")"
STEPS="${*:-sweep alpha explore}"
for s in $STEPS; do
  for l in 6 7 8 9; do
    if [ "$s" = sweep ]; then ph=sweep,rand,planted; else ph=$s; fi
    nice python3 cmeasure.py raw/${s}_l$l.jsonl.gz 540 $l $ph 2>> raw/log.txt
  done
done
