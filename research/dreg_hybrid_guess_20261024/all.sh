#!/usr/bin/env bash
# Resumable chain of tiers 0..5 (stop-and-restart safe).  all.sh BIN_DIR
HERE=$(cd "$(dirname "$0")" && pwd)
for t in 0 1 2 3 4 5; do "$HERE/run.sh" "$1" $t; done
echo "$(date -u +%FT%TZ) chain finished" | tee -a "$HERE/runs/queue.log"
