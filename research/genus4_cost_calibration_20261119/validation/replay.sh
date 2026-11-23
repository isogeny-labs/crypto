#!/usr/bin/env bash
# Validator count replay (genus4_cost_calibration_20261119). Re-runs whole IC configuration files with
# the exact argv of run.sh's ic() (only --out differs), pinned with taskset -c 3 instead of -c 2.
# Counts are compared by compare_replay in recompute.py; wall times are not compared (core 3 is shared
# with a concurrent curve search at replay time).
set -euo pipefail
cd /home/user/crypto
B=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/genus_ladder.po.validator_copy
V=research/genus4_cost_calibration_20261119/validation/replay
mkdir -p $V
echo "binary $(sha256sum $B)"
for spec in 16411:2.0 16411:2.8 32771:1.4; do
  P=${spec%%:*}; cc=${spec##*:}
  cs=$([ $P = 16411 ] && echo 373/455 || echo 355/882)
  echo "start $P cc$cc $(date -u +%FT%TZ)"
  taskset -c 3 $B run --primes $P --genera 4 --max-h 1 --curves 2 --curve-seeds $P=$cs --targets 1 --seeds 2 --arms x \
    --cc $cc --sge 4,1.5 --sge 8,2 --threads 1 --out $V/replay_p${P}_cc$cc.jsonl > $V/replay_p${P}_cc$cc.log 2>&1
  echo "end $P cc$cc $(date -u +%FT%TZ)"
done
echo done
