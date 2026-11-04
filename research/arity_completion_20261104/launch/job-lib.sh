#!/usr/bin/env bash
# Shared pod-side helpers for the arity-completion command Jobs on Iso Lab EKS.
# Sourced by job-*.sh. Runs as UID 1000 in the study image; /work is the only
# writable tree. Results go to $ISOLAB_OUTPUT_DIR and are synced to
# $ISOLAB_RESULTS_URI every SYNC_SECONDS (default 60) and once more at exit,
# so a Spot interruption loses at most one interval of records.
OUT=${ISOLAB_OUTPUT_DIR:-/work/results}
mkdir -p "$OUT"
LOG="$OUT/job.log"
SYNC_PID=''
log() { printf '%s %s\n' "$(date -u +%FT%TZ)" "$*" | tee -a "$LOG" >&2; }
# ISOLAB_CRIU=1 (CRIU/EBS command mode): the payload tree must hold no network
# sockets at dump time and the pod has no AWS identity, so nothing here talks to
# S3; the supervisor uploads $OUT after the computation exits.
CRIU=${ISOLAB_CRIU:-0}
sync_up() {
  [ "$CRIU" = 1 ] && return 0
  aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors >/dev/null 2>&1 \
    || log "sync to $ISOLAB_RESULTS_URI failed; next interval retries"
}
start_sync_loop() {
  [ "$CRIU" = 1 ] && return 0
  ( while true; do sleep "${SYNC_SECONDS:-60}"; sync_up; done ) &
  SYNC_PID=$!
}
finish() {
  local rc=$?
  trap - EXIT
  [ -n "$SYNC_PID" ] && kill "$SYNC_PID" 2>/dev/null
  printf '{"exit_code":%s,"job_name":"%s","pod_uid":"%s","node_name":"%s","finished_at":"%s"}\n' \
    "$rc" "${ISOLAB_JOB_ID:-}" "${POD_UID:-}" "${NODE_NAME:-}" "$(date -u +%FT%TZ)" > "$OUT/job-exit.json"
  log "finished exit $rc"
  (cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
  local i
  if [ "$CRIU" != 1 ]; then
    for i in 1 2 3 4 5; do
      aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors && break
      sleep 20
    done
  fi
  exit "$rc"
}
preamble() {
  trap finish EXIT
  trap 'exit 143' TERM
  log "job ${ISOLAB_JOB_ID:-?} pod ${POD_UID:-?} node ${NODE_NAME:-?} start"
  cp /opt/study/build-info.json "$OUT/build-info.json"
  sha256sum /usr/local/bin/n83_msweep_cell /usr/local/bin/pkm_tower_pilot > "$OUT/binary.sha256"
  [ "$CRIU" = 1 ] || aws sts get-caller-identity > "$OUT/runner-identity.json" 2>&1 || true
  { grep -m1 'model name' /proc/cpuinfo; nproc; cat /sys/fs/cgroup/cpuset.cpus.effective 2>/dev/null; cat /sys/fs/cgroup/cpu.max 2>/dev/null; } > "$OUT/host.txt"
  env | grep -E '^(ISOLAB|POD_UID|NODE_NAME|RAYON|OMP|AWS_REGION|HOME)' | LC_ALL=C sort > "$OUT/env.txt"
  start_sync_loop
}
# resume_from S3_PREFIX: pre-download a previous attempt's records so a frozen
# runner that skips existing records continues instead of re-running them.
resume_from() {
  [ -n "${1:-}" ] || return 0
  if [ -d "$1" ]; then
    (cd "$1" && find . -type f ! -name job-exit.json ! -name SHA256SUMS ! -name job.log ! -name env.txt ! -name host.txt \
      -exec cp --parents -p {} "$OUT/" \;)
  else
    aws s3 cp "$1" "$OUT/" --recursive --only-show-errors \
      --exclude 'job-exit.json' --exclude 'SHA256SUMS' --exclude 'job.log' --exclude 'env.txt' --exclude 'host.txt'
  fi
  log "resumed records from $1"
}
