#!/usr/bin/env bash
# Amendment A4 (PROTOCOL.md §13): the remaining queue after the container restart of
# 2026-11-01 (between 06:11Z and 07:52Z), which killed run_probes.sh drive during arm F5a.
# The baseline 4111 cell had already finished and been checked; the caps below are the ones
# run_probes.sh derived from its quot.log (F4 elapsed 3074.88 s, cpu 10349.42 s).
#   run_queue.sh   baseline cells 262231 and 33554371 first, then F5a, F5b, W1, W3, W2
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
SBA_CAP_S=12996     # int(1.25 * 10349.42) + 60
W_CAP_S=3903        # int(1.25 * 3074.88) + 60
echo "queue start $(date -u +%FT%TZ)"
"$HERE/run_cells.sh" m5 262231 || true
"$HERE/run_cells.sh" m5 33554371 || true
"$HERE/run_probes.sh" sba F5a 0 1 "$SBA_CAP_S" || true
"$HERE/run_probes.sh" sba F5b 1 0 "$SBA_CAP_S" || true
for a in W1 W3 W2; do "$HERE/run_weighted.sh" arm "$a" "$W_CAP_S" || true; done
echo "queue finished $(date -u +%FT%TZ)"
