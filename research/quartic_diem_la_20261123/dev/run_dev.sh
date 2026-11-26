#!/bin/sh
# PROTOCOL.md section 4 development grid (not in any fit). Run from the study directory.
set -e
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.qla
for W in 4 8; do for F in 1.5 3; do for E in 8 16 64; do
  $B run --primes 4093,8191,32771 --genera 2 --arms - --curve-seeds 4093=1,8191=1,32771=1 \
    --quartic --quartic-arms 2lp-sfb,2lp-sfb-sge --quartic-inhom 4 --quartic-sge $W,$F,$E \
    --quartic-seeds 4093=45/89,8191=20/106,32771=32/163/217/221/229 \
    --qc 3.0 --max-h 1 --curves 5 --seeds 2 --rho-cap 1 --threads 2 --out dev/dev_W${W}_F${F}_E${E}.jsonl
done; done; done
