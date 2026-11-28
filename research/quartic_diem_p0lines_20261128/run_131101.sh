#!/bin/sh
# PROTOCOL.md section 3 (run from this study directory).
set -e
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.p0
$B run --primes 131101 --genera 2 --arms - --curve-seeds 131101=1 \
  --quartic --quartic-arms 2lp-sfb-sge-nofb,2lp-sfb-sge-p0,2lp-sfb-sge-p0i3 --quartic-inhom 4 --quartic-sge 8,1.5,64 \
  --quartic-seeds 131101=53/177/193/228/288 \
  --qc 3.0 --max-h 1 --curves 5 --seeds 6 --rho-cap 1 --threads 1 --out raw/run131101.jsonl
