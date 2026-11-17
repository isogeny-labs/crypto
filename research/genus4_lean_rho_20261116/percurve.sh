#!/usr/bin/env bash
# One curve per process (resume after the 01:57 restart): arms and flags as run.sh, written to
# p<P>_c<CURVE>.jsonl. Rows also present in earlier files must match in every field but wall_ns.
# Usage: percurve.sh P CURVE THREADS RHOCAP [SUFFIX]  (SUFFIX keeps earlier partial files)
set -euo pipefail
sfx=${5:+_$5}
d=research/genus4_lean_rho_20261116
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.lean
$B run --primes $1 --curves 1 --genera 4 --seeds 2 --cc 2.0 --max-h 2 --calibrate --arms wBQ --targets 3 \
  --threads $3 --rho-cap $4 --curve-seeds $1=$2 --out $d/p$1_c$2$sfx.jsonl > $d/p$1_c$2$sfx.log 2>&1
echo "curve $1/$2 done"
