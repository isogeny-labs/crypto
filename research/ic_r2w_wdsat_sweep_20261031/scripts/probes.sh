#!/bin/bash
# per-system probes (first-solution mode, -x -b), brute-force root check, seed 4242
export WDSAT_FIRST=1 WDSAT_BIN=/Volumes/SSD990-2/r2w-build/bins
P=target/release/examples/wdsat_probe
for l in 3 4 5 6 7; do case $l in 3|4) N=24;; 5) N=8;; 6) N=3;; 7) N=2;; esac
 for c in "9 0" "9 1" "11 1" "13 0" "15 0"; do set -- $c
  $P $1 $2 $l $N 4242 1 > probes/probe_first_n$1a$2l$l.jsonl 2> probes/err_n$1a$2l$l.txt
 done; done
echo done > probes/DONE
