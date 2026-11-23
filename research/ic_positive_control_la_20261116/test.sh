#!/usr/bin/env bash
# Held-out test (PROTOCOL sections 3, 5). cc* = 2.0 from TRAINING.md, so S* = S and is not run separately.
# Binary: the build of caae5eca9f (binary.sha256).
set -euo pipefail
d=research/ic_positive_control_la_20261116
B=${IC_BIN:-/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/ic_positive_control.caae5ec}
$B run --primes 128,256,512,1024,2048,4096,8192,16384,32768,65536 \
  --curves 5 --curves-map 4099=3,8209=3,16411=2,32771=2,65537=1 \
  --seeds 4 --arms MNSR --threads 3 --json $d/run.json > $d/run.log 2>&1
