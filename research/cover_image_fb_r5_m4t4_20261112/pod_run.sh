#!/usr/bin/env bash
# Runs on the RunPod pod in /work: round 5 (PROTOCOL.md), four arms at once.
cd /work
BIN=./pkm_tower_pilot
COMMON="--p 2013265921 --engine f4 --kinds kummer --budget 28800 --ladder-t none --planted 1 --random 0 --controls tower --m 4 --t-min 4 --max-t-m4 4"
mkdir -p runs; sha256sum $BIN > runs/binary.sha256; nproc > runs/host.txt; cat /sys/fs/cgroup/memory.max >> runs/host.txt 2>/dev/null
arm() {
  local name=$1; shift
  echo "$(date -u +%FT%TZ) start $name" >> runs/progress.txt
  $BIN $COMMON "$@" --out runs/$name.jsonl > runs/$name.log 2>&1
  local rc=$?
  echo "$(date -u +%FT%TZ) end $name exit=$rc" >> runs/progress.txt
}
export -f arm; export BIN COMMON
cat <<LIST | xargs -P 4 -L 1 bash -c 'arm "$@"' _
m4t4-none
m4t4-cubic --cover-map cubic
m4t4-quart --cover-map quart
m4t4-generic3 --cover-map generic3
LIST
echo "$(date -u +%FT%TZ) all done" >> runs/progress.txt
