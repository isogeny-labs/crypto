#!/bin/bash
# A/A second instance at n=43 (preregistered "two instances per n"); fallclose_q, batch 8192.
cd "$(dirname "$0")"
EXE=fallclose_q ./run_cell.sh L4 R 43 5 4 8192
