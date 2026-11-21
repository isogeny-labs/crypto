#!/usr/bin/env bash
# Q (plain rho, no window) replays of the nonce plan with the validator's build, flags as run.sh/resume.sh.
set -u
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/fuval/bin/genus_ladder.val
O=/home/user/crypto/research/genus4_followup_20261118/validation/out/replay
mkdir -p $O
c="--genera 4 --max-h 2 --cc 2.0 --rho-cap 5e8 --arms Q"
echo "start $(date -u +%FT%TZ) $(sha256sum $B)"
$B run --primes 16411 --curves 1 --curve-seeds 16411=180 --targets 1 --target-from 7 --seeds 2 $c --threads 2 --out $O/q_180_k7.jsonl > $O/q_180_k7.log 2>&1 &
$B run --primes 16411 --curves 1 --curve-seeds 16411=172 --targets 1 --target-from 1 --seeds 1 $c --threads 1 --out $O/q_172_k1.jsonl > $O/q_172_k1.log 2>&1 &
$B run --primes 16411 --curves 1 --curve-seeds 16411=2640 --targets 1 --target-from 9 --seeds 1 $c --threads 1 --out $O/q_2640_k9.jsonl > $O/q_2640_k9.log 2>&1 &
wait
echo "end $(date -u +%FT%TZ)"
