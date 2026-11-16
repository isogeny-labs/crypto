#!/usr/bin/env bash
# Set S, the S3 quad arms. The same xargs continuation bug as pod_run_S_none.sh: the S4-none line also
# ended in a space, so each merged job swallowed the following S3-quad line too. One command each here.
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
  arm S3-quad-s$s --seed $s --m 3 --t-min 4 --max-t-m3 4 --controls tower,null --null-chain --cover-map quad &
done
wait
echo "$(date -u +%FT%TZ) set S quad3 arms done" >> runs/progress.txt
