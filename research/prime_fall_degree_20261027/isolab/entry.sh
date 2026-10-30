#!/bin/bash
# Usage: entry.sh <stage>... ; runs run.sh stages in /work, uploads runs/ to S3.
set -u
: "${ISOLAB_RESULTS_URI:?}"
mkdir -p /work/study && cp /opt/study/run.sh /work/study/run.sh && cd /work/study
export BIN=/usr/local/bin/pkm_tower_pilot
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-$(nproc)}
up() { aws s3 sync /work/study/runs "$ISOLAB_RESULTS_URI/runs" --only-show-errors; aws s3 cp /work/study/progress.txt "$ISOLAB_RESULTS_URI/progress.txt" --only-show-errors 2>/dev/null; }
( while sleep 300; do up; done ) &
rc=0
for s in "$@"; do bash ./run.sh "$s" || rc=$?; done
kill %1 2>/dev/null; up
sha256sum /usr/local/bin/pkm_tower_pilot > /work/binary.sha256
aws s3 cp /work/binary.sha256 "$ISOLAB_RESULTS_URI/binary.sha256" --only-show-errors
exit $rc
