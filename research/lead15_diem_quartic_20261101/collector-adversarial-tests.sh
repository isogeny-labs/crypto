#!/usr/bin/env bash
# Offline integration/adversarial tests for lead15-collect.sh. All records are
# synthetic control-plane fixtures; no scientific binary or target computation runs.
set -euo pipefail

root=$(cd "$(dirname "$0")" && pwd -P)
tmp=$(mktemp -d)
trap 'rm -rf -- "$tmp"' EXIT
evidence=$tmp/evidence
mkdir "$evidence"
h40=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
h64=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
h64c=cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc
digest=sha256:$h64c
image=example.invalid/lead15@sha256:$h64c
job=lead15-dt-e5-q257-aaaaaaaa
job_uid=job-uid
pod=pod-name
pod_uid=pod-uid
node=node-name
results=s3://isolab-results-104186811093-us-east-2/runs/$job/
pod_results=${results}${pod_uid}/
collector_sha=$(sha256sum "$root/lead15-collect.sh" | awk '{print $1}')

jq -n --arg commit "$h40" --arg h "$h64" --arg binary "$h64" --arg worker "$h64c" '
  {schema:"lead15.build-info/v1",source_revision:$commit,cargo_lock_sha256:$h,
   rustc_verbose:"rustc fixture",target_triple:"x86_64-unknown-linux-gnu",
   base_image_uri:("example.invalid/base@sha256:"+$h),image_recipe_sha256:$h,
   build_command:["cargo","build","--locked","--release","--example","lead15_diem_quartic"],
   lead15_binary_sha256:$binary,lead15_worker_sha256:$worker,
   binary_path:"/opt/lead15/lead15_diem_quartic",worker_path:"/opt/lead15/lead15-worker"}
' > "$evidence/build-info.json"
build_sha=$(sha256sum "$evidence/build-info.json" | awk '{print $1}')

jq --arg commit "$h40" --arg h "$h64" --arg worker "$h64c" --arg collector "$collector_sha" \
  --arg build "$build_sha" --arg image "$image" --arg digest "$digest" '
  .status="frozen" | .instrument_commit=$commit | .freeze_commit=$commit |
  .protocol_sha256=$h | .protocol_file_sha256=$h | .binary_sha256=$h |
  .build_info_sha256=$build | .build_info_schema_sha256=$h |
  .analysis_source_sha256=$h | .library_source_sha256=$h |
  .worker_source_sha256=$worker | .collector_source_sha256=$collector |
  .k4_reference_sha256=$h | .result_schema_sha256=$h | .source_manifest_schema_sha256=$h |
  .completion_receipt_schema_sha256=$h | .request_schema_sha256=$h |
  .image_uri=$image | .image_digest=$digest |
  .jobs |= map(
    .job_name=("lead15-dt-e5-q" + (.q|tostring) + "-aaaaaaaa") |
    .argv_template=["/opt/lead15/lead15-worker",(.q|tostring),"${SOURCE_MANIFEST_SHA256}",
      $commit,$h,$h,$digest,"3600"] |
    .request_artifact_s3_uri=("s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/requests/q-" + (.q|tostring) + ".json") |
    .results_prefix=("s3://isolab-results-104186811093-us-east-2/runs/" + .job_name + "/")) |
  .durable_output_prefix="s3://isolab-results-104186811093-us-east-2/runs/" |
  .checkpoint_receipt={s3_uri:"s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/freeze.json",
    sha256:$h,bytes:1,version_id:"freeze-version",readback_verified:true} |
  .frozen_at="2026-11-01T00:00:00Z"
' "$root/source-manifest.json" > "$tmp/source-manifest.json"
jsonschema -i "$tmp/source-manifest.json" "$root/source-manifest.schema.json" >/dev/null
manifest_sha=$(sha256sum "$tmp/source-manifest.json" | awk '{print $1}')
argv=$(jq -c --arg manifest "$manifest_sha" \
  '.jobs[] | select(.q==257) | .argv_template | map(if .=="${SOURCE_MANIFEST_SHA256}" then $manifest else . end)' \
  "$tmp/source-manifest.json")

jq -n --arg manifest "$manifest_sha" --arg job "$job" --arg image "$image" --argjson argv "$argv" '
  {schema:"lead15.isolab-request/v1",tool:"isolab_k8s_run",source_manifest_sha256:$manifest,
   arguments:{job_name:$job,command:$argv,image:$image,cpu:4,memory:"16Gi",seconds:7200,retries:0}}
' > "$evidence/request.json"
request_bytes=$(stat -c %s "$evidence/request.json")
jq -n --argjson bytes "$request_bytes" '{ContentLength:$bytes,ETag:"\"request-etag\"",VersionId:"request-version"}' \
  > "$evidence/request-head.json"
jq -n --arg job "$job" --arg uid "$job_uid" --arg results "$results" \
  '{job:{job_name:$job,uid:$uid,state:"pending"},results_uri:$results}' > "$evidence/submission.json"

jq -n --arg job "$job" --arg uid "$job_uid" --arg image "$image" --arg results "${results}\$(POD_UID)/" \
  --argjson argv "$argv" '
  {apiVersion:"batch/v1",kind:"Job",metadata:{name:$job,uid:$uid,namespace:"isolab",labels:{"isolab.dev/job":"true"}},
   spec:{backoffLimit:0,activeDeadlineSeconds:7200,template:{spec:{restartPolicy:"Never",
     serviceAccountName:"runner",automountServiceAccountToken:false,terminationGracePeriodSeconds:60,
     nodeSelector:{"isolab.dev/pool":"cpu-spot"},
     containers:[{name:"task",image:$image,command:$argv,workingDir:"/work",
       env:[{name:"HOME",value:"/work"},{name:"AWS_REGION",value:"us-east-2"},
            {name:"AWS_DEFAULT_REGION",value:"us-east-2"},
            {name:"POD_UID",valueFrom:{fieldRef:{fieldPath:"metadata.uid"}}},
            {name:"NODE_NAME",valueFrom:{fieldRef:{fieldPath:"spec.nodeName"}}},
            {name:"ISOLAB_JOB_ID",value:$job},{name:"ISOLAB_OUTPUT_DIR",value:"/work/results"},
            {name:"ISOLAB_RESULTS_URI",value:$results},{name:"RAYON_NUM_THREADS",value:"4"},
            {name:"OMP_NUM_THREADS",value:"4"}],
       resources:{requests:{cpu:"4",memory:"16Gi","ephemeral-storage":"10Gi"},
                  limits:{cpu:"4",memory:"16Gi","ephemeral-storage":"10Gi"}}}]}}}}
' > "$evidence/job.json"
jq -n --slurpfile desired "$evidence/job.json" --arg job_uid "$job_uid" --arg pod "$pod" --arg uid "$pod_uid" --arg node "$node" \
  --arg image "$image" --arg digest "$digest" --argjson argv "$argv" '
  {metadata:{name:$pod,uid:$uid,ownerReferences:[{uid:$job_uid,controller:true}]},
   spec:{nodeName:$node,containers:[{image:$image,command:$argv,
     env:$desired[0].spec.template.spec.containers[0].env,
     resources:$desired[0].spec.template.spec.containers[0].resources}]},
   status:{containerStatuses:[{restartCount:0,imageID:("example.invalid/lead15@"+$digest),state:{terminated:{exitCode:0}}}]}}
' > "$evidence/pod.json"
jq -n --arg job "$job" --arg uid "$job_uid" --arg pod "$pod" --arg pod_uid "$pod_uid" --arg node "$node" '
  {job:{job_name:$job,uid:$uid,state:"succeeded"},pods:[{name:$pod,uid:$pod_uid,node:$node,status:{}}]}
' > "$evidence/status.json"

printf '{}\n' > "$evidence/records.jsonl"
payload_sha=$(sha256sum "$evidence/records.jsonl" | awk '{print $1}')
payload_bytes=$(stat -c %s "$evidence/records.jsonl")
jq -n --argjson q 257 --arg manifest "$manifest_sha" --arg commit "$h40" --arg protocol "$h64" \
  --arg binary "$h64" --arg image "$digest" --arg worker "$h64c" --arg job "$job" \
  --arg pod "$pod_uid" --arg node "$node" '
  {schema:"lead15.worker-start/v1",q:$q,source_manifest_sha256:$manifest,instrument_commit:$commit,
   protocol_sha256:$protocol,binary_sha256:$binary,image_digest:$image,worker_sha256:$worker,
   scientific_deadline_seconds:3600,job_name:$job,pod_uid:$pod,node_name:$node,
   scientific_environment:{RAYON_NUM_THREADS:"1",OMP_NUM_THREADS:"1"},started_at:"2026-11-01T00:00:00Z"}
' > "$evidence/worker-start.json"
jq -n --arg job "$job" --arg pod "$pod" --arg uid "$pod_uid" --arg node "$node" '
  {schema:"lead15.runtime-provenance/v1",cpu_model:"fixture",logical_cpus:4,memory_bytes:1024,
   kernel:"fixture",job_name:$job,pod_name:$pod,pod_uid:$uid,node_name:$node}
' > "$evidence/runtime-provenance.json"
jq -n --arg sha "$payload_sha" --argjson bytes "$payload_bytes" --arg results "$pod_results" \
  --arg job "$job" --arg pod "$pod_uid" --arg node "$node" '
  {schema:"lead15.worker-exit/v1",terminal_class:"completed",deadline_reached:false,exit_code:0,
   child_elapsed_seconds:1,payload:{path:"records.jsonl",sha256:$sha,bytes:$bytes,final_newline:true},
   started_at:"2026-11-01T00:00:00Z",finished_at:"2026-11-01T00:00:01Z",results_uri:$results,
   job_name:$job,pod_uid:$pod,node_name:$node}
' > "$evidence/worker-exit.json"
(
  cd "$evidence"
  sha256sum ./records.jsonl ./worker-start.json ./worker-exit.json ./runtime-provenance.json ./build-info.json
) > "$evidence/SHA256SUMS"
jq -n --arg sha "$payload_sha" --argjson bytes "$payload_bytes" --arg results "$pod_results" '
  {schema:"lead15.worker-upload/v1",uploaded:true,upload_exit:0,terminal_class:"completed",
   child_exit_code:0,results_uri:$results,payload_sha256:$sha,payload_bytes:$bytes}
' > "$evidence/worker-upload.json"

objects='{}'
listing='[]'
for name in records.jsonl worker-start.json worker-exit.json runtime-provenance.json build-info.json SHA256SUMS worker-upload.json; do
  bytes=$(stat -c %s "$evidence/$name")
  etag="\"etag-$name\""
  objects=$(jq -c --arg name "$name" --arg etag "$etag" --argjson bytes "$bytes" \
    '. + {($name):{ContentLength:$bytes,ETag:$etag,VersionId:("version-"+$name)}}' <<<"$objects")
  listing=$(jq -c --arg key "runs/$job/$pod_uid/$name" --arg etag "$etag" --argjson bytes "$bytes" \
    '. + [{Key:$key,Size:$bytes,ETag:$etag}]' <<<"$listing")
done
jq -n --arg results "$results" --argjson objects "$objects" \
  '{schema:"lead15.object-heads/v1",results_prefix:$results,objects:$objects}' > "$evidence/object-heads.json"
jq -n --arg results "$results" --argjson objects "$listing" \
  '{results_uri:$results,objects:$objects,next_token:null}' > "$evidence/results.json"

"$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$evidence" >/dev/null
jsonschema -i "$evidence/completion-receipt.json" "$root/completion-receipt.schema.json" >/dev/null
jq -e '.terminal_class=="completed" and .payload.readback_verified==true' "$evidence/completion-receipt.json" >/dev/null

expect_reject() {
  local name=$1 filter=$2 file=$3
  local bad=$tmp/$name
  cp -a "$evidence" "$bad"
  rm "$bad/completion-receipt.json"
  jq "$filter" "$bad/$file" > "$bad/mutated.json"
  mv "$bad/mutated.json" "$bad/$file"
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$bad" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: mutation $name was accepted" >&2
    exit 1
  fi
}
expect_reject request-deadline '.arguments.seconds=3600' request.json
expect_reject job-deadline '.spec.activeDeadlineSeconds=3600' job.json
expect_reject pod-image '.status.containerStatuses[0].imageID="wrong@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"' pod.json
expect_reject missing-version '.objects["records.jsonl"].VersionId=null' object-heads.json
expect_reject copied-payload-hash '.payload.sha256="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"' worker-exit.json

echo "collector adversarial tests passed"
