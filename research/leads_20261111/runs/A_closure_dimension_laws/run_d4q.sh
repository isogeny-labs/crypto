#!/bin/bash
# Sequential D=4 cells with fallclose_q (byte-for-byte validated against fallclose: out/validate_q.txt and the
# posL4 cross-check below).  FULL=1, single-threaded, 1200 s timeout each.  Waits for the fallclose R35 s7 cell.
cd "$(dirname "$0")"
while pgrep -f "run_cell.sh L4 R 35 7" > /dev/null; do sleep 15; done
export EXE=fallclose_q
./run_cell.sh posL4q R 35 6 4
./run_cell.sh L4 R 37 1 4
./run_cell.sh L4 R 37 3 4
./run_cell.sh L4 R 39 3 4
./run_cell.sh L4 R 39 7 4
./run_cell.sh negT4 T 35 2 4
./run_cell.sh negT4 T 37 2 4
./run_cell.sh L4 R 41 1 4 4096
./run_cell.sh L4 R 41 2 4 4096
