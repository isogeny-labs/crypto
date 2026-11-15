#!/bin/bash
# Sequential D=3 grid, single-threaded: positive controls (logged seeds), L3 test cells, T-type negatives.
cd "$(dirname "$0")"
for p in "15 1" "15 5" "15 6" "17 1" "17 2" "17 8" "19 4" "19 7" "21 3"; do set -- $p; ./run_cell.sh posL3 R $1 $2 3; done
for c in "23 4" "23 8" "25 5" "25 6" "27 3" "27 6" "29 3" "29 4" "31 2" "31 3" "33 1" "33 5"; do set -- $c; ./run_cell.sh L3 R $1 $2 3; done
for c in "23 2" "23 35" "27 12" "27 17" "31 16" "31 17" "33 2" "33 18"; do set -- $c; ./run_cell.sh negT T $1 $2 3; done
