#!/usr/bin/env bash
# SALVAGE: verify the downloaded result tree of a FAILED command Job (one pod, nonzero exit) that has no SHA256SUMS manifest.
#   verify_failed_nosums.sh STATUS_JSON LAUNCH_RESULT_JSON RESULTS_DIR LOCAL_SUMS_OUT FILE STRING [FILE STRING ...]
# STATUS_JSON, LAUNCH_RESULT_JSON, RESULTS_DIR: as in verify_results_lines.sh (RESULTS_DIR must hold exactly one
# directory, named by the Pod UID; the study's entry script syncs into that Pod UID prefix).
# LOCAL_SUMS_OUT: a file this script writes: sha256sum of every file under the Pod UID directory, sorted by path.
# Each FILE STRING pair: FILE (inside the Pod UID directory) must contain STRING as a fixed substring on some line.
# Exit codes: 50 Job not failed with one pod and a nonzero pod exit; 51 Job UID or name differs from the launch receipt;
# 52 results tree is not exactly one directory named by the Pod UID; 54 a FILE STRING pair is not satisfied.
set -euo pipefail
[[ $# -ge 6 && $(( ($# - 4) % 2 )) == 0 ]] || { echo 'usage: verify_failed_nosums.sh STATUS_JSON LAUNCH_RESULT_JSON RESULTS_DIR LOCAL_SUMS_OUT FILE STRING [FILE STRING ...]' >&2; exit 2; }
status=$1 launch=$2 dir=$3 sums=$4
shift 4
jq -e '.state=="failed" and (.pods|length)==1 and (.pods[0].exit_code|type)=="number" and .pods[0].exit_code!=0' "$status" >/dev/null \
  || { echo 'FAIL: Job is not failed with exactly one pod and a nonzero pod exit' >&2; exit 50; }
jq -e --slurpfile l "$launch" '(.job_uid|length)>0 and .job_uid==$l[0].job_uid and .job_name==$l[0].job_name' "$status" >/dev/null \
  || { echo 'FAIL: Job UID or name differs from the launch receipt' >&2; exit 51; }
uid=$(jq -er '.pods[0].uid' "$status")
count=$(find "$dir" -mindepth 1 -maxdepth 1 | wc -l)
[[ "$count" == 1 && -d "$dir/$uid" ]] || { echo "FAIL: $dir is not exactly one directory named $uid" >&2; exit 52; }
while [[ $# -gt 0 ]]; do
  file=$1 str=$2
  shift 2
  grep -qF -- "$str" "$dir/$uid/$file" || { echo "FAIL: $file does not contain: $str" >&2; exit 54; }
  echo "found in $file: $str"
done
(cd "$dir/$uid" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -d '\n' sha256sum) > "$sums"
echo "local sha256 lines: $(wc -l < "$sums") written to $sums"
echo "verified: job $(jq -r .job_name "$status") job_uid $(jq -r .job_uid "$status") pod_uid $uid"
