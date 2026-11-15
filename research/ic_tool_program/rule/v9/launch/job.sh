#!/bin/bash
# Pod-side runner for rule v9 on Iso Lab EKS (launch/README.md).  Runs as UID
# 1000 in the study image; /work is the only writable tree.  The steps are
# ../run.sh's, in its order, minus the three that need git or only read the
# rows (claims, analyse, matched), which run afterwards on the checkout.
#   job.sh [S3 prefix of an earlier attempt to resume from]
# Rows are synced to $ISOLAB_RESULTS_URI every SYNC_SECONDS (60) and at exit,
# so a Spot loss costs at most one interval; `icprog rule size` resumes from
# the rows a run tree holds.
set -u
OUT=/work/results ROOT=/work/repo S=/opt/rulev9
mkdir -p "$OUT" "$ROOT"
LOG=$OUT/job.log
log() { printf '== %s %s\n' "$(date -u +%FT%TZ)" "$*" >> "$LOG"; }
R=$ROOT/research/ic_tool_program/rule
sync_up() {
  for c in v9 v9c; do
    [ -d "$R/$c/runs" ] && aws s3 sync --only-show-errors --exclude '*.tmp' "$R/$c/runs" "${ISOLAB_RESULTS_URI}rule/$c/runs"
  done
  aws s3 sync --only-show-errors "$OUT" "${ISOLAB_RESULTS_URI}job" || log "sync failed; the next interval retries"
}
finish() {
  local rc=$?
  trap - EXIT
  [ -n "${SYNC_PID:-}" ] && kill "$SYNC_PID" 2>/dev/null
  printf '{"exit_code":%s,"job_name":"%s","pod_uid":"%s","node_name":"%s","finished_at":"%s"}\n' \
    "$rc" "${ISOLAB_JOB_ID:-}" "${POD_UID:-}" "${NODE_NAME:-}" "$(date -u +%FT%TZ)" > "$OUT/job-exit.json"
  log "finished exit $rc"
  (cd "$R" && find v9/runs v9c/runs -type f ! -name '*.tmp' 2>/dev/null | LC_ALL=C sort | xargs -r sha256sum) > "$OUT/SHA256SUMS"
  for i in 1 2 3 4 5; do sync_up && break; sleep 20; done
  exit "$rc"
}
trap finish EXIT
trap 'exit 143' TERM
log "job ${ISOLAB_JOB_ID:-?} pod ${POD_UID:-?} node ${NODE_NAME:-?} start"
tar -xf $S/tree.tar -C "$ROOT"
cp $S/build-info.json "$OUT/build-info.json"
(cd $S/bin && sha256sum *) > "$OUT/binary.sha256"
cmp <(sort -k2 "$OUT/binary.sha256" | awk '{print $1"  "$2}') <(sort -k2 "$R/v9/binaries.sha256") \
  || { log "the image's binaries are not those binaries.sha256 names; stopped"; exit 3; }
{ grep -m1 'model name' /proc/cpuinfo; nproc; cat /sys/fs/cgroup/cpuset.cpus.effective 2>/dev/null; cat /sys/fs/cgroup/cpu.max 2>/dev/null; uname -r; } > "$OUT/host.txt"
env | grep -E '^(ISOLAB|POD_UID|NODE_NAME|RAYON|OMP|KIC|AWS_REGION|HOME)' | LC_ALL=C sort > "$OUT/env.txt"
if [ -n "${1:-}" ]; then
  for c in v9 v9c; do
    aws s3 sync --only-show-errors --exclude '*.tmp' "${1}rule/$c/runs" "$R/$c/runs" || { log "resume from $1 failed"; exit 4; }
  done
  log "resumed rows from $1"
fi
( while true; do sleep "${SYNC_SECONDS:-60}"; sync_up; done ) &
SYNC_PID=$!
cd "$ROOT"
P=$S/bin/icprog ISO=$S/bin/isolated_bench FIX=$S/bin/koblitz_rho_fixture
declare -A IC=([v9]=$S/bin/ic-v9 [v9c]=$S/bin/ic-v9c)
declare -A C=([v9]=f465f2fb1b037a57fa1d923ac8eea27ab9a603e2 [v9c]=370e19193f6f30bf04ca2e82a2c3626932dc069e)
FIXC=f465f2fb1b037a57fa1d923ac8eea27ab9a603e2
step() {
  log "$1"; shift
  "$@" >> "$LOG" 2>&1
  local rc=$?; echo "   exit $rc" >> "$LOG"
  [ $rc = 0 ] || { log "stopped"; exit 1; }
}
pin() {
  local out=$OUT/$1-pin.json
  log "$1 pin"
  $P rule pin --comparison $1 --ic ${IC[$1]} --ic-commit ${C[$1]} --isolate $ISO > "$out" 2>> "$LOG"
  echo "   exit $?" >> "$LOG"
  jq -e '.held == true and .names_agree == true' "$out" > /dev/null || { log "the pin failed for $1; stopped"; exit 1; }
}
unset KIC_RHO_CANON
for cmp in v9 v9c; do
  step "$cmp manifest" $P rule manifest --comparison $cmp --ic ${IC[$cmp]} --ic-commit ${C[$cmp]} --isolate $ISO
  pin $cmp
done
for size in 1,47 0,57 0,41 0,53 1,59 0,61; do
  for cmp in v9 v9c; do
    step "$cmp size $size" $P rule size --comparison $cmp --size $size --ic ${IC[$cmp]} --ic-commit ${C[$cmp]} --isolate $ISO
  done
done
for cmp in v9 v9c; do
  step "$cmp reference" $P rule reference --comparison $cmp --fixture $FIX --fixture-commit $FIXC --isolate $ISO
done
log "all pod steps done"
