#!/bin/bash
# Second sequential driver (starts when run_all.sh has finished): preregistered positive control T31,
# negative control T35 satisfiable, and the ADDED (not preregistered) same-N control T33 (treatise: stall).
cd "$(dirname "$0")"
while pgrep -f run_all.sh > /dev/null; do sleep 20; done
run() { timeout 1200 ./run_cell.sh "$@" || echo "{\"tag\": \"$7\", \"type\": \"$1\", \"n\": $2, \"k\": $3, \"seed\": $4, \"error\": \"timeout_or_fail\"}" >> out/cells.jsonl; }
run T 31 16 16 4 0 pos_T31
run T 33 17 2 4 0 added_T33
run T 33 17 18 4 0 added_T33
run T 35 17 4 4 0 neg_sat
