#!/usr/bin/env bash
# Round 7, T: m = 4, t = 3 cost, one process at a time, three passes in fixed orders.
cd /work
BIN=./pkm_tower_pilot
mkdir -p runs; sha256sum $BIN > runs/binary.sha256; nproc > runs/host.txt; cat /sys/fs/cgroup/memory.max >> runs/host.txt 2>/dev/null
one() {
  local pass=$1 map=$2 cm=()
  [ "$map" != none ] && cm=(--cover-map "$map")
  local name=T-p$pass-$map
  echo "$(date -u +%FT%TZ) start $name" >> runs/progress.txt
  $BIN --p 2013265921 --engine f4 --kinds kummer --budget 3600 --ladder-t none --planted 1 --random 1 \
    --controls tower --m 4 --t-min 3 --max-t-m4 3 "${cm[@]}" --out runs/$name.jsonl > runs/$name.log 2>&1
  local rc=$?
  echo "$(date -u +%FT%TZ) end $name exit=$rc" >> runs/progress.txt
}
for map in none quad cubic quart generic3; do one 1 $map; done
for map in generic3 quart cubic quad none; do one 2 $map; done
for map in cubic none generic3 quad quart; do one 3 $map; done
echo "$(date -u +%FT%TZ) set T done" >> runs/progress.txt
