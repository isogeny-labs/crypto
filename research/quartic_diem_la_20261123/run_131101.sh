#!/bin/sh
# PROTOCOL.md section 5 (run from this study directory). DRAFT until frozen.
set -e
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.qla
$B run --primes 131101 --genera 2 --arms - --curve-seeds 131101=1 \
  --quartic --quartic-arms 2lp-sfb,2lp-sfb-sge --quartic-inhom 4 --quartic-sge W,FILL,EXCESS \
  --quartic-seeds 131101=53/177/193/228/288 \
  --qc 3.0 --max-h 1 --curves 5 --seeds 2 --rho-cap 1 --threads 1 --out raw/run131101.jsonl
