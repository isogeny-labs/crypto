#!/usr/bin/env bash
# Replay study 2 with the validator-rebuilt a99e7267f1 binary (1 thread): arms w and R,
# 3 curves x 2 targets (cs, cs + 1000003) x 2 seeds = 12 w + 12 R runs.
set -u
G=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/g4val
B=${B:-$G/bin/genus_ladder.a99e7267f1}; TAG=${TAG:-}
sha256sum $B
echo "start $(date -u +%FT%TZ)"
$B run --primes 8209 --genera 4 --curves 3 --targets 2 --seeds 2 --arms wR --cc 2.0 --max-h 2 --threads 1 --rho-cap 1.2e8 \
  --curve-seeds 8209=363/393/407 --out $G/out/replay_s2_8209_wR_t2${TAG}.jsonl > $G/out/replay_s2_8209_wR_t2${TAG}.log 2>&1
echo "rc=$? done $(date -u +%FT%TZ)"
python3 -I $G/scripts/compare_runs.py $G/out/replay_s2_8209_wR_t2${TAG}.jsonl /home/user/crypto/research/genus4_rho_refs_20261116/run.jsonl
