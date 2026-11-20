#!/usr/bin/env bash
# PROTOCOL section 3. Binary: build of the frozen sources (binary.sha256).
set -euo pipefail
d=research/ic_positive_control_capped_20261116
B=${IC_BIN:-/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/bin/ic_positive_control.capped}
$B run --primes 128,256,512,1024,2048,4096,8192,16384,32768,65536,131072 \
  --curves 5 --curves-map 4099=3,8209=3,16411=2,32771=2,65537=1,131101=1 \
  --seeds 4 --arms W --threads 3 --json $d/run_w.json > $d/run_w.log 2>&1
$B run --primes 128,256,512,1024,2048,4096,8192 \
  --curves 5 --curves-map 4099=3,8209=3 --seeds 4 --arms SR --threads 3 --json $d/run_sr.json > $d/run_sr.log 2>&1
