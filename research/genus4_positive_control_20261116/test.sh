#!/usr/bin/env bash
# PROTOCOL section 3. Binary: build of the frozen sources (binary.sha256).
set -euo pipefail
d=research/genus4_positive_control_20261116
B=${GL_BIN:-/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/genus_ladder.g4}
$B run --primes 2053,4099,8209 --genera 4 --curves 3 --seeds 4 --arms cvwR \
  --cc 2.0 --max-h 2 --threads 3 --rho-cap 1.2e8 \
  --curve-seeds 2053=40/60/109,4099=72/247/418,8209=363/393/407 --out $d/run.jsonl > $d/run.log 2>&1
