#!/usr/bin/env bash
# Set S on one pod: 5 seeds x 5 maps x {S3, S4}, 16 arms at a time.
cd /work
BIN=./pkm_tower_pilot
COMMON="--p 2013265921 --engine f4 --kinds kummer --budget 3600 --ladder-t none --planted 1 --random 1"
mkdir -p runs; sha256sum $BIN > runs/binary.sha256; nproc > runs/host.txt; cat /sys/fs/cgroup/memory.max >> runs/host.txt 2>/dev/null
arm() {
  local name=$1; shift
  echo "$(date -u +%FT%TZ) start $name" >> runs/progress.txt
  $BIN $COMMON "$@" --out runs/$name.jsonl > runs/$name.log 2>&1
  local rc=$?
  echo "$(date -u +%FT%TZ) end $name exit=$rc" >> runs/progress.txt
}
export -f arm; export BIN COMMON
for s in 101 102 103 104 105; do
  for m in none quad cubic quart generic3; do
    cm=""; [ $m != none ] && cm="--cover-map $m"
    echo "S3-$m-s$s --seed $s --m 3 --t-min 4 --max-t-m3 4 --controls tower,null --null-chain $cm"
    echo "S4-$m-s$s --seed $s --m 4 --t-min 3 --max-t-m4 3 --controls tower $cm"
  done
done | xargs -P 16 -L 1 bash -c 'arm "$@"' _
echo "$(date -u +%FT%TZ) set S done" >> runs/progress.txt
