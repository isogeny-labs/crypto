#!/usr/bin/env bash
# Training sweep for cc* (PROTOCOL section 4). Curves 5001.., disjoint from the test curves.
set -euo pipefail
d=research/ic_positive_control_la_20261116
for cc in 0.7 1.0 1.4 2.0; do
  target/release/ic_positive_control run --primes 4096,8192,16384 --curves 2 --seeds 2 \
    --arms SR --cc $cc --threads 3 --curve-seed0 5001 --json $d/train_cc$cc.json > $d/train_cc$cc.log 2>&1
done
