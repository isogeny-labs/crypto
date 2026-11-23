#!/usr/bin/env bash
# Cover-image factor-base probe (PROTOCOL.md). Four arms in parallel, one per core:
# the reference (no cover map) and the three cover maps; then m = 3.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
D=research/cover_image_fb_20261112
BIN=target/release/examples/pkm_tower_pilot
mkdir -p "$D/runs"
sha256sum "$BIN" > "$D/runs/binary.sha256"
git rev-parse HEAD > "$D/runs/commit.txt"
COMMON=(--p 2013265921 --engine f4 --kinds kummer,dickson --budget 600)
log() { echo "$(date -u +%FT%TZ) $*" >> "$D/runs/progress.txt"; }
arm() {  # name, extra flags...
  local name=$1; shift
  log "start $name"
  "$BIN" "${COMMON[@]}" "$@" --out "$D/runs/$name.jsonl" > "$D/runs/$name.log" 2>&1
  log "end $name exit=$?"
}
arm m2-none    --m 2 --controls tower,null,randbase --t-min 2 --max-t 8 --ladder-t none &
arm m2-square  --m 2 --controls tower,null,randbase --t-min 2 --max-t 8 --cover-map square &
arm m2-quad    --m 2 --controls tower,null,randbase --t-min 2 --max-t 8 --cover-map quad &
arm m2-generic --m 2 --controls tower,null,randbase --t-min 2 --max-t 8 --cover-map generic &
wait
arm m3-none    --m 3 --controls tower --t-min 2 --max-t-m3 4 --ladder-t none &
arm m3-square  --m 3 --controls tower --t-min 2 --max-t-m3 4 --cover-map square &
arm m3-quad    --m 3 --controls tower --t-min 2 --max-t-m3 4 --cover-map quad &
arm m3-generic --m 3 --controls tower --t-min 2 --max-t-m3 4 --cover-map generic &
wait
log "all done"
