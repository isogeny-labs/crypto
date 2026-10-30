#!/bin/bash
# usage: run_ladder.sh <n> <session numbers...>   (sessions/n<n>-s<i>)
BIN=${BIN:-/dev/shm/tgt-q/release/ecbench}
cd "$(dirname "$0")"
n=$1; shift
for i in "$@"; do
  d="sessions/n$n-s$i"
  [ -d "$d" ] && continue
  flock -s /dev/shm/cpu.lock "$BIN" run --spec "specs/n$n-s$i.json" --out "$d" --allow-busy --wait > "sessions/n$n-s$i.log" 2>&1
  tail -1 "sessions/n$n-s$i.log"
done
