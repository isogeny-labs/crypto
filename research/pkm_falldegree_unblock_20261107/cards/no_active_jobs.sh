#!/usr/bin/env bash
# Best-effort pool check: read the named Jobs through isolab.eks.command-status/v1 and require that each
# is missing, succeeded or failed (not pending, not active). The status route reads one Job by name; it cannot
# list Jobs, so a Job that is not named here is not seen. It never submits anything.
#   no_active_jobs.sh LAUNCHER OUT_DIR JOB_NAME [JOB_NAME ...]
# Exit 60: a named Job is pending or active. Exit 61: a status query gave no usable result. Exit 2: bad usage.
# Needs ISOLAB_BUCKET, ISOLAB_CODEBUILD_PROJECT, AWS_REGION in the environment.
set -uo pipefail
[[ $# -ge 3 ]] || { echo 'usage: no_active_jobs.sh LAUNCHER OUT_DIR JOB_NAME [JOB_NAME ...]' >&2; exit 2; }
launcher=$1 out=$2
shift 2
: "${ISOLAB_BUCKET:?}" "${ISOLAB_CODEBUILD_PROJECT:?}" "${AWS_REGION:?}"
mkdir -p "$out"
for job in "$@"; do
  [[ "$job" =~ ^[a-z0-9]([-a-z0-9]*[a-z0-9])?$ && ${#job} -le 55 ]] || { echo "FAIL: bad job name $job" >&2; exit 2; }
  req="$out/$job.status-request.json"
  jq -n --arg job "$job" '{schema:"isolab.eks.command-status/v1", job_name:$job}' > "$req"
  ISOLAB_CLIENT_OUTPUT="$out/$job" timeout 900 "$launcher" "$req" > "$out/$job.stdout" 2> "$out/$job.stderr"
  rc=$?
  if [[ -s "$out/$job/result.json" ]] && jq -e --arg job "$job" '.job_name==$job' "$out/$job/result.json" >/dev/null 2>&1; then
    state=$(jq -r '.state' "$out/$job/result.json")
  else
    state=none
  fi
  printf '%s job %s launcher_exit %d state %s\n' "$(date -u +%FT%TZ)" "$job" "$rc" "$state"
  case "$state" in
    missing|succeeded|failed) ;;
    pending|active) echo "FAIL: job $job is $state" >&2; exit 60 ;;
    *) echo "FAIL: no usable status for job $job" >&2; exit 61 ;;
  esac
done
echo "pool check: every named job is missing, succeeded or failed"
