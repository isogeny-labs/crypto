#!/usr/bin/env bash
# Set S, PKM (none) arms. pod_run_S.sh lost these: with an empty --cover-map the line ended in a space,
# which `xargs -L 1` reads as a continuation, so each S3-none line swallowed its S4-none line (exit 101).
cd /work
BIN=./pkm_tower_pilot
COMMON="--p 2013265921 --engine f4 --kinds kummer --budget 3600 --ladder-t none --planted 1 --random 1"
arm() {
  local name=$1; shift
  echo "$(date -u +%FT%TZ) start $name" >> runs/progress.txt
  $BIN $COMMON "$@" --out runs/$name.jsonl > runs/$name.log 2>&1
  local rc=$?
  echo "$(date -u +%FT%TZ) end $name exit=$rc" >> runs/progress.txt
}
for s in 101 102 103 104 105; do
  arm S3-none-s$s --seed $s --m 3 --t-min 4 --max-t-m3 4 --controls tower,null --null-chain &
  arm S4-none-s$s --seed $s --m 4 --t-min 3 --max-t-m4 3 --controls tower &
done
wait
echo "$(date -u +%FT%TZ) set S none arms done" >> runs/progress.txt
