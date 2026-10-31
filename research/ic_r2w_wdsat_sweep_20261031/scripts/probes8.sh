#!/bin/bash
export WDSAT_FIRST=1 WDSAT_BIN=/Volumes/SSD990-2/r2w-build/bins
P=target/release/examples/wdsat_probe
for c in "9 0" "9 1" "11 1" "13 0" "15 0"; do set -- $c
  $P $1 $2 8 3 4242 1 > probes/probe_first_n$1a$2l8.jsonl 2> probes/err_n$1a$2l8.txt
done
echo done > probes/DONE8
