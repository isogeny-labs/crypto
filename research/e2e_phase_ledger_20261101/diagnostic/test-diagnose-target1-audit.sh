#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

bucket=fixture-bucket
job=gh-41343654905-1
launcher=a518dca6-0894-4dce-bc3c-5bf02a165242
source=32303a6d51d7a43ce887cd9da8f6d82afea4793c
binary=f3aab23417ece64cb3373e5c66d2c3be343e18c0e89347f626e0aea18ab1b774
workflow=41343654905
job_uid=d5c4f94f-7171-46a0-a14f-e5cb7e213dca
pod=9a1f74a1-2ae0-4dd3-ab1e-51fa771eca69
diagnostic=11111111-2222-4333-8444-555555555555
launcher_dir="$tmp/s3/$bucket/runs/$job/launcher/$launcher"
pod_dir="$tmp/s3/$bucket/runs/$job/$pod"
mkdir -p "$launcher_dir" "$pod_dir" "$tmp/bin"
ln -s "$(realpath "$here/../../../scripts/tests/fake-aws.sh")" "$tmp/bin/aws"

jq -n --arg job "$job" --arg source "$source" --arg binary "$binary" '
  {schema:"isolab.eks.request/v1",job:{job_name:$job,spec:{schema:"ecbench.spec/v1",label:"fixture"},
   build:{image_uri:"104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
   source_revision:$source,ecbench_sha256:$binary}},provenance:{source_revision:$source}}
' > "$launcher_dir/request.json"
request=$(sha256sum "$launcher_dir/request.json" | cut -d ' ' -f1)
jq -n --arg job "$job" --arg launcher "$launcher" --arg request "$request" \
  --arg source "$source" --arg binary "$binary" --arg job_uid "$job_uid" '
  {schema:"isolab.eks.result/v1",state:"failed",job_name:$job,job_uid:$job_uid,
   pod_uid:"",results_uri:"",receipt_uri:"s3://fixture",request_sha256:$request,
   launcher:("isolab-experiment-launcher:"+$launcher),exit_code:23,
   build:{image_uri:"104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
   source_revision:$source,ecbench_sha256:$binary}}
' > "$launcher_dir/result.json"
jq -n --arg job "$job" --arg job_uid "$job_uid" --arg pod "$pod" '
  {job:{job_name:$job,uid:$job_uid,state:"failed"},pods:[{uid:$pod,
   status:{phase:"Failed",containerStatuses:[{state:{terminated:{exitCode:41}}}]}}]}
' > "$launcher_dir/status.json"

jq -n --arg job "$job" --arg pod "$pod" \
  '{job_name:$job,pod_uid:$pod,exit_code:41}' > "$pod_dir/job-exit.json"
jq -n --arg source "$source" --arg binary "$binary" \
  '{source_revision:$source,ecbench_sha256:$binary}' > "$pod_dir/build-info.json"
printf '%s  /usr/local/bin/ecbench\n' "$binary" > "$pod_dir/binary.sha256"
jq '.job.spec' "$launcher_dir/request.json" > "$pod_dir/input-spec.json"
jq -n '{ok:false,regraded:true,records_bound:true,records:364,verified_records:363,
  replays:[{reproduced:true},{reproduced:false}],
  problems:["private predicate detail", "private path /work/results/session/records.jsonl"]}' \
  > "$pod_dir/audit.json"
(cd "$pod_dir" && sha256sum job-exit.json build-info.json binary.sha256 input-spec.json audit.json) \
  > "$pod_dir/SHA256SUMS"
names=(SHA256SUMS job-exit.json build-info.json binary.sha256 input-spec.json audit.json)
jq -n --arg job "$job" --arg pod "$pod" --argjson names \
  "$(printf '%s\n' "${names[@]}" | jq -Rsc 'split("\n")[:-1]')" '
  {objects:[$names[]|{Key:("runs/"+$job+"/"+$pod+"/"+.)}]}
' > "$launcher_dir/objects.json"

PATH="$tmp/bin:$PATH" FAKE_S3_ROOT="$tmp/s3" ISOLAB_BUCKET="$bucket" \
  CODEBUILD_BUILD_ID="isolab-experiment-launcher:$diagnostic" \
  "$here/diagnose-target1-audit" "$job" "$launcher" "$request" "$source" "$binary" \
  "$workflow" "$job_uid" "$pod" > "$tmp/output-key.txt"
output="$tmp/s3/$bucket/runs/$job/launcher/$diagnostic/audit-summary.json"
jq -e --arg request "$request" --arg diagnostic "$diagnostic" '
  .schema=="isolab.eks.audit-receipt-diagnosis/v1"
  and .request_sha256==$request and .diagnostic_launcher_id==$diagnostic
  and .manifest_verified==true and .pod_exit_code==41
  and .audit=={ok:false,regraded:true,records_bound:true,records:364,
    verified_records:363,replay_count:2,reproduced:1,problem_count:2}
  and .gate_predicates=={ok:false,regraded:true,records_bound:true,
    records_positive:true,verified_records_equal_records:false,
    replay_count_positive:true,all_replays_reproduced:false}
  and .failed_predicates==["ok","verified_records_equal_records","all_replays_reproduced"]
  and .experiment_retry==false
' "$output" >/dev/null
! rg -q 'private predicate detail|private path|records\.jsonl' "$output"
[[ $(<"$tmp/output-key.txt") == "runs/$job/launcher/$diagnostic/audit-summary.json" ]]

# Every selected receipt is fail-closed under a manifest mismatch.
printf '0%.0s' {1..64} > "$pod_dir/audit.json"
if PATH="$tmp/bin:$PATH" FAKE_S3_ROOT="$tmp/s3" ISOLAB_BUCKET="$bucket" \
  CODEBUILD_BUILD_ID="isolab-experiment-launcher:$diagnostic" \
  "$here/diagnose-target1-audit" "$job" "$launcher" "$request" "$source" "$binary" \
  "$workflow" "$job_uid" "$pod" >/dev/null 2>&1; then
  echo 'diagnosis accepted a receipt whose manifest hash did not match' >&2
  exit 1
fi
printf 'Fixed target-1 audit diagnosis and privacy controls passed\n'
