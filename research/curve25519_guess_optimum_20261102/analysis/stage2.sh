#!/usr/bin/env bash
# Stage 2 cells from the rule frozen in PROTOCOL.md applied to stage 1: (t*,G*) = (10,9), edge.
D=$(cd "$(dirname "$0")" && pwd)
for k in 24 28; do
  s=$((2026111700 + (k-24)/4*100))
  for G in 7 8 9; do "$D/run_cells.sh" s2a $k 10 $G $((s+10)) 900; done
done
"$D/run_cells.sh" s2b 24 12 9  2026111712 900
"$D/run_cells.sh" s2b 24 12 10 2026111712 900
"$D/run_cells.sh" s2b 24 12 11 2026111712 900
for G in 11 12 13; do "$D/run_cells.sh" s2b 28 14 $G 2026111814 900; done
