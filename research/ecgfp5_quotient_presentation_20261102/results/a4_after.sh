#!/usr/bin/env bash
# A4: post-restart A/A cells after the A3 queue (results/a3_restart.sh) finishes.
cd "$(dirname "$0")/.."
while pgrep -f "bash results/a3_restart.sh" > /dev/null; do sleep 30; done
date -u +"%FT%TZ A4 start"
bash run_a1.sh A0_4111 U1F_4111
date -u +"%FT%TZ A4 done"
