#!/usr/bin/env bash
# Replay study 1 with the validator-rebuilt bce0eb2148 binary (1 thread; shared host):
# p = 2053, 4099 every arm (cvwR, 96 runs); p = 8209 arms c, v, w (36 runs).
set -u
G=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/g4val
B=${B:-$G/bin/genus_ladder.bce0eb2148}; TAG=${TAG:-}
sha256sum $B
cd $G/out
echo "start $(date -u +%FT%TZ)"
$B run --primes 2053,4099 --genera 4 --curves 3 --seeds 4 --arms cvwR --cc 2.0 --max-h 2 --threads 1 --rho-cap 1.2e8 \
  --curve-seeds 2053=40/60/109,4099=72/247/418 --out $G/out/replay_s1_2053_4099${TAG}.jsonl > $G/out/replay_s1_2053_4099${TAG}.log 2>&1
echo "rc=$? 2053/4099 done $(date -u +%FT%TZ)"
$B run --primes 8209 --genera 4 --curves 3 --seeds 4 --arms cvw --cc 2.0 --max-h 2 --threads 1 --rho-cap 1.2e8 \
  --curve-seeds 8209=363/393/407 --out $G/out/replay_s1_8209_cvw${TAG}.jsonl > $G/out/replay_s1_8209_cvw${TAG}.log 2>&1
echo "rc=$? 8209 cvw done $(date -u +%FT%TZ)"
for f in replay_s1_2053_4099${TAG}.jsonl replay_s1_8209_cvw${TAG}.jsonl; do
  python3 -I $G/scripts/compare_runs.py $G/out/$f /home/user/crypto/research/genus4_positive_control_20261116/run.jsonl
done
