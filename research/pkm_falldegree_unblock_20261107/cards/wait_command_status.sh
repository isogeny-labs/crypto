#!/usr/bin/env bash
# Wait for one command Job to reach a terminal state, using only
# isolab.eks.command-status/v1 reads (launch-remote-command with a status request).
# It never submits a Job and never starts anything but status reads, so it can be re-run safely.
#   wait_command_status.sh STATUS_REQUEST JOB_NAME OUT_DIR MAX_WAIT_SECONDS POLL_SECONDS LAUNCHER
# Each poll writes OUT_DIR/poll-<UTC time>/ (the launcher's own output) plus .stdout/.stderr files.
# On a terminal state (succeeded or failed) it copies that poll's result.json to OUT_DIR/final-status.json
# and exits 0; the verdict on the Job is left to verify_results.sh.
# Exit 40: the Job is missing. Exit 41: no terminal state within MAX_WAIT_SECONDS. Exit 2: bad usage.
# Needs ISOLAB_BUCKET, ISOLAB_CODEBUILD_PROJECT, AWS_REGION in the environment.
set -uo pipefail
[[ $# == 6 ]] || { echo 'usage: wait_command_status.sh STATUS_REQUEST JOB_NAME OUT_DIR MAX_WAIT_SECONDS POLL_SECONDS LAUNCHER' >&2; exit 2; }
req=$1 job=$2 out=$3 max=$4 poll=$5 launcher=$6
: "${ISOLAB_BUCKET:?}" "${ISOLAB_CODEBUILD_PROJECT:?}" "${AWS_REGION:?}"
[[ "$max" =~ ^[1-9][0-9]*$ && "$poll" =~ ^[1-9][0-9]*$ ]] || { echo 'MAX_WAIT_SECONDS and POLL_SECONDS must be positive integers' >&2; exit 2; }
jq -e --arg job "$job" '.schema=="isolab.eks.command-status/v1" and .job_name==$job' "$req" >/dev/null \
  || { echo 'FAIL: STATUS_REQUEST is not a command-status/v1 request for this job' >&2; exit 2; }
mkdir -p "$out"
start=$SECONDS n=0
while :; do
  n=$((n + 1))
  pdir="$out/poll-$(printf '%04d' "$n")-$(date -u +%Y%m%dT%H%M%SZ)"
  ISOLAB_CLIENT_OUTPUT="$pdir" timeout 1800 "$launcher" "$req" > "$pdir.stdout" 2> "$pdir.stderr"
  rc=$?
  state=none
  if [[ -s "$pdir/result.json" ]] && jq -e --arg job "$job" '.job_name==$job' "$pdir/result.json" >/dev/null 2>&1; then
    state=$(jq -r '.state' "$pdir/result.json")
  fi
  printf '%s poll %d launcher_exit %d state %s\n' "$(date -u +%FT%TZ)" "$n" "$rc" "$state"
  case "$state" in
    succeeded|failed)
      cp "$pdir/result.json" "$out/final-status.json"
      jq -c '{state, job_name, job_uid, pods, launcher, receipt_uri}' "$out/final-status.json"
      exit 0 ;;
    missing)
      echo 'FAIL: the Job is missing' >&2
      exit 40 ;;
  esac
  if (( SECONDS - start >= max )); then
    echo "FAIL: no terminal state within $max seconds (last state $state)" >&2
    exit 41
  fi
  sleep "$poll"
done
