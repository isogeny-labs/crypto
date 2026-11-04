#!/usr/bin/env bash
D=$(cd "$(dirname "$0")" && pwd)
for t in 6 7 8 9 10; do for G in $(seq 4 $((t-1))); do "$D/run_cells.sh" s1 20 $t $G $((2026111600+t)) 600; done; done
for t in 6 7 8; do "$D/run_cells.sh" repro 20 $t 4 $((2026111400+t)) 600; done
