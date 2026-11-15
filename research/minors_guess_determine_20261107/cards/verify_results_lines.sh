#!/usr/bin/env bash
# Verify a downloaded command-Job result tree against the Job's status and launch receipts, then check
# fixed strings in named files of the result directory.
#   verify_results_lines.sh STATUS_JSON LAUNCH_RESULT_JSON RESULTS_DIR FILE STRING [FILE STRING ...]
# STATUS_JSON: final-status.json written by wait_command_status.sh.
# LAUNCH_RESULT_JSON: the result.json written by launch-remote-command for the submission.
# RESULTS_DIR: the download of s3://BUCKET/runs/JOB/ (without launcher/); it must hold exactly one
# directory, named by the Pod UID of the status receipt.
# Each FILE STRING pair: FILE (inside that directory) must contain STRING as a fixed substring on some line.
# Exit codes: 50 Job not succeeded with pod exit 0; 51 Job UID or name differs from the launch receipt;
# 52 results tree is not exactly one directory named by the Pod UID; 53 SHA256SUMS missing or empty;
# 54 a FILE STRING pair is not satisfied; 55 sha256sum -c failed (every listed file must exist).
set -euo pipefail
[[ $# -ge 5 && $(( ($# - 3) % 2 )) == 0 ]] || { echo 'usage: verify_results_lines.sh STATUS_JSON LAUNCH_RESULT_JSON RESULTS_DIR FILE STRING [FILE STRING ...]' >&2; exit 2; }
status=$1 launch=$2 dir=$3
shift 3
jq -e '.state=="succeeded" and (.pods|length)==1 and .pods[0].exit_code==0' "$status" >/dev/null \
  || { echo 'FAIL: Job is not succeeded with exactly one pod and pod exit 0' >&2; exit 50; }
jq -e --slurpfile l "$launch" '(.job_uid|length)>0 and .job_uid==$l[0].job_uid and .job_name==$l[0].job_name' "$status" >/dev/null \
  || { echo 'FAIL: Job UID or name differs from the launch receipt' >&2; exit 51; }
uid=$(jq -er '.pods[0].uid' "$status")
count=$(find "$dir" -mindepth 1 -maxdepth 1 | wc -l)
[[ "$count" == 1 && -d "$dir/$uid" ]] || { echo "FAIL: $dir is not exactly one directory named $uid" >&2; exit 52; }
[[ -s "$dir/$uid/SHA256SUMS" ]] || { echo 'FAIL: SHA256SUMS missing or empty' >&2; exit 53; }
while [[ $# -gt 0 ]]; do
  file=$1 str=$2
  shift 2
  grep -qF -- "$str" "$dir/$uid/$file" || { echo "FAIL: $file does not contain: $str" >&2; exit 54; }
  echo "found in $file: $str"
done
(cd "$dir/$uid" && sha256sum -c SHA256SUMS) || { echo 'FAIL: sha256sum -c' >&2; exit 55; }
echo "SHA256SUMS lines: $(wc -l < "$dir/$uid/SHA256SUMS")"
echo "verified: job $(jq -r .job_name "$status") job_uid $(jq -r .job_uid "$status") pod_uid $uid"
