#!/usr/bin/env bash
# Resume p = 32771 after the 00:18 restart: the run.sh command unchanged (search from seed 1),
# written to p32771_resume.jsonl. Before the restart it admitted no curve (~95 seeds, ~85 s/seed).
set -euo pipefail
d=research/genus4_lean_rho_20261116
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.lean
c="--genera 4 --seeds 2 --cc 2.0 --max-h 2 --threads 1 --calibrate"
s32=$(python3 -c "print('/'.join(map(str,range(1,4001))))")
$B run --primes 32771 --curves 3 --targets 1 --arms wBQ $c --threads 2 --rho-cap 3e9 --curve-seeds 32771=$s32 --out $d/p32771_resume.jsonl > $d/p32771_resume.log 2>&1
echo resume32771-done
