#!/usr/bin/env bash
# Set L: one m = 4, t = 4 arm on this pod. Usage: pod_run_L.sh NAME SEED [--cover-map MAP]
cd /work
BIN=./pkm_tower_pilot
name=$1; seed=$2; shift 2
mkdir -p runs; sha256sum $BIN > runs/binary.sha256; nproc > runs/host.txt; cat /sys/fs/cgroup/memory.max >> runs/host.txt 2>/dev/null
echo "$(date -u +%FT%TZ) start $name" >> runs/progress.txt
$BIN --p 2013265921 --engine f4 --kinds kummer --budget 25200 --ladder-t none --planted 1 --random 0 \
  --controls tower --m 4 --t-min 4 --max-t-m4 4 --seed $seed "$@" --out runs/$name.jsonl > runs/$name.log 2>&1
rc=$?
echo "$(date -u +%FT%TZ) end $name exit=$rc" >> runs/progress.txt
