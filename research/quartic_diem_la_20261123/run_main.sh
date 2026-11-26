#!/bin/sh
# PROTOCOL.md section 5 (run from this study directory). DRAFT until frozen.
set -e
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.qla
$B run --primes 4099,16411,65537 --genera 2 --arms - \
  --curve-seeds 4099=1,16411=1,65537=1 \
  --quartic --quartic-arms 2lp-sfb,2lp-sfb-sge --quartic-inhom 4 --quartic-sge W,FILL,EXCESS \
  --quartic-seeds 4099=35/205/220/257/295,16411=93/158/252/366/382,65537=13/57/156/166/369 \
  --qc 3.0 --max-h 1 --curves 5 --seeds 2 --rho-cap 1 --threads 2 --out raw/run.jsonl
