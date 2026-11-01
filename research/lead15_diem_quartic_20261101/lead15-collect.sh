#!/usr/bin/env bash
# Build one LEAD-15 completion receipt only from locally retained, byte-for-byte
# control-plane and S3 artifacts. This script never submits a Job, downloads an
# object, or writes to S3. The native analyzer must independently repeat every
# hash and semantic check before admitting a cell.
set -euo pipefail

die() {
  echo "lead15-collect: $*" >&2
  exit 1
}

usage() {
  echo "usage: lead15-collect <q> <frozen-source-manifest.json> <evidence-directory>" >&2
  exit 2
}

[[ $# == 3 ]] || usage
q=$1
manifest=$2
evidence=$3
case "$q" in 257|2053|32771|4194301) ;; *) usage ;; esac
[[ -f "$manifest" && ! -L "$manifest" ]] || die "manifest must be a regular, non-symlink file"
[[ -d "$evidence" && ! -L "$evidence" ]] || die "evidence directory must be a real directory"
evidence=$(cd "$evidence" && pwd -P)
output=$evidence/completion-receipt.json
[[ ! -e "$output" ]] || die "$output already exists"

required=(request.json request-head.json submission.json job.json status.json results.json object-heads.json)
for name in "${required[@]}"; do
  [[ -f "$evidence/$name" && ! -L "$evidence/$name" ]] || die "missing regular evidence file $name"
  jq -e . "$evidence/$name" >/dev/null || die "invalid JSON in $name"
done
jq -e . "$manifest" >/dev/null || die "invalid source manifest JSON"

sha() { sha256sum "$1" | awk '{print $1}'; }
size() { stat -c %s "$1"; }
local_descriptor() {
  local name=$1 path=$evidence/$1
  jq -cn --arg path "$name" --arg sha "$(sha "$path")" --argjson bytes "$(size "$path")" \
    '{path:$path,sha256:$sha,bytes:$bytes}'
}
optional_local_descriptor() {
  local name=$1
  if [[ -f "$evidence/$name" && ! -L "$evidence/$name" ]]; then
    local_descriptor "$name"
  elif [[ -e "$evidence/$name" ]]; then
    die "$name is not a regular, non-symlink file"
  else
    printf 'null\n'
  fi
}

manifest_sha=$(sha "$manifest")
collector_sha=$(sha "$0")
jq -e --argjson q "$q" --arg collector "$collector_sha" '
  .schema == "lead15.source-manifest/v1" and .status == "frozen" and
  .protocol_id == "LEAD15-DT-E5-20261101" and
  .launch_tool == "isolab_k8s_run" and .scientific_deadline_seconds == 3600 and
  .outer_job_deadline_seconds == 7200 and .termination_grace_seconds == 60 and
  .retries == 0 and .threads_per_cell == 1 and .isolation == "none" and
  .requested_resources == {cpu:4,memory_gib:16,spot:true} and
  .scientific_environment == {RAYON_NUM_THREADS:"1",OMP_NUM_THREADS:"1"} and
  .collector_source_sha256 == $collector and
  ([.jobs[] | select(.q == $q)] | length) == 1
' "$manifest" >/dev/null || die "frozen manifest or collector identity mismatch"

job=$(jq -c --argjson q "$q" '.jobs[] | select(.q == $q)' "$manifest")
job_name=$(jq -er '.job_name' <<<"$job")
request_uri=$(jq -er '.request_artifact_s3_uri' <<<"$job")
results_root=$(jq -er '.results_prefix' <<<"$job")
image_uri=$(jq -er '.image_uri' "$manifest")
image_digest=$(jq -er '.image_digest' "$manifest")
instrument_commit=$(jq -er '.instrument_commit' "$manifest")
protocol_sha=$(jq -er '.protocol_sha256' "$manifest")
binary_sha=$(jq -er '.binary_sha256' "$manifest")
worker_sha=$(jq -er '.worker_source_sha256' "$manifest")
build_info_sha=$(jq -er '.build_info_sha256' "$manifest")
[[ "$image_uri" == *@"$image_digest" ]] || die "manifest image URI/digest mismatch"
[[ "$results_root" == "s3://isolab-results-104186811093-us-east-2/runs/$job_name/" ]] || die "manifest results prefix mismatch"

expected_argv=$(jq -ce --arg manifest "$manifest_sha" --arg q "$q" \
  'if .q != ($q|tonumber) then error("q mismatch") else
   .argv_template | map(if . == "${SOURCE_MANIFEST_SHA256}" then $manifest else . end) end' <<<"$job")
jq -e --arg q "$q" --arg manifest "$manifest_sha" --arg job "$job_name" \
  --arg image "$image_uri" --argjson argv "$expected_argv" '
  .schema == "lead15.isolab-request/v1" and .tool == "isolab_k8s_run" and
  .source_manifest_sha256 == $manifest and
  .arguments == {job_name:$job,command:$argv,image:$image,cpu:4,memory:"16Gi",seconds:7200,retries:0} and
  .arguments.command[1] == $q
' "$evidence/request.json" >/dev/null || die "request bytes do not match the frozen launch contract"
request_sha=$(sha "$evidence/request.json")
request_bytes=$(size "$evidence/request.json")
request_etag=$(jq -er '.ETag | select(type == "string" and length > 1)' "$evidence/request-head.json")
request_version=$(jq -er '.VersionId | select(type == "string" and length > 0)' "$evidence/request-head.json")
jq -e --argjson bytes "$request_bytes" '.ContentLength == $bytes' "$evidence/request-head.json" >/dev/null \
  || die "request S3 head length does not match local bytes"
request_artifact=$(jq -cn --arg path request.json --arg sha "$request_sha" --argjson bytes "$request_bytes" \
  --arg uri "$request_uri" --arg etag "$request_etag" --arg version "$request_version" \
  '{path:$path,sha256:$sha,bytes:$bytes,s3_uri:$uri,etag:$etag,version_id:$version,readback_verified:true}')

jq -e --arg job "$job_name" --arg results "$results_root" '
  .job.job_name == $job and (.job.uid | type == "string" and length > 0) and
  .results_uri == $results
' "$evidence/submission.json" >/dev/null || die "submission receipt mismatch"
job_uid=$(jq -er '.job.uid' "$evidence/submission.json")

# Verify the exact raw Kubernetes Job, not a copied summary of selected fields.
jq -e --arg job "$job_name" --arg uid "$job_uid" --arg image "$image_uri" \
  --arg results "${results_root}\$(POD_UID)/" --argjson argv "$expected_argv" '
  .apiVersion == "batch/v1" and .kind == "Job" and
  .metadata.name == $job and .metadata.uid == $uid and
  .metadata.namespace == "isolab" and .metadata.labels["isolab.dev/job"] == "true" and
  .spec.backoffLimit == 0 and .spec.activeDeadlineSeconds == 7200 and
  .spec.template.spec.restartPolicy == "Never" and
  .spec.template.spec.serviceAccountName == "runner" and
  .spec.template.spec.automountServiceAccountToken == false and
  .spec.template.spec.terminationGracePeriodSeconds == 60 and
  (.spec.template.spec.containers | length) == 1 and
  .spec.template.spec.containers[0].name == "task" and
  .spec.template.spec.containers[0].workingDir == "/work" and
  .spec.template.spec.containers[0].image == $image and
  .spec.template.spec.containers[0].command == $argv and
  .spec.template.spec.containers[0].resources.requests.cpu == "4" and
  .spec.template.spec.containers[0].resources.limits.cpu == "4" and
  .spec.template.spec.containers[0].resources.requests.memory == "16Gi" and
  .spec.template.spec.containers[0].resources.limits.memory == "16Gi" and
  .spec.template.spec.containers[0].resources.requests["ephemeral-storage"] == "10Gi" and
  .spec.template.spec.containers[0].resources.limits["ephemeral-storage"] == "10Gi" and
  .spec.template.spec.nodeSelector == {"isolab.dev/pool":"cpu-spot"} and
  all(.spec.template.spec.containers[0].env[];
    ((.valueFrom.fieldRef.apiVersion? // "v1") == "v1")) and
  (.spec.template.spec.containers[0].env | map(del(.valueFrom.fieldRef.apiVersion))) == [
    {name:"HOME",value:"/work"},{name:"AWS_REGION",value:"us-east-2"},
    {name:"AWS_DEFAULT_REGION",value:"us-east-2"},
    {name:"POD_UID",valueFrom:{fieldRef:{fieldPath:"metadata.uid"}}},
    {name:"NODE_NAME",valueFrom:{fieldRef:{fieldPath:"spec.nodeName"}}},
    {name:"ISOLAB_JOB_ID",value:$job},{name:"ISOLAB_OUTPUT_DIR",value:"/work/results"},
    {name:"ISOLAB_RESULTS_URI",value:$results},{name:"RAYON_NUM_THREADS",value:"4"},
    {name:"OMP_NUM_THREADS",value:"4"}]
' "$evidence/job.json" >/dev/null || die "raw Job bytes violate the frozen launch contract"

jq -e --arg job "$job_name" --arg uid "$job_uid" '
  .job.job_name == $job and .job.uid == $uid and
  (.job.state == "succeeded" or .job.state == "failed") and
  (.pods | type == "array" and length <= 1)
' "$evidence/status.json" >/dev/null || die "status is nonterminal, retried, or belongs to another Job"
job_state=$(jq -er '.job.state' "$evidence/status.json")
attempts=$(jq '.pods | length' "$evidence/status.json")

pod_json=null
pod_uid=''
pod_name=''
node_name=''
pod_restarts=null
if [[ $attempts == 1 ]]; then
  [[ -f "$evidence/pod.json" && ! -L "$evidence/pod.json" ]] || die "raw pod.json is required for a created Pod"
  jq -e . "$evidence/pod.json" >/dev/null || die "invalid pod.json"
  pod_uid=$(jq -er '.pods[0].uid' "$evidence/status.json")
  pod_name=$(jq -er '.pods[0].name' "$evidence/status.json")
  node_name=$(jq -r '.pods[0].node // ""' "$evidence/status.json")
  jq -e --slurpfile desired "$evidence/job.json" --arg job_uid "$job_uid" --arg pod "$pod_name" --arg uid "$pod_uid" \
    --arg image "$image_uri" --argjson argv "$expected_argv" '
    .metadata.name == $pod and .metadata.uid == $uid and
    ([.metadata.ownerReferences[] | select(.uid == $job_uid and .controller == true)] | length) == 1 and
    (.spec.containers | length) == 1 and .spec.containers[0].image == $image and
    .spec.containers[0].command == $argv and
    .spec.containers[0].env == $desired[0].spec.template.spec.containers[0].env and
    .spec.containers[0].resources == $desired[0].spec.template.spec.containers[0].resources
  ' "$evidence/pod.json" >/dev/null || die "raw Pod bytes violate Job/image/command identity"
  if [[ -n "$node_name" ]]; then
    jq -e --arg node "$node_name" --arg digest "$image_digest" '
      .spec.nodeName == $node and (.status.containerStatuses | length) == 1 and
      .status.containerStatuses[0].restartCount == 0 and
      (.status.containerStatuses[0].imageID | endswith("@" + $digest))
    ' "$evidence/pod.json" >/dev/null || die "scheduled Pod placement/runtime image mismatch"
    pod_restarts=0
    pod_json=$(jq -cn --arg name "$pod_name" --arg uid "$pod_uid" --arg node "$node_name" \
      '{name:$name,uid:$uid,node_name:$node}')
  else
    jq -e '.spec.nodeName == null' "$evidence/pod.json" >/dev/null || die "Pod placement snapshots disagree"
    pod_json=$(jq -cn --arg name "$pod_name" --arg uid "$pod_uid" '{name:$name,uid:$uid,node_name:null}')
  fi
else
  [[ ! -e "$evidence/pod.json" ]] || die "pod.json exists although status reports no Pod"
fi

jq -e --arg results "$results_root" --arg job "$job_name" '
  .results_uri == $results and (.objects | type == "array") and .next_token == null and
  ([.objects[].Key] | length == (unique | length)) and
  ([.objects[].Key | select(startswith("runs/" + $job + "/") | not)] | length) == 0
' "$evidence/results.json" >/dev/null || die "results listing is incomplete, duplicated, or misbound"
jq -e --arg results "$results_root" '
  .schema == "lead15.object-heads/v1" and .results_prefix == $results and (.objects | type == "object")
' "$evidence/object-heads.json" >/dev/null || die "object head bundle mismatch"

remote_descriptor() {
  local name=$1 path=$evidence/$1 key
  [[ -n "$pod_uid" ]] || die "$name exists without a Pod identity"
  key="runs/$job_name/$pod_uid/$name"
  [[ -f "$path" && ! -L "$path" ]] || die "$name must be a regular, non-symlink file"
  local bytes digest etag version
  bytes=$(size "$path")
  digest=$(sha "$path")
  jq -e --arg name "$name" --argjson bytes "$bytes" '
    .objects[$name].ContentLength == $bytes and
    (.objects[$name].ETag | type == "string" and length > 1) and
    (.objects[$name].VersionId | type == "string" and length > 0)
  ' "$evidence/object-heads.json" >/dev/null || die "$name S3 head/version does not match local bytes"
  etag=$(jq -er --arg name "$name" '.objects[$name].ETag' "$evidence/object-heads.json")
  version=$(jq -er --arg name "$name" '.objects[$name].VersionId' "$evidence/object-heads.json")
  jq -e --arg key "$key" --arg etag "$etag" --argjson bytes "$bytes" '
    ([.objects[] | select(.Key == $key and .Size == $bytes and .ETag == $etag)] | length) == 1
  ' "$evidence/results.json" >/dev/null || die "$name missing or mismatched in the complete results listing"
  jq -cn --arg path "$name" --arg sha "$digest" --argjson bytes "$bytes" \
    --arg uri "${results_root}${pod_uid}/$name" --arg etag "$etag" --arg version "$version" \
    '{path:$path,sha256:$sha,bytes:$bytes,s3_uri:$uri,etag:$etag,version_id:$version,readback_verified:true}'
}
optional_remote_descriptor() {
  local name=$1
  if [[ -f "$evidence/$name" && ! -L "$evidence/$name" ]]; then
    remote_descriptor "$name"
  elif [[ -e "$evidence/$name" ]]; then
    die "$name is not a regular, non-symlink file"
  else
    printf 'null\n'
  fi
}

payload_remote=$(optional_remote_descriptor records.jsonl)
start_remote=$(optional_remote_descriptor worker-start.json)
exit_remote=$(optional_remote_descriptor worker-exit.json)
upload_remote=$(optional_remote_descriptor worker-upload.json)
runtime_remote=$(optional_remote_descriptor runtime-provenance.json)
build_remote=$(optional_remote_descriptor build-info.json)
sums_remote=$(optional_remote_descriptor SHA256SUMS)

# Every recovered byte is independently admitted even when interruption left a
# strict prefix of the normal worker set. Only a complete coherent set can
# establish a scientific terminal class; a proper subset remains infrastructure
# failure while preserving its hashes and S3 versions in the receipt.
worker_complete=false
if [[ "$start_remote" != null && "$exit_remote" != null && "$upload_remote" != null && \
      "$runtime_remote" != null && "$build_remote" != null && "$sums_remote" != null ]]; then
  worker_complete=true
fi
for name in worker-start.json worker-exit.json worker-upload.json runtime-provenance.json build-info.json; do
  if [[ -f "$evidence/$name" ]]; then
    jq -e . "$evidence/$name" >/dev/null || die "invalid JSON in $name"
  fi
done

terminal_class=infrastructure_failure
deadline_reached=false
exit_code=null
payload_sha=null
payload_bytes=0
payload_etag=null
payload_version=null
payload_uploaded=false
bulk_uploaded=false
payload_readback=false
final_newline=false
started_at=null
finished_at=null
resource_provenance=null
empty_sha=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

if [[ "$payload_remote" != null ]]; then
  payload_sha=$(jq -r '.sha256' <<<"$payload_remote")
  payload_bytes=$(jq -r '.bytes' <<<"$payload_remote")
  payload_etag=$(jq -c '.etag' <<<"$payload_remote")
  payload_version=$(jq -c '.version_id' <<<"$payload_remote")
  payload_readback=true
  if [[ $payload_bytes -gt 0 && $(tail -c 1 "$evidence/records.jsonl" | od -An -tuC | tr -d '[:space:]') == 10 ]]; then
    final_newline=true
  fi
fi

if [[ "$start_remote" != null ]]; then
  jq -e --argjson q "$q" --arg manifest "$manifest_sha" --arg commit "$instrument_commit" \
    --arg protocol "$protocol_sha" --arg binary "$binary_sha" --arg image "$image_digest" \
    --arg worker "$worker_sha" --arg job "$job_name" --arg pod "$pod_uid" --arg node "$node_name" '
    .schema == "lead15.worker-start/v1" and .q == $q and
    .source_manifest_sha256 == $manifest and .instrument_commit == $commit and
    .protocol_sha256 == $protocol and .binary_sha256 == $binary and
    .image_digest == $image and .worker_sha256 == $worker and
    .scientific_deadline_seconds == 3600 and
    .scientific_environment == {RAYON_NUM_THREADS:"1",OMP_NUM_THREADS:"1"} and
    .job_name == $job and .pod_uid == $pod and .node_name == $node
  ' "$evidence/worker-start.json" >/dev/null || die "worker-start identity mismatch"
fi
if [[ "$build_remote" != null ]]; then
  [[ "$(sha "$evidence/build-info.json")" == "$build_info_sha" ]] || die "build-info byte hash mismatch"
  jq -e --arg commit "$instrument_commit" --arg binary "$binary_sha" --arg worker "$worker_sha" '
    .schema == "lead15.build-info/v1" and .source_revision == $commit and
    .lead15_binary_sha256 == $binary and .lead15_worker_sha256 == $worker and
    .binary_path == "/opt/lead15/lead15_diem_quartic" and
    .worker_path == "/opt/lead15/lead15-worker" and
    .build_command == ["cargo","build","--locked","--release","--example","lead15_diem_quartic"]
  ' "$evidence/build-info.json" >/dev/null || die "build-info semantic mismatch"
fi
if [[ "$exit_remote" != null ]]; then
  jq -e --arg job "$job_name" --arg pod "$pod_uid" --arg node "$node_name" --arg results "${results_root}${pod_uid}/" '
    .schema == "lead15.worker-exit/v1" and .job_name == $job and .pod_uid == $pod and
    .node_name == $node and .results_uri == $results and
    (.terminal_class == "completed" or .terminal_class == "deadline" or
     .terminal_class == "scientific_failure" or .terminal_class == "infrastructure_failure") and
    (.deadline_reached | type == "boolean") and
    (.exit_code | type == "number" and floor == . and . >= 0 and . <= 255)
  ' "$evidence/worker-exit.json" >/dev/null || die "worker-exit identity/class mismatch"
  worker_terminal=$(jq -er '.terminal_class' "$evidence/worker-exit.json")
  deadline_reached=$(jq -r '.deadline_reached' "$evidence/worker-exit.json")
  exit_code=$(jq -r '.exit_code' "$evidence/worker-exit.json")
  started_at=$(jq -c '.started_at' "$evidence/worker-exit.json")
  finished_at=$(jq -c '.finished_at' "$evidence/worker-exit.json")
  if [[ "$payload_remote" != null ]]; then
    jq -e --arg sha "$payload_sha" --argjson bytes "$payload_bytes" --argjson newline "$final_newline" '
      .payload.sha256 == $sha and .payload.bytes == $bytes and .payload.final_newline == $newline
    ' "$evidence/worker-exit.json" >/dev/null || die "worker-exit payload metadata mismatch"
  else
    jq -e --arg empty "$empty_sha" \
      '.payload.bytes == 0 and .payload.sha256 == $empty and .payload.final_newline == false' \
      "$evidence/worker-exit.json" >/dev/null \
      || die "missing local payload despite nonempty worker payload claim"
  fi
fi
if [[ "$runtime_remote" != null ]]; then
  jq -e --arg job "$job_name" --arg pod "$pod_uid" --arg node "$node_name" '
    .schema == "lead15.runtime-provenance/v1" and .job_name == $job and
    .pod_uid == $pod and .node_name == $node and
    (.cpu_model | type == "string" and length > 0) and .logical_cpus > 0 and
    .memory_bytes > 0 and (.kernel | type == "string" and length > 0)
  ' "$evidence/runtime-provenance.json" >/dev/null || die "runtime provenance mismatch"
  resource_provenance=$(jq -c '{cpu_model,logical_cpus,memory_bytes,kernel,node_name}' "$evidence/runtime-provenance.json")
fi
if [[ "$upload_remote" != null ]]; then
  jq -e --arg results "${results_root}${pod_uid}/" '
    .schema == "lead15.worker-upload/v1" and .results_uri == $results and
    (.uploaded | type == "boolean") and (.upload_exit | type == "number")
  ' "$evidence/worker-upload.json" >/dev/null || die "worker-upload receipt mismatch"
  bulk_uploaded=$(jq -r '.uploaded' "$evidence/worker-upload.json")
  if [[ "$payload_remote" != null && "$bulk_uploaded" == true ]]; then
    payload_uploaded=true
  fi
  if [[ "$payload_remote" != null ]]; then
    jq -e --arg sha "$payload_sha" --argjson bytes "$payload_bytes" '
      .payload_sha256 == $sha and .payload_bytes == $bytes
    ' "$evidence/worker-upload.json" >/dev/null || die "worker-upload payload metadata mismatch"
  else
    jq -e --arg empty "$empty_sha" '.payload_sha256 == $empty and .payload_bytes == 0' \
      "$evidence/worker-upload.json" >/dev/null || die "worker-upload empty-payload metadata mismatch"
  fi
fi

if [[ "$sums_remote" != null ]]; then
  # Do not execute paths from the worker checksum file. Compare only fixed names.
  for name in worker-start.json worker-exit.json runtime-provenance.json build-info.json; do
    if [[ -f "$evidence/$name" ]]; then
      expected=$(awk -v path="./$name" '$2 == path {print $1}' "$evidence/SHA256SUMS")
      [[ "$expected" =~ ^[0-9a-f]{64}$ && "$expected" == "$(sha "$evidence/$name")" ]] \
        || die "SHA256SUMS mismatch for $name"
    fi
  done
  if [[ "$payload_remote" != null ]]; then
    expected=$(awk '$2 == "./records.jsonl" {print $1}' "$evidence/SHA256SUMS")
    [[ "$expected" == "$payload_sha" ]] || die "SHA256SUMS mismatch for records.jsonl"
  fi
fi

if [[ $worker_complete == true ]]; then
  if [[ "$worker_terminal" == completed && "$job_state" == succeeded && "$exit_code" == 0 && \
        "$bulk_uploaded" == true && "$payload_readback" == true && "$final_newline" == true ]]; then
    terminal_class=completed
  elif [[ "$worker_terminal" == deadline && "$job_state" == failed && \
          "$deadline_reached" == true && "$bulk_uploaded" == true ]]; then
    terminal_class=deadline
  elif [[ "$worker_terminal" == scientific_failure && "$job_state" == failed && "$deadline_reached" == false && \
          "$exit_code" != 0 && "$bulk_uploaded" == true ]]; then
    terminal_class=scientific_failure
  else
    terminal_class=infrastructure_failure
  fi
elif [[ "$exit_remote" == null ]]; then
  # Use the raw Pod's terminated status only as infrastructure provenance. It
  # can never manufacture a scientific terminal classification.
  if [[ $attempts == 1 ]]; then
    exit_code=$(jq -c '.status.containerStatuses[0].state.terminated.exitCode // null' "$evidence/pod.json")
  fi
fi

payload_present=false
payload_uri=null
if [[ "$payload_remote" != null ]]; then
  payload_present=true
  payload_uri=$(jq -c '.s3_uri' <<<"$payload_remote")
fi
payload=$(jq -cn --argjson present "$payload_present" --argjson uri "$payload_uri" \
  --argjson sha "$([[ "$payload_sha" == null ]] && echo null || jq -Rn --arg v "$payload_sha" '$v')" \
  --argjson bytes "$payload_bytes" --argjson etag "$payload_etag" --argjson version "$payload_version" \
  --argjson uploaded "$payload_uploaded" --argjson readback "$payload_readback" --argjson newline "$final_newline" \
  '{present:$present,s3_uri:$uri,sha256:$sha,bytes:$bytes,etag:$etag,version_id:$version,
    uploaded:$uploaded,readback_verified:$readback,final_newline:$newline}')

lineage=$(jq -cn \
  --argjson request "$(local_descriptor request.json)" \
  --argjson request_head "$(local_descriptor request-head.json)" \
  --argjson submission "$(local_descriptor submission.json)" \
  --argjson job "$(local_descriptor job.json)" \
  --argjson pod "$(optional_local_descriptor pod.json)" \
  --argjson status "$(local_descriptor status.json)" \
  --argjson results "$(local_descriptor results.json)" \
  --argjson heads "$(local_descriptor object-heads.json)" \
  --argjson payload "$(optional_local_descriptor records.jsonl)" \
  --argjson start "$(optional_local_descriptor worker-start.json)" \
  --argjson exit "$(optional_local_descriptor worker-exit.json)" \
  --argjson upload "$(optional_local_descriptor worker-upload.json)" \
  --argjson runtime "$(optional_local_descriptor runtime-provenance.json)" \
  --argjson build "$(optional_local_descriptor build-info.json)" \
  --argjson sums "$(optional_local_descriptor SHA256SUMS)" \
  '{request:$request,request_head:$request_head,submission:$submission,job:$job,pod:$pod,
    status:$status,results:$results,object_heads:$heads,payload:$payload,worker_start:$start,
    worker_exit:$exit,worker_upload:$upload,runtime_provenance:$runtime,build_info:$build,
    sha256sums:$sums}')

collected_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)
receipt=$(jq -cn \
  --argjson q "$q" --arg manifest "$manifest_sha" --arg collector "$collector_sha" \
  --arg request_sha "$request_sha" --argjson request_artifact "$request_artifact" \
  --arg image_uri "$image_uri" --arg image_digest "$image_digest" --argjson argv "$expected_argv" \
  --arg job_name "$job_name" --arg job_uid "$job_uid" --argjson pod "$pod_json" \
  --argjson attempts "$attempts" --argjson restarts "$pod_restarts" --arg terminal "$terminal_class" \
  --argjson deadline "$deadline_reached" --argjson exit "$exit_code" --argjson payload "$payload" \
  --argjson resource "$resource_provenance" --argjson lineage "$lineage" \
  --argjson started "$started_at" --argjson finished "$finished_at" --arg collected "$collected_at" '
  {schema:"lead15.completion-receipt/v1",protocol_id:"LEAD15-DT-E5-20261101",q:$q,
   source_manifest_sha256:$manifest,collector_sha256:$collector,request_sha256:$request_sha,
   request_artifact:$request_artifact,image_uri:$image_uri,image_digest:$image_digest,argv:$argv,
   requested_resources:{cpu:4,memory_gib:16,spot:true},
   scientific_environment:{RAYON_NUM_THREADS:"1",OMP_NUM_THREADS:"1"},
   scientific_deadline_seconds:3600,outer_job_deadline_seconds:7200,
   termination_grace_seconds:60,retries:0,attempts:$attempts,pod_restarts:$restarts,
   job:{name:$job_name,uid:$job_uid},pod:$pod,terminal_class:$terminal,
   deadline_reached:$deadline,exit_code:$exit,payload:$payload,resource_provenance:$resource,
   lineage_artifacts:$lineage,started_at:$started,finished_at:$finished,collected_at:$collected}')

# noclobber closes a race between the initial existence check and final write.
( set -o noclobber; printf '%s\n' "$receipt" > "$output" ) || die "refusing to overwrite $output"
echo "$output"
