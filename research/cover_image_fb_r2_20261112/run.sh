#!/usr/bin/env bash
# Cover-image probe, round 2 (PROTOCOL.md). One arm per process, four at a time.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"
D=research/cover_image_fb_r2_20261112
BIN=target/release/examples/pkm_tower_pilot
mkdir -p "$D/runs"
sha256sum "$BIN" > "$D/runs/binary.sha256"
git rev-parse HEAD > "$D/runs/commit.txt"
git diff --quiet HEAD -- examples/pkm_tower_pilot.rs || echo "dirty pilot source" >> "$D/runs/commit.txt"
COMMON="--p 2013265921 --engine f4 --kinds kummer --budget 600 --ladder-t none"
arm() {  # name, then the arm's flags
  local name=$1; shift
  echo "$(date -u +%FT%TZ) start $name" >> "$D/runs/progress.txt"
  $BIN $COMMON "$@" --out "$D/runs/$name.jsonl" > "$D/runs/$name.log" 2>&1
  echo "$(date -u +%FT%TZ) end $name exit=$?" >> "$D/runs/progress.txt"
}
export -f arm; export D BIN COMMON
M2="--m 2 --controls tower,null --t-min 3 --max-t 7"
M3="--m 3 --controls tower --t-min 2 --max-t-m3 3"
cat <<LIST | xargs -P 4 -L 1 bash -c 'arm "$@"' _
m2-cubic $M2 --cover-map cubic
m2-quart $M2 --cover-map quart
m2-generic3 $M2 --cover-map generic3
m2-fourth $M2 --cover-map fourth
m2-none $M2
m2-square $M2 --cover-map square
m2-quad $M2 --cover-map quad
m3-none $M3
m3-cubic $M3 --cover-map cubic
m3-quart $M3 --cover-map quart
m3-generic3 $M3 --cover-map generic3
LIST
echo "$(date -u +%FT%TZ) all done" >> "$D/runs/progress.txt"
