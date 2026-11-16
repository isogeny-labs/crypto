#!/bin/bash
# r2mllc image entry script (round 2, 2026-11-08). Runs the committed mllc harness for the sizes given and uploads
# its own outputs plus a SHA256SUMS to $ISOLAB_RESULTS_URI (the pod's /work is temporary).
# Usage: r2mllc_entry.sh SIZE...        e.g.  r2mllc_entry.sh 12 16     (one harness process per size, in parallel)
# Mirrors research/ml_learnability_control_20261107/mllc_job.sh (same environment, same harness call per size);
# that script is not used because it always runs all four sizes and does not upload.
# Exit status: 0 only if every size wrote its JSON with a decision table and the upload succeeded.
set -u
: "${ISOLAB_RESULTS_URI:?}"
export AWS_DEFAULT_REGION=${AWS_DEFAULT_REGION:-us-east-2}   # aws-cli v1 ignores AWS_REGION
[ "$#" -ge 1 ] || { echo "usage: $0 SIZE..." >&2; exit 2; }
for m in "$@"; do case "$m" in 12|16|20|24) ;; *) echo "bad size $m" >&2; exit 2;; esac; done
SRC=/opt/study/src
OUT=${ISOLAB_OUTPUT_DIR:-/work/results}; mkdir -p "$OUT" && cd "$SRC" || exit 2
export OMP_NUM_THREADS=2 OPENBLAS_NUM_THREADS=2 MKL_NUM_THREADS=2 PYTHONHASHSEED=0 PYTHONDONTWRITEBYTECODE=1

up() {  # upload everything in $OUT (files only, no deletion); SHA256SUMS regenerated first
  ( cd "$OUT" && find . -maxdepth 1 -type f ! -name SHA256SUMS -printf '%f\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS )
  aws s3 sync "$OUT" "$ISOLAB_RESULTS_URI" --only-show-errors
}

{ echo "argv: $*"; echo "start: $(date -u +%FT%TZ)"; echo "nproc: $(nproc)"; grep -m1 'model name' /proc/cpuinfo
  grep MemTotal /proc/meminfo; uname -r; cat /opt/study/build-info.json; } > "$OUT/host.txt" 2>&1
python3 -I -c "import numpy,scipy,sklearn,sys;print(sys.version,numpy.__version__,scipy.__version__,sklearn.__version__)" | tee "$OUT/versions.txt"
( cd "$SRC" && sha256sum PROTOCOL.md mllc_ecgen.py mllc_harness.py mllc_job.sh ) | tee "$OUT/source.sha256"
up

( while sleep 600; do up; done ) &
syncer=$!
pids=()
for m in "$@"; do
  python3 -u mllc_harness.py --sizes "$m" --out "$OUT/ladder_m$m.json" > "$OUT/ladder_m$m.log" 2>&1 &
  pids+=($!)
done
fail=0; i=0
for m in "$@"; do
  wait "${pids[$i]}"; rc=$?; i=$((i+1))
  echo "size $m exit $rc" >> "$OUT/exit_codes.txt"
  [ "$rc" = 0 ] && [ -s "$OUT/ladder_m$m.json" ] || fail=1
done
kill "$syncer" 2>/dev/null
echo "end: $(date -u +%FT%TZ) fail=$fail" >> "$OUT/exit_codes.txt"
up || fail=1
aws s3 ls "$ISOLAB_RESULTS_URI" > "$OUT/.listing" 2>&1 || fail=1
cat "$OUT/SHA256SUMS"
exit "$fail"
