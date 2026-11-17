#!/usr/bin/env bash
# Pod-side runner (Iso Lab EKS command Job, UID 1000, /work writable). Runs every unit of
# /opt/study/units.tsv (largest first, 4 at a time), the in-pod chain check and the per-size
# calibration, writes one JSONL per unit to $OUT/units/, and syncs $OUT to $ISOLAB_RESULTS_URI
# every 60 s and at exit. Existing unit files are skipped (infrastructure retries only).
set -uo pipefail
OUT=${ISOLAB_OUTPUT_DIR:-/work/results}
mkdir -p "$OUT/units" "$OUT/calib"
LOG="$OUT/job.log"
log() { printf '%s %s\n' "$(date -u +%FT%TZ)" "$*" | tee -a "$LOG" >&2; }
sync_up() { aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors >/dev/null 2>&1 || log "sync failed"; }
SYNC_PID=''
finish() {
  local rc=$?; trap - EXIT
  [ -n "$SYNC_PID" ] && kill "$SYNC_PID" 2>/dev/null
  printf '{"exit_code":%s,"job_name":"%s","pod_uid":"%s","node_name":"%s","finished_at":"%s"}\n' \
    "$rc" "${ISOLAB_JOB_ID:-}" "${POD_UID:-}" "${NODE_NAME:-}" "$(date -u +%FT%TZ)" > "$OUT/job-exit.json"
  log "finished exit $rc"
  (cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
  for i in 1 2 3 4 5; do aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors && break; sleep 20; done
  exit "$rc"
}
trap finish EXIT
trap 'exit 143' TERM
log "job ${ISOLAB_JOB_ID:-?} pod ${POD_UID:-?} node ${NODE_NAME:-?} start"
cp /opt/study/build-info.json "$OUT/build-info.json"
sha256sum /opt/study/bin/v2 /opt/study/bin/v3 /opt/study/units.tsv /opt/study/job.sh > "$OUT/binary.sha256"
aws sts get-caller-identity > "$OUT/runner-identity.json" 2>&1 || true
{ grep -m1 'model name' /proc/cpuinfo; nproc; cat /sys/fs/cgroup/cpuset.cpus.effective 2>/dev/null; cat /sys/fs/cgroup/cpu.max 2>/dev/null; } > "$OUT/host.txt"
( while true; do sleep 60; sync_up; done ) & SYNC_PID=$!
/opt/study/bin/v3 chaincheck 4051 8191 16363 32719 65479 131071 262111 524287 1048507 2097091 4194271 > "$OUT/chaincheck.jsonl" 2>&1
log "chaincheck rc $?"
run_unit() {
  local arm=$1 bin=$2 q=$3 s=$4 c=$5
  local f="$OUT/units/${arm}_${q}_${s}.jsonl"
  [ -s "$f" ] && return 0
  local t0; t0=$(date +%s%N)
  /opt/study/bin/"$bin" ic "$q" "$s" D 3.0 "$c" > "$f.tmp" 2> "$OUT/units/${arm}_${q}_${s}.err"
  local rc=$?
  printf '{"rec":"unit","arm":"%s","bin":"%s","q":%s,"seed":%s,"c":%s,"rc":%s,"wall_ms":%s}\n' \
    "$arm" "$bin" "$q" "$s" "$c" "$rc" "$(( ($(date +%s%N) - t0) / 1000000 ))" >> "$f.tmp"
  mv "$f.tmp" "$f"
}
export -f run_unit; export OUT
xargs -P 4 -L 1 bash -c 'run_unit "$@"' _ < /opt/study/units.tsv
log "units done"
for q in 4051 8191 16363 32719 65479 131071 262111 524287 1048507 2097091 4194271; do
  for c in 1 2 3; do /opt/study/bin/v2 calib "$q" "$c" >> "$OUT/calib/calib.jsonl" 2>/dev/null; done
done
log "calibration done"
n=$(ls "$OUT/units"/*.jsonl | wc -l); log "unit files $n"
[ "$n" -eq "$(wc -l < /opt/study/units.tsv)" ]
