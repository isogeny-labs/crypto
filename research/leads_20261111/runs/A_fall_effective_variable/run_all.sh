#!/bin/bash
# Sequential driver for the preregistered D=4 cells and controls (one job at a time, single thread,
# each under a 1200 s timeout).  Instances: first two unsatisfiable seeds per cell from out/instances.jsonl.
cd "$(dirname "$0")"
run() { timeout 1200 ./run_cell.sh "$@" || echo "{\"tag\": \"$7\", \"type\": \"$1\", \"n\": $2, \"k\": $3, \"seed\": $4, \"error\": \"timeout_or_fail\"}" >> out/cells.jsonl; }
run T 35 17 1 4 0 cell_i
run T 35 17 2 4 0 cell_i
run R 34 17 2 4 0 cell_ii
run R 34 17 4 4 0 cell_ii
run T 32 16 4 4 0 cell_iv
run R 36 18 1 4 0 cell_iii
run R 36 18 2 4 0 cell_iii
run R 35 18 6 4 0 pos_R35
run R 34 17 1 4 0 neg_sat
