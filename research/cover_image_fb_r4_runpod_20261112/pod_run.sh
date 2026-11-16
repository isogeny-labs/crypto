#!/usr/bin/env bash
# Runs on the RunPod pod in /work. Tower cells of round 4 (PROTOCOL.md), three arms at a time.
cd /work
BIN=./pkm_tower_pilot
COMMON="--p 2013265921 --engine f4 --kinds kummer --budget 3600 --ladder-t none --planted 1 --random 1 --controls tower"
mkdir -p runs; sha256sum $BIN > runs/binary.sha256; nproc > runs/host.txt; free -g >> runs/host.txt
arm() {
  local name=$1; shift
  echo "$(date -u +%FT%TZ) start $name" >> runs/progress.txt
  $BIN $COMMON "$@" --out runs/$name.jsonl > runs/$name.log 2>&1
  local rc=$?
  echo "$(date -u +%FT%TZ) end $name exit=$rc" >> runs/progress.txt
}
export -f arm; export BIN COMMON
cat <<LIST | xargs -P 3 -L 1 bash -c 'arm "$@"' _
m3t5-none --m 3 --t-min 5 --max-t-m3 5
m3t5-cubic --m 3 --t-min 5 --max-t-m3 5 --cover-map cubic
m3t5-quart --m 3 --t-min 5 --max-t-m3 5 --cover-map quart
m4-none --m 4 --t-min 2 --max-t-m4 3
m4-cubic --m 4 --t-min 2 --max-t-m4 3 --cover-map cubic
m4-quart --m 4 --t-min 2 --max-t-m4 3 --cover-map quart
m3t5-generic3 --m 3 --t-min 5 --max-t-m3 5 --cover-map generic3
m4-generic3 --m 4 --t-min 2 --max-t-m4 3 --cover-map generic3
m3t5-quad --m 3 --t-min 5 --max-t-m3 5 --cover-map quad
LIST
echo "$(date -u +%FT%TZ) all done" >> runs/progress.txt
