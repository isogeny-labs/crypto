#!/usr/bin/env bash
# Amendment 1: the null controls at t <= 4, after the tower pass.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"
D=research/cover_image_fb_r3_m3_20261112
BIN=target/release/examples/pkm_tower_pilot
COMMON="--p 2013265921 --engine f4 --kinds kummer --budget 1800 --ladder-t none"
NULL="--m 3 --controls null --null-chain --t-min 2 --max-t-m3 4"
arm() {
  local name=$1; shift
  echo "$(date -u +%FT%TZ) start $name" >> "$D/runs/progress.txt"
  $BIN $COMMON "$@" --out "$D/runs/$name.jsonl" > "$D/runs/$name.log" 2>&1
  local rc=$?
  echo "$(date -u +%FT%TZ) end $name exit=$rc" >> "$D/runs/progress.txt"
}
export -f arm; export D BIN COMMON
cat <<LIST | xargs -P 3 -L 1 bash -c 'arm "$@"' _
m3null-none $NULL
m3null-cubic $NULL --cover-map cubic
m3null-quart $NULL --cover-map quart
m3null-generic3 $NULL --cover-map generic3
m3null-quad $NULL --cover-map quad
LIST
echo "$(date -u +%FT%TZ) nulls done" >> "$D/runs/progress.txt"
