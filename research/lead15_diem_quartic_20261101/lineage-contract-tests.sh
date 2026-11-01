#!/usr/bin/env bash
# Schema-level positive and adversarial tests for the LEAD-15 launch/lineage
# contract. These fixtures contain no scientific input or output data.
set -euo pipefail

root=$(cd "$(dirname "$0")" && pwd -P)
validator=${JSONSCHEMA:-jsonschema}
command -v "$validator" >/dev/null || {
  echo "lineage-contract-tests: jsonschema CLI is required" >&2
  exit 2
}
tmp=$(mktemp -d)
trap 'rm -rf -- "$tmp"' EXIT
hex40=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
hex64=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
hex64c=cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
image="example.invalid/lead15@sha256:$hex64c"

jq -n --arg c "$hex40" --arg h "$hex64" --arg worker "$hex64c" '
  {schema:"lead15.build-info/v1",source_revision:$c,cargo_lock_sha256:$h,
   rustc_verbose:"rustc fixture",target_triple:"x86_64-unknown-linux-gnu",
   base_image_uri:("example.invalid/base@sha256:"+$h),image_recipe_sha256:$h,
   build_command:["cargo","build","--locked","--release","--example","lead15_diem_quartic"],
   lead15_binary_sha256:$h,lead15_worker_sha256:$worker,
   binary_path:"/opt/lead15/lead15_diem_quartic",worker_path:"/opt/lead15/lead15-worker"}
' > "$tmp/build-info-valid.json"
"$validator" -i "$tmp/build-info-valid.json" "$root/build-info.schema.json" >/dev/null
jq --arg digest "sha256:$hex64c" '.image_digest=$digest' "$tmp/build-info-valid.json" > "$tmp/build-info-self-digest.json"
if "$validator" -i "$tmp/build-info-self-digest.json" "$root/build-info.schema.json" >/dev/null 2>&1; then
  echo "lineage-contract-tests: self-referential image digest was accepted in build-info" >&2
  exit 1
fi

jq -n --arg h "$hex64" --arg c "$hex40" --arg image "$image" --arg digest "sha256:$hex64c" '
  {schema:"lead15.isolab-request/v1",tool:"isolab_k8s_run",source_manifest_sha256:$h,
   arguments:{job_name:"lead15-dt-e5-q257-aaaaaaaa",
     command:["/opt/lead15/lead15-worker","257",$h,$c,$h,$h,$digest,"3600"],
     image:$image,cpu:4,memory:"16Gi",seconds:7200,retries:0}}
' > "$tmp/request-valid.json"
"$validator" -i "$tmp/request-valid.json" "$root/isolab-request.schema.json" >/dev/null

expect_request_reject() {
  local name=$1 filter=$2
  jq "$filter" "$tmp/request-valid.json" > "$tmp/$name.json"
  if "$validator" -i "$tmp/$name.json" "$root/isolab-request.schema.json" >/dev/null 2>&1; then
    echo "lineage-contract-tests: request mutation $name was accepted" >&2
    exit 1
  fi
}
expect_request_reject outer-deadline-3600 '.arguments.seconds=3600'
expect_request_reject retry-enabled '.arguments.retries=1'
expect_request_reject mutable-image '.arguments.image="example.invalid/lead15:latest"'
expect_request_reject self-hash-argument '.arguments.command += ["bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"]'

local_file=$(jq -cn --arg h "$hex64" '{path:"artifact.json",sha256:$h,bytes:1}')
remote_file=$(jq -cn --arg h "$hex64" \
  '{path:"request.json",sha256:$h,bytes:1,s3_uri:"s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/requests/request.json",etag:"\"etag\"",version_id:"version-1",readback_verified:true}')
lineage=$(jq -cn --argjson f "$local_file" '
  {request:$f,request_head:$f,submission:$f,job:$f,pod:$f,status:$f,results:$f,
   object_heads:$f,payload:$f,worker_start:$f,worker_exit:$f,worker_upload:$f,
   runtime_provenance:$f,build_info:$f,sha256sums:$f}')
jq -n --arg h "$hex64" --arg c "$hex40" --arg image "$image" --arg digest "sha256:$hex64c" \
  --argjson request "$remote_file" --argjson lineage "$lineage" '
  {schema:"lead15.completion-receipt/v1",protocol_id:"LEAD15-DT-E5-20261101",q:257,
   source_manifest_sha256:$h,collector_sha256:$h,request_sha256:$h,request_artifact:$request,
   image_uri:$image,image_digest:$digest,
   argv:["/opt/lead15/lead15-worker","257",$h,$c,$h,$h,$digest,"3600"],
   requested_resources:{cpu:4,memory_gib:16,spot:true},
   scientific_environment:{RAYON_NUM_THREADS:"1",OMP_NUM_THREADS:"1"},
   scientific_deadline_seconds:3600,outer_job_deadline_seconds:7200,
   termination_grace_seconds:60,retries:0,attempts:1,pod_restarts:0,
   job:{name:"lead15-dt-e5-q257-aaaaaaaa",uid:"job-uid"},
   pod:{name:"pod-name",uid:"pod-uid",node_name:"node-name"},terminal_class:"completed",
   deadline_reached:false,exit_code:0,
   payload:{present:true,s3_uri:"s3://isolab-results-104186811093-us-east-2/runs/lead15-dt-e5-q257-aaaaaaaa/pod-uid/records.jsonl",
     sha256:$h,bytes:1,etag:"\"etag\"",version_id:"version-2",uploaded:true,
     readback_verified:true,final_newline:true},
   resource_provenance:{cpu_model:"fixture",logical_cpus:4,memory_bytes:1,kernel:"fixture",node_name:"node-name"},
   lineage_artifacts:$lineage,started_at:"2026-11-01T00:00:00Z",finished_at:"2026-11-01T00:00:01Z",
   collected_at:"2026-11-01T00:00:02Z"}
' > "$tmp/completion-valid.json"
"$validator" -i "$tmp/completion-valid.json" "$root/completion-receipt.schema.json" >/dev/null

expect_completion_reject() {
  local name=$1 filter=$2
  jq "$filter" "$tmp/completion-valid.json" > "$tmp/$name.json"
  if "$validator" -i "$tmp/$name.json" "$root/completion-receipt.schema.json" >/dev/null 2>&1; then
    echo "lineage-contract-tests: completion mutation $name was accepted" >&2
    exit 1
  fi
}
expect_completion_reject copied-metadata-only 'del(.lineage_artifacts.job)'
expect_completion_reject unverified-payload '.payload.readback_verified=false'
expect_completion_reject wrong-results-bucket '.payload.s3_uri="s3://setec-astronomy-artifacts/result.jsonl"'
expect_completion_reject outer-deadline-collapsed '.outer_job_deadline_seconds=3600'
expect_completion_reject restarted-pod '.pod_restarts=1'

"$validator" -i "$root/source-manifest.json" "$root/source-manifest.schema.json" >/dev/null
echo "lineage contract schema tests passed"
