#!/bin/bash
# lanes run in parallel; each is sequential over sizes ascending, seeds 5 6 7
cd "$(dirname "$0")"
( for b in 24 28 32 36; do for s in 5 6 7; do python3 run.py bits j0 $b $s raw/prime_j0.jsonl; done; done > raw/lane_j0.log 2>&1 ) &
( for b in 24 28 32; do for s in 5 6 7; do python3 run.py bits j1728 $b $s raw/prime_j1728.jsonl; done; done > raw/lane_j1728.log 2>&1 ) &
( for d in 20 22 24 26; do for s in 5 6 7; do python3 run.py char2 random $d $s raw/binary.jsonl; done; done > raw/lane_bin.log 2>&1 ) &
wait
