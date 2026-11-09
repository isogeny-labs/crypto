#!/bin/bash
# r2img entry for minors_guess_determine_20261107 (round 2, 2026-11-08). Runs the frozen run_panel.sh UNCHANGED
# (48 shards x 25 instances, 8 arms, 6 sizes; merge; python3 -I minors_gd_analyze.py -> analysis.json) with
# JOBS parallel shard processes, syncing OUT to S3 every 600 s and at exit. Usage: entry.sh [JOBS=14]
set -u
: "${ISOLAB_RESULTS_URI:?}" "${ISOLAB_OUTPUT_DIR:?}"
D=${R2IMG_DIR:-/opt/minors}; OUT=$ISOLAB_OUTPUT_DIR/panel; JOBS=${1:-14}
mkdir -p "$OUT"; cd "$ISOLAB_OUTPUT_DIR" || exit 2
up() { aws s3 sync "$ISOLAB_OUTPUT_DIR" "$ISOLAB_RESULTS_URI" --only-show-errors; }
{ echo "start: $(date -u +%FT%TZ)"; echo "jobs: $JOBS"; echo "nproc: $(nproc)"; grep -m1 'model name' /proc/cpuinfo; grep MemTotal /proc/meminfo; uname -r
  cat "$D/build-info.json"; } > host.txt
sha256sum "$D/minors_gd" "$D/run_panel.sh" "$D/minors_gd_analyze.py" > binary.sha256
up
( while sleep 600; do up; done ) &
syncer=$!
bash "$D/run_panel.sh" "$D/minors_gd" "$OUT" "$JOBS" > run_panel.log 2>&1; rc=$?
kill "$syncer" 2>/dev/null
echo "end: $(date -u +%FT%TZ) run_panel exit $rc" > progress.txt
find . -type f ! -name SHA256SUMS | sort | sed 's|^\./||' | xargs sha256sum > SHA256SUMS
up
exit "$rc"
