#!/usr/bin/env bash
# Finish a launch whose client was interrupted after `codebuild start-build`:
# wait for that exact launcher build, then fetch the result.json that
# remote/launch-remote would have downloaded (s3://BUCKET/runs/JOB/launcher/LAUNCH_ID/result.json).
# It never starts a build. The job's verdict is left to check_session_complete.sh.
#   wait_launcher_result.sh BUILD_ID JOB_NAME OUTPUT_DIR MAX_WAIT_SECONDS
# OUTPUT_DIR holds the build-start.json and request.json written by launch-remote.
# Needs AWS_REGION and ISOLAB_BUCKET.
set -euo pipefail
[[ $# == 4 ]] || { echo 'usage: wait_launcher_result.sh BUILD_ID JOB_NAME OUTPUT_DIR MAX_WAIT_SECONDS' >&2; exit 2; }
build_id=$1 job=$2 out=$3 max=$4
: "${ISOLAB_BUCKET:?}" "${AWS_REGION:?}"
[[ "$max" =~ ^[1-9][0-9]*$ ]] || { echo 'MAX_WAIT_SECONDS must be a positive integer' >&2; exit 2; }
jq -e --arg id "$build_id" '.build.id==$id' "$out/build-start.json" >/dev/null \
  || { echo 'FAIL: build-start.json does not name this build' >&2; exit 40; }
jq -e --arg job "$job" '.job.job_name==$job' "$out/request.json" >/dev/null \
  || { echo 'FAIL: request.json is not for this job' >&2; exit 40; }
digest=$(sha256sum "$out/request.json" | cut -d ' ' -f1)
status=IN_PROGRESS
while (( SECONDS < max )); do
  aws codebuild batch-get-builds --ids "$build_id" > "$out/build.json"
  status=$(jq -er '.builds[0].buildStatus' "$out/build.json")
  [[ "$status" != IN_PROGRESS ]] && break
  sleep 15
done
printf 'Launcher: %s status %s\n' "$build_id" "$status"
[[ "$status" != IN_PROGRESS ]] || { echo "FAIL: build still IN_PROGRESS after $max seconds" >&2; exit 41; }
launch_id=${build_id##*:}
uri="s3://$ISOLAB_BUCKET/runs/$job/launcher/$launch_id/"
printf 'Launcher receipts: %s\n' "$uri"
aws s3 cp "${uri}result.json" "$out/result.json" --only-show-errors \
  || { echo 'FAIL: no final receipt; inspect the saved build status and CloudWatch logs' >&2; exit 25; }
jq -e --arg job "$job" --arg digest "$digest" '.job_name==$job and .request_sha256==$digest' "$out/result.json" >/dev/null \
  || { echo 'FAIL: result.json does not match this job and request' >&2; exit 42; }
cat "$out/result.json"
