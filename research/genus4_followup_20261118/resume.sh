#!/usr/bin/env bash
# Deviation (logged before reading any Q result): run.sh's wrapper hit the 2 h background limit after
# 62 of 90 Q runs. Missing: curve 2640, k = 3..17 (28 keys). Re-run that curve's Q block whole; the two
# already-present keys are re-computed and must be identical (analyze.py's duplicate check). Output name
# b_q.jsonl.r1 matches analyze.py's frozen glob b_q.jsonl*.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
d=research/genus4_followup_20261118
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.fu
$B run --primes 16411 --curves 1 --curve-seeds 16411=2640 --targets 15 --target-from 3 --seeds 2 --arms Q --cc 2.0 \
  --genera 4 --max-h 2 --threads 4 --rho-cap 5e8 --out $d/b_q.jsonl.r1 > $d/b_q_r1.log 2>&1
echo done
