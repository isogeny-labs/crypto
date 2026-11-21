#!/bin/bash
# Follow-up (post-hoc, POSTHOC_PREDICTION.md frozen before this ran): F1-F3 at D=4, then low-degree n=11,19.
cd "$(dirname "$0")"
while pgrep -f run_controls.sh > /dev/null || pgrep -f run_all.sh > /dev/null; do sleep 20; done
run() { timeout 1200 ./run_cell.sh "$@" || echo "{\"tag\": \"$7\", \"type\": \"$1\", \"n\": $2, \"k\": $3, \"seed\": $4, \"error\": \"timeout_or_fail\"}" >> out/cells.jsonl; }
run T 34 17 2 4 0 F1
run T 34 17 4 4 0 F1
run R 37 18 1 4 0 F3
run R 37 18 2 4 0 F3
run T 37 18 2 4 0 F2
run T 37 18 4 4 0 F2
