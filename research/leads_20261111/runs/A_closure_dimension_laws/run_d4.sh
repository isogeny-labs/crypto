#!/bin/bash
# Sequential D=4 cells (FULL=1, single-threaded, 1200 s timeout each): L4 positive control and fresh instances.
cd "$(dirname "$0")"
./run_cell.sh posL4 R 35 6 4
./run_cell.sh L4 R 35 7 4
./run_cell.sh L4 R 37 1 4
./run_cell.sh L4 R 37 3 4
./run_cell.sh L4 R 39 3 4
./run_cell.sh L4 R 39 7 4
