#!/usr/bin/env bash
# Pod-side command for the first counted algebraic-track row (NOT RUN; see BLOCKERS.md).
# Runs inside the study image (ecbench built at the frozen commit, this file and the
# frozen spec layered on top, as research/arity_completion_20261104/launch did).
#   job-algebraic-row.sh run SPEC.json SESSION_NAME
#   job-algebraic-row.sh replay S3_PREFIX_OF_JOB1_RESULTS SESSION_NAME   (independent replay: another host class)
# Gates: ecbench plan must succeed (every curve registered=true, every method
# resolves) before the run; the session audit must be OK with every replay identical.
set -uo pipefail
MODE=${1:?run|replay}; ARG=${2:?spec path or S3 prefix}; NAME=${3:?session name}
ECB=/usr/local/bin/ecbench
OUT=${ISOLAB_OUTPUT_DIR:-/work/results}; mkdir -p "$OUT"
log() { printf '%s %s\n' "$(date -u +%FT%TZ)" "$*" | tee -a "$OUT/job.log" >&2; }
sync_up() { aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors >/dev/null 2>&1 || log "sync failed; next interval retries"; }
( while true; do sleep 60; sync_up; done ) & SYNC=$!
finish() {
  rc=$?; trap - EXIT; kill "$SYNC" 2>/dev/null
  printf '{"exit_code":%s,"job_name":"%s","pod_uid":"%s","node_name":"%s","finished_at":"%s"}\n' \
    "$rc" "${ISOLAB_JOB_ID:-}" "${POD_UID:-}" "${NODE_NAME:-}" "$(date -u +%FT%TZ)" > "$OUT/job-exit.json"
  (cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
  for i in 1 2 3 4 5; do aws s3 cp "$OUT/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors && break; sleep 20; done
  exit "$rc"
}
trap finish EXIT; trap 'exit 143' TERM
log "job ${ISOLAB_JOB_ID:-?} pod ${POD_UID:-?} node ${NODE_NAME:-?} start"
cp /opt/study/build-info.json "$OUT/build-info.json" 2>/dev/null || true
sha256sum "$ECB" > "$OUT/inputs.sha256"
{ grep -m1 'model name' /proc/cpuinfo; nproc; cat /sys/fs/cgroup/cpu.max 2>/dev/null; } > "$OUT/host.txt"
"$ECB" host > "$OUT/ecbench-host.json" 2>&1 || true
if [ "$MODE" = replay ]; then
  aws s3 cp "$ARG/session-$NAME/" "$OUT/session-$NAME/" --recursive --only-show-errors || exit 3
  (cd "$OUT/session-$NAME" && sha256sum records.jsonl spec.json session.json >> "$OUT/inputs.sha256")
  "$ECB" verify --dir "$OUT/session-$NAME" --replay-all --exit-code --out "$OUT/session-$NAME.independent-audit.json" > "$OUT/verify.log" 2>&1
  vrc=$?; log "independent ecbench verify --replay-all exit $vrc"; exit "$vrc"
fi
SPEC=$ARG; sha256sum "$SPEC" >> "$OUT/inputs.sha256"
"$ECB" plan --spec "$SPEC" > "$OUT/plan.txt" 2> "$OUT/plan.err" || { log "ecbench plan refused the spec"; exit 2; }
log "plan ok"
"$ECB" run --wait --cpus inherit --spec "$SPEC" --out "$OUT/session-$NAME" > "$OUT/run.log" 2>&1
rc=$?; log "ecbench run exit $rc"
[ -d "$OUT/session-$NAME" ] || exit "${rc:-1}"
"$ECB" verify --dir "$OUT/session-$NAME" --replay-all --exit-code --out "$OUT/session-$NAME.audit.json" > "$OUT/verify.log" 2>&1
vrc=$?; log "ecbench verify --replay-all exit $vrc"
[ "$rc" -eq 0 ] && exit "$vrc"
exit "$rc"
