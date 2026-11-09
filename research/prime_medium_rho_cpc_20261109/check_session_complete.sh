#!/usr/bin/env bash
# Mechanical encoding of PROTOCOL.md section 10 for one launched job: the job's
# session is admitted for counts when it is complete with a passing audit and
# the pod ended with exit 0, 42 or 43 (42 and 43 are the pod's L2 and wall A/A
# gates, which gate wall time and never counts). Anything else fails this script.
#   check_session_complete.sh JOB_NAME OUTPUT_DIR EXPECTED_EXECUTIONS
# OUTPUT_DIR holds the launcher's result.json. Needs AWS_REGION and ISOLAB_BUCKET.
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: check_session_complete.sh JOB_NAME OUTPUT_DIR EXPECTED_EXECUTIONS' >&2; exit 2; }
job=$1 out=$2 n=$3
: "${ISOLAB_BUCKET:?}" "${AWS_REGION:?}"
[[ "$n" =~ ^[1-9][0-9]*$ ]] || { echo 'EXPECTED_EXECUTIONS must be a positive integer' >&2; exit 2; }
jq -e --arg job "$job" '.schema=="isolab.eks.result/v1" and .job_name==$job
  and (.job_uid|length)>0 and (.receipt_uri|length)>0' "$out/result.json" >/dev/null \
  || { echo 'FAIL: result.json is not a launcher result for this job' >&2; exit 30; }
job_uid=$(jq -er .job_uid "$out/result.json")
receipt_uri=$(jq -er .receipt_uri "$out/result.json")
mkdir -p "$out/s3"
aws s3 cp "${receipt_uri}status.json" "$out/s3/status.json" --only-show-errors
prefixes=$(aws s3api list-objects-v2 --bucket "$ISOLAB_BUCKET" --prefix "runs/$job/" --delimiter / \
  --query 'CommonPrefixes[].Prefix' --output json)
pods=$(jq -c '[(. // [])[] | select(endswith("/launcher/")|not)]' <<<"$prefixes")
[[ $(jq length <<<"$pods") == 1 ]] || { echo "FAIL: expected exactly one pod prefix under runs/$job/, found: $pods" >&2; exit 31; }
pod=$(jq -er '.[0] | rtrimstr("/") | split("/") | last' <<<"$pods")
jq -e --arg juid "$job_uid" --arg pod "$pod" '.job.uid==$juid and ([.pods[].uid]==[$pod])' \
  "$out/s3/status.json" >/dev/null || { echo 'FAIL: Job or Pod UID mismatch between result.json, status.json and the S3 prefix' >&2; exit 32; }
jq -e --arg pod "$pod" '.pod_uid=="" or .pod_uid==$pod' "$out/result.json" >/dev/null \
  || { echo 'FAIL: result.json pod_uid differs from the S3 prefix' >&2; exit 32; }
base="s3://$ISOLAB_BUCKET/runs/$job/$pod"
for f in SHA256SUMS job-exit.json audit.json audit.stderr l2-gate.json; do
  aws s3 cp "$base/$f" "$out/s3/$f" --only-show-errors
done
(cd "$out/s3" && grep -E ' (job-exit\.json|audit\.json|audit\.stderr|l2-gate\.json)$' SHA256SUMS | sha256sum -c -) >/dev/null \
  || { echo 'FAIL: downloaded file hash differs from SHA256SUMS' >&2; exit 33; }
jq -e --arg job "$job" --arg pod "$pod" '.job_name==$job and .pod_uid==$pod
  and (.exit_code==0 or .exit_code==42 or .exit_code==43)' "$out/s3/job-exit.json" >/dev/null \
  || { echo "FAIL: job-exit.json is not exit 0, 42 or 43: $(cat "$out/s3/job-exit.json")" >&2; exit 34; }
jq -e --argjson n "$n" '.ok==true and .records_bound==true and .regraded==true
  and .session_status=="complete" and .planned_executions==$n and .records==$n
  and (.problems|length)==0 and (.replays|length)>0 and all(.replays[]; .reproduced==true)' \
  "$out/s3/audit.json" >/dev/null || { echo 'FAIL: audit.json is not a complete, passing audit of the expected executions' >&2; exit 35; }
grep -Fq ' 0 problems' "$out/s3/audit.stderr" || { echo 'FAIL: audit.stderr does not report 0 problems' >&2; exit 35; }
jq -n --arg job "$job" --arg job_uid "$job_uid" --arg pod "$pod" --arg base "$base/" --argjson n "$n" \
  --slurpfile exit "$out/s3/job-exit.json" --slurpfile audit "$out/s3/audit.json" --slurpfile gate "$out/s3/l2-gate.json" \
  '{schema:"prime-medium-rho-cpc/session-admission/v1",job_name:$job,job_uid:$job_uid,pod_uid:$pod,results_uri:$base,
    job_exit_code:$exit[0].exit_code,expected_executions:$n,audit_records:$audit[0].records,
    audit_replays:($audit[0].replays|length),l2_accepted:$gate[0].accepted,l2_below_l2:($gate[0].below_l2|length),
    admitted_for_counts:true}' > "$out/admission.json"
cat "$out/admission.json"
