#!/usr/bin/env bash
# Amendment 1: set B only (m = 4, t = 2 and 3).
cd /work
BIN=./pkm_tower_pilot
COMMON="--p 2013265921 --engine f4 --kinds kummer --budget 3600 --ladder-t none --planted 1 --random 1 --controls tower"
arm() {
  local name=$1; shift
  echo "$(date -u +%FT%TZ) start $name" >> runs/progress.txt
  $BIN $COMMON "$@" --out runs/$name.jsonl > runs/$name.log 2>&1
  local rc=$?
  echo "$(date -u +%FT%TZ) end $name exit=$rc" >> runs/progress.txt
}
export -f arm; export BIN COMMON
cat <<LIST | xargs -P 3 -L 1 bash -c 'arm "$@"' _
m4-none --m 4 --t-min 2 --max-t-m4 3
m4-cubic --m 4 --t-min 2 --max-t-m4 3 --cover-map cubic
m4-quart --m 4 --t-min 2 --max-t-m4 3 --cover-map quart
m4-generic3 --m 4 --t-min 2 --max-t-m4 3 --cover-map generic3
LIST
echo "$(date -u +%FT%TZ) set B done" >> runs/progress.txt
