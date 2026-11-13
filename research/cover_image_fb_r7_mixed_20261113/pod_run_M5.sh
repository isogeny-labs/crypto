#!/usr/bin/env bash
# Round 7, M5: m = 5, t = 2 and 3, all maps in parallel (one explicit command each).
cd /work
BIN=./pkm_tower_pilot
mkdir -p runs; sha256sum $BIN > runs/binary.sha256; nproc > runs/host.txt; cat /sys/fs/cgroup/memory.max >> runs/host.txt 2>/dev/null
one() {
  local map=$1 cm=()
  [ "$map" != none ] && cm=(--cover-map "$map")
  local name=M5-$map
  echo "$(date -u +%FT%TZ) start $name" >> runs/progress.txt
  $BIN --p 2013265921 --engine f4 --kinds kummer --budget 7200 --ladder-t none --planted 1 --random 1 \
    --controls tower --m 5 --t-min 2 --max-t-m4 3 "${cm[@]}" --out runs/$name.jsonl > runs/$name.log 2>&1
  local rc=$?
  echo "$(date -u +%FT%TZ) end $name exit=$rc" >> runs/progress.txt
}
one none & one quad & one cubic & one quart & one generic3 &
wait
echo "$(date -u +%FT%TZ) set M5 done" >> runs/progress.txt
