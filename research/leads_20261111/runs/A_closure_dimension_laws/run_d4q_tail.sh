#!/bin/bash
# Replaces the tail of run_d4q.sh / run_d4q43.sh (batch 4096 was slower than 8192 at n=41: more batches).
# Waits for the running R41 s1 cell (batch 4096), then R41 s2 and R43 s3 with batch 8192.
cd "$(dirname "$0")"
while pgrep -f "run_cell.sh L4 R 41 1" > /dev/null; do sleep 15; done
export EXE=fallclose_q
./run_cell.sh L4 R 41 2 4 8192
./run_cell.sh L4 R 43 3 4 8192
