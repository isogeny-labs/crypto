#!/bin/bash
# usage: run_verify.sh <session>...   replays every measured run in a fresh process
BIN=${BIN:-/dev/shm/tgt-q/release/ecbench}
cd "$(dirname "$0")"
for s in "$@"; do
  flock -s /dev/shm/cpu.lock "$BIN" verify --dir sessions/$s --replay-all --exit-code --out verify/$s.verify.json > verify/$s.log 2>&1
  echo "verify $s exit=$?" >> verify/status.txt
done
