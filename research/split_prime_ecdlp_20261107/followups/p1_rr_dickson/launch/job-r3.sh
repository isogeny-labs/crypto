#!/usr/bin/env bash
# job-r3.sh SYS -- pod-side entry for P1 round 3 (PROTOCOL-R3.md) on Iso Lab EKS, one system per Job.
# Runs as UID 1000; /work is the only writable tree. Results go to $ISOLAB_OUTPUT_DIR and sync to
# $ISOLAB_RESULTS_URI every 60 s and at exit (pattern: research/arity_completion_20261104/launch/job-lib.sh).
set -u
SYS=${1:?usage: job-r3.sh rr|t}
OUT=${ISOLAB_OUTPUT_DIR:-/work/results}; mkdir -p "$OUT"; LOG="$OUT/job.log"; SYNC_PID=''
export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1
log() { printf '%s %s\n' "$(date -u +%FT%TZ)" "$*" | tee -a "$LOG" >&2; }
sync_up() { aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors >/dev/null 2>&1 || log "sync failed; next interval retries"; }
finish() {
  local rc=$?; trap - EXIT; [ -n "$SYNC_PID" ] && kill "$SYNC_PID" 2>/dev/null
  printf '{"exit_code":%s,"system":"%s","job_name":"%s","pod_uid":"%s","node_name":"%s","finished_at":"%s"}\n' \
    "$rc" "$SYS" "${ISOLAB_JOB_ID:-}" "${POD_UID:-}" "${NODE_NAME:-}" "$(date -u +%FT%TZ)" > "$OUT/job-exit.json"
  log "finished exit $rc"
  (cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
  for i in 1 2 3 4 5; do aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors && break; sleep 20; done
  exit "$rc"
}
trap finish EXIT; trap 'exit 143' TERM
log "job ${ISOLAB_JOB_ID:-?} pod ${POD_UID:-?} node ${NODE_NAME:-?} system $SYS start"
cp /opt/study/build-info.json "$OUT/build-info.json"
aws sts get-caller-identity > "$OUT/runner-identity.json" 2>&1 || true
{ grep -m1 'model name' /proc/cpuinfo; nproc; cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max 2>/dev/null; } > "$OUT/host.txt"
env | grep -E '^(ISOLAB|POD_UID|NODE_NAME|RAYON|OMP|AWS_REGION|HOME)' | LC_ALL=C sort > "$OUT/env.txt"
( while true; do sleep 60; sync_up; done ) & SYNC_PID=$!
case "$SYS" in
  rr) VL=62914560 ;;
  t)  VL=58720256 ;;
  *)  log "HARD_ERROR: system must be rr or t"; exit 1 ;;
esac
P1TOOL=/usr/local/bin/p1tool OUT_DIR="$OUT/$SYS" VLIMIT=$VL TLIMIT=86400 \
  bash /opt/study/calibration/run_cells_r3.sh "$SYS" 2>&1 | tee -a "$LOG"
exit "${PIPESTATUS[0]}"
