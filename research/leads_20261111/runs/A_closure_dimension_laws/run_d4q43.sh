#!/bin/bash
# ADDED after the n=35..39 cells (still a preregistered L4 prediction: n=43 -> 5805); runs after run_d4q.sh.
cd "$(dirname "$0")"
while pgrep -f run_d4q.sh > /dev/null; do sleep 20; done
export EXE=fallclose_q
./run_cell.sh L4 R 43 3 4 4096
