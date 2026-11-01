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
checksum_b64() { openssl dgst -sha256 -binary "$1" | base64 | tr -d '\n'; }

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
  .pre_manifest_package_receipt={scope:"pre_manifest_package_excluding_source_manifest_and_requests",
    s3_uri:"s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/pre-manifest-package.tar",
    sha256:$h,bytes:1,version_id:"freeze-version",readback_verified:true} |
  .frozen_at="2026-11-01T00:00:00Z"
' "$root/source-manifest.json" > "$tmp/source-manifest.json"
jsonschema -i "$tmp/source-manifest.json" "$root/source-manifest.schema.json" >/dev/null
for mutation in job-q-coupling request-uri prereg-uri; do
  case "$mutation" in
    job-q-coupling)
      filter='.jobs[0].job_name="lead15-dt-e5-q2053-aaaaaaaa" |
        .jobs[0].argv_template[1]="2053" |
        .jobs[0].results_prefix="s3://isolab-results-104186811093-us-east-2/runs/lead15-dt-e5-q2053-aaaaaaaa/"'
      ;;
    request-uri) filter='.jobs[0].request_artifact_s3_uri="s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/requests/q-2053.json"' ;;
    prereg-uri) filter='.pre_manifest_package_receipt.s3_uri="s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/other.tar"' ;;
  esac
  jq "$filter" "$tmp/source-manifest.json" > "$tmp/source-manifest-$mutation.json"
  if jsonschema -i "$tmp/source-manifest-$mutation.json" "$root/source-manifest.schema.json" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: source manifest mutation $mutation was schema-valid" >&2
    exit 1
  fi
done
manifest_sha=$(sha256sum "$tmp/source-manifest.json" | awk '{print $1}')
argv=$(jq -c --arg manifest "$manifest_sha" \
  '.jobs[] | select(.q==257) | .argv_template | map(if .=="${SOURCE_MANIFEST_SHA256}" then $manifest else . end)' \
  "$tmp/source-manifest.json")

jq -n --arg manifest "$manifest_sha" --arg job "$job" --arg image "$image" --argjson argv "$argv" '
  {schema:"lead15.isolab-request/v1",tool:"isolab_k8s_run",source_manifest_sha256:$manifest,
   arguments:{job_name:$job,command:$argv,image:$image,cpu:4,memory:"16Gi",seconds:7200,retries:0}}
' > "$evidence/request.json"
request_bytes=$(stat -c %s "$evidence/request.json")
request_checksum=$(checksum_b64 "$evidence/request.json")
jq -n --argjson bytes "$request_bytes" --arg checksum "$request_checksum" \
  '{ContentLength:$bytes,ETag:"\"request-etag\"",VersionId:"request-version",ChecksumSHA256:$checksum}' \
  > "$evidence/request-head.json"
jq -n --arg job "$job" --arg uid "$job_uid" --arg results "$results" \
  '{job:{job_name:$job,uid:$uid,state:"pending"},results_uri:$results}' > "$evidence/submission.json"

jq -n --arg job "$job" --arg uid "$job_uid" --arg image "$image" --arg results "${results}\$(POD_UID)/" \
  --argjson argv "$argv" '
  {apiVersion:"batch/v1",kind:"Job",metadata:{name:$job,uid:$uid,namespace:"isolab",generation:1,
    labels:{"isolab.dev/job":"true"}},
   spec:{backoffLimit:0,activeDeadlineSeconds:7200,ttlSecondsAfterFinished:86400,
    selector:{matchLabels:{"batch.kubernetes.io/controller-uid":$uid}},
    template:{metadata:{creationTimestamp:null,labels:{"isolab.dev/job":"true",
       "batch.kubernetes.io/controller-uid":$uid,"batch.kubernetes.io/job-name":$job,
       "controller-uid":$uid,"job-name":$job},
      annotations:{"karpenter.sh/do-not-disrupt":"true"}},
     spec:{restartPolicy:"Never",
     serviceAccountName:"runner",automountServiceAccountToken:false,terminationGracePeriodSeconds:60,
     nodeSelector:{"isolab.dev/pool":"cpu-spot"},
     tolerations:[{key:"isolab.dev/workload",value:"cpu",operator:"Equal",effect:"NoSchedule"}],
     affinity:{podAntiAffinity:{requiredDuringSchedulingIgnoredDuringExecution:[{
       labelSelector:{matchLabels:{"isolab.dev/job":"true"}},topologyKey:"kubernetes.io/hostname"}]}},
     securityContext:{runAsNonRoot:true,runAsUser:1000,runAsGroup:1000,fsGroup:1000,
       seccompProfile:{type:"RuntimeDefault"}},
     containers:[{name:"task",image:$image,command:$argv,workingDir:"/work",
       env:[{name:"HOME",value:"/work"},{name:"AWS_REGION",value:"us-east-2"},
            {name:"AWS_DEFAULT_REGION",value:"us-east-2"},
            {name:"POD_UID",valueFrom:{fieldRef:{fieldPath:"metadata.uid"}}},
            {name:"NODE_NAME",valueFrom:{fieldRef:{fieldPath:"spec.nodeName"}}},
            {name:"ISOLAB_JOB_ID",value:$job},{name:"ISOLAB_OUTPUT_DIR",value:"/work/results"},
            {name:"ISOLAB_RESULTS_URI",value:$results},{name:"RAYON_NUM_THREADS",value:"4"},
            {name:"OMP_NUM_THREADS",value:"4"}],
       resources:{requests:{cpu:"4",memory:"16Gi","ephemeral-storage":"10Gi"},
                  limits:{cpu:"4",memory:"16Gi","ephemeral-storage":"10Gi"}},
       securityContext:{allowPrivilegeEscalation:false,readOnlyRootFilesystem:true,
         capabilities:{drop:["ALL"]}},
       volumeMounts:[{name:"work",mountPath:"/work"},{name:"tmp",mountPath:"/tmp"}]}],
     volumes:[{name:"work",emptyDir:{sizeLimit:"8Gi"}},
              {name:"tmp",emptyDir:{sizeLimit:"1Gi"}}]}}}}
' > "$evidence/job.json"
jq -n --slurpfile desired "$evidence/job.json" --arg job_uid "$job_uid" --arg pod "$pod" --arg uid "$pod_uid" --arg node "$node" \
  --arg job "$job" --arg image "$image" --arg digest "$digest" '
  {apiVersion:"v1",kind:"Pod",metadata:{name:$pod,generateName:($job+"-"),namespace:"isolab",uid:$uid,
    generation:1,
    labels:($desired[0].spec.template.metadata.labels + {
      "topology.kubernetes.io/region":"us-east-2","topology.kubernetes.io/zone":"us-east-2a"}),
    annotations:$desired[0].spec.template.metadata.annotations,
    ownerReferences:[{apiVersion:"batch/v1",kind:"Job",name:$job,uid:$job_uid,
      controller:true,blockOwnerDeletion:true}]},
   spec:($desired[0].spec.template.spec |
     .nodeName=$node | .serviceAccount="runner" | .dnsPolicy="ClusterFirst" |
     .schedulerName="default-scheduler" | .enableServiceLinks=true |
     .preemptionPolicy="PreemptLowerPriority" | .priority=0 |
     .tolerations += [
       {effect:"NoExecute",key:"node.kubernetes.io/not-ready",operator:"Exists",tolerationSeconds:300},
       {effect:"NoExecute",key:"node.kubernetes.io/unreachable",operator:"Exists",tolerationSeconds:300}] |
     .containers[0].imagePullPolicy="IfNotPresent" |
     .containers[0].terminationMessagePath="/dev/termination-log" |
     .containers[0].terminationMessagePolicy="File" |
     .containers[0].env |= map(if .valueFrom.fieldRef? then
       .valueFrom.fieldRef.apiVersion="v1" else . end) |
     .containers[0].env += [
       {name:"AWS_ROLE_ARN",value:"arn:aws:iam::104186811093:role/isolab-job"},
       {name:"AWS_WEB_IDENTITY_TOKEN_FILE",
        value:"/var/run/secrets/eks.amazonaws.com/serviceaccount/token"}] |
     .containers[0].volumeMounts += [{name:"aws-iam-token",
       mountPath:"/var/run/secrets/eks.amazonaws.com/serviceaccount",readOnly:true,
       recursiveReadOnly:"Disabled"}] |
     .volumes += [{name:"aws-iam-token",projected:{defaultMode:420,
       sources:[{serviceAccountToken:{audience:"sts.amazonaws.com",
         expirationSeconds:86400,path:"token"}}]}}]),
   status:{observedGeneration:1,containerStatuses:[{name:"task",image:$image,restartCount:0,
     imageID:("example.invalid/lead15@"+$digest),
     state:{terminated:{exitCode:0,reason:"Completed"}}}]}}
' > "$evidence/pod.json"
jq -n --arg job "$job" --arg uid "$job_uid" --arg pod "$pod" --arg pod_uid "$pod_uid" --arg node "$node" '
  {job:{job_name:$job,uid:$uid,state:"succeeded"},pods:[{name:$pod,uid:$pod_uid,node:$node,status:{}}]}
' > "$evidence/status.json"

printf '{}\n' > "$evidence/records.jsonl"
: > "$evidence/timeout-events.log"
: > "$evidence/stdout.log"
: > "$evidence/stderr.log"
printf '%s  %s\n' "$h64" /opt/lead15/lead15_diem_quartic > "$evidence/binary.sha256"
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
  {schema:"lead15.worker-exit/v1",terminal_class:"completed",deadline_reached:false,
   timeout_fired:false,exit_code:0,
   child_elapsed_seconds:1,payload:{path:"records.jsonl",sha256:$sha,bytes:$bytes,final_newline:true},
   started_at:"2026-11-01T00:00:00Z",finished_at:"2026-11-01T00:00:01Z",results_uri:$results,
   job_name:$job,pod_uid:$pod,node_name:$node}
' > "$evidence/worker-exit.json"
(
  cd "$evidence"
  sha256sum ./records.jsonl ./worker-start.json ./worker-exit.json ./runtime-provenance.json \
    ./build-info.json ./binary.sha256 ./stdout.log ./stderr.log ./timeout-events.log
) > "$evidence/SHA256SUMS"
jq -n --arg sha "$payload_sha" --argjson bytes "$payload_bytes" --arg results "$pod_results" '
  {schema:"lead15.worker-upload/v1",uploaded:true,upload_exit:0,terminal_class:"completed",
   child_exit_code:0,results_uri:$results,payload_sha256:$sha,payload_bytes:$bytes}
' > "$evidence/worker-upload.json"

objects='{}'
listing='[]'
for name in records.jsonl worker-start.json worker-exit.json runtime-provenance.json build-info.json binary.sha256 stdout.log stderr.log timeout-events.log SHA256SUMS worker-upload.json; do
  bytes=$(stat -c %s "$evidence/$name")
  checksum=$(checksum_b64 "$evidence/$name")
  etag="\"etag-$name\""
  objects=$(jq -c --arg name "$name" --arg etag "$etag" --arg checksum "$checksum" --argjson bytes "$bytes" \
    '. + {($name):{ContentLength:$bytes,ETag:$etag,VersionId:("version-"+$name),ChecksumSHA256:$checksum}}' <<<"$objects")
  listing=$(jq -c --arg key "runs/$job/$pod_uid/$name" --arg etag "$etag" --argjson bytes "$bytes" \
    '. + [{Key:$key,Size:$bytes,ETag:$etag,ChecksumAlgorithm:["SHA256"]}]' <<<"$listing")
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
expect_aligned_job_pod_reject() {
  local name=$1 job_filter=$2 pod_filter=$3 bad=$tmp/$1
  cp -a "$evidence" "$bad"
  unlink "$bad/completion-receipt.json"
  jq "$job_filter" "$bad/job.json" > "$bad/job.next"
  mv "$bad/job.next" "$bad/job.json"
  jq "$pod_filter" "$bad/pod.json" > "$bad/pod.next"
  mv "$bad/pod.next" "$bad/pod.json"
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$bad" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: aligned Job/Pod mutation $name was accepted" >&2
    exit 1
  fi
}
expect_short_remote_etag_reject() {
  local label=$1 etag=$2 dir=$tmp/$1-remote-etag name=records.jsonl
  local key="runs/$job/$pod_uid/$name"
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  jq --arg name "$name" --arg etag "$etag" '.objects[$name].ETag=$etag' \
    "$dir/object-heads.json" > "$dir/object-heads.next"
  mv "$dir/object-heads.next" "$dir/object-heads.json"
  jq --arg key "$key" --arg etag "$etag" \
    '.objects |= map(if .Key == $key then .ETag=$etag else . end)' \
    "$dir/results.json" > "$dir/results.next"
  mv "$dir/results.next" "$dir/results.json"
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$dir" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: short remote ETag $label was accepted" >&2
    exit 1
  fi
}
noncanonical_pad_variant() {
  local value=$1 alphabet=ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/
  local last=${value:42:1} index=-1 i
  [[ ${#value} == 44 && ${value:43:1} == = ]] || return 1
  for ((i=0; i<64; i++)); do
    if [[ ${alphabet:i:1} == "$last" ]]; then
      index=$i
      break
    fi
  done
  [[ $index -ge 0 && $((index % 4)) == 0 && $index -lt 63 ]] || return 1
  printf '%s%s=\n' "${value:0:42}" "${alphabet:index+1:1}"
}
expect_noncanonical_checksum_reject() {
  local kind=$1 dir=$tmp/noncanonical-checksum-$1 canonical alternate
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  case "$kind" in
    request)
      canonical=$(jq -er '.ChecksumSHA256' "$dir/request-head.json")
      alternate=$(noncanonical_pad_variant "$canonical")
      jq --arg checksum "$alternate" '.ChecksumSHA256=$checksum' \
        "$dir/request-head.json" > "$dir/request-head.next"
      mv "$dir/request-head.next" "$dir/request-head.json"
      ;;
    remote)
      canonical=$(jq -er '.objects["records.jsonl"].ChecksumSHA256' "$dir/object-heads.json")
      alternate=$(noncanonical_pad_variant "$canonical")
      jq --arg checksum "$alternate" '.objects["records.jsonl"].ChecksumSHA256=$checksum' \
        "$dir/object-heads.json" > "$dir/object-heads.next"
      mv "$dir/object-heads.next" "$dir/object-heads.json"
      ;;
  esac
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$dir" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: noncanonical $kind checksum was accepted" >&2
    exit 1
  fi
}
expect_manifest_admission_reject() {
  local name=$1 filter=$2 dir=$tmp/manifest-$1 mutated=$tmp/manifest-$1.json
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  jq "$filter" "$tmp/source-manifest.json" > "$mutated"
  if "$root/lead15-collect.sh" 257 "$mutated" "$dir" \
      >"$tmp/manifest-$name.stdout" 2>"$tmp/manifest-$name.stderr"; then
    echo "collector-adversarial-tests: manifest argv mutation $name was accepted" >&2
    exit 1
  fi
  grep -Fq 'frozen manifest or collector identity mismatch' "$tmp/manifest-$name.stderr" || {
    echo "collector-adversarial-tests: manifest argv mutation $name reached a later guard" >&2
    exit 1
  }
  [[ ! -e "$dir/completion-receipt.json" && ! -L "$dir/completion-receipt.json" ]] || {
    echo "collector-adversarial-tests: manifest argv mutation $name wrote a receipt" >&2
    exit 1
  }
}
expect_infrastructure() {
  local name=$1 filter=$2 file=$3
  local bad=$tmp/$name
  cp -a "$evidence" "$bad"
  rm "$bad/completion-receipt.json"
  jq "$filter" "$bad/$file" > "$bad/mutated.json"
  mv "$bad/mutated.json" "$bad/$file"
  "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$bad" >/dev/null
  jq -e '.terminal_class == "infrastructure_failure"' "$bad/completion-receipt.json" >/dev/null \
    || { echo "collector-adversarial-tests: mutation $name retained a scientific class" >&2; exit 1; }
}
drop_remote() {
  local dir=$1 name=$2 key="runs/$job/$pod_uid/$2"
  [[ -f "$dir/$name" ]] && unlink "$dir/$name"
  jq --arg name "$name" 'del(.objects[$name])' "$dir/object-heads.json" > "$dir/object-heads.next"
  mv "$dir/object-heads.next" "$dir/object-heads.json"
  jq --arg key "$key" '.objects |= map(select(.Key != $key))' "$dir/results.json" > "$dir/results.next"
  mv "$dir/results.next" "$dir/results.json"
}
refresh_remote() {
  local dir=$1 name=$2 key="runs/$job/$pod_uid/$2" bytes checksum
  bytes=$(stat -c %s "$dir/$name")
  checksum=$(checksum_b64 "$dir/$name")
  jq --arg name "$name" --arg checksum "$checksum" --argjson bytes "$bytes" '
    .objects[$name].ContentLength=$bytes | .objects[$name].ChecksumSHA256=$checksum
  ' "$dir/object-heads.json" > "$dir/object-heads.next"
  mv "$dir/object-heads.next" "$dir/object-heads.json"
  jq --arg key "$key" --argjson bytes "$bytes" '
    .objects |= map(if .Key == $key then .Size=$bytes else . end)
  ' "$dir/results.json" > "$dir/results.next"
  mv "$dir/results.next" "$dir/results.json"
}
refresh_worker_sums() {
  local dir=$1 name
  (
    cd "$dir"
    : > SHA256SUMS.next
    for name in records.jsonl worker-start.json worker-exit.json runtime-provenance.json \
      build-info.json binary.sha256 stdout.log stderr.log timeout-events.log; do
      [[ ! -f "$name" ]] || sha256sum "./$name" >> SHA256SUMS.next
    done
    mv SHA256SUMS.next SHA256SUMS
  )
  refresh_remote "$dir" SHA256SUMS
}
expect_authenticated_reject() {
  local name=$1 filter=$2 file=$3 bad=$tmp/$1
  cp -a "$evidence" "$bad"
  unlink "$bad/completion-receipt.json"
  jq "$filter" "$bad/$file" > "$bad/mutated.json"
  mv "$bad/mutated.json" "$bad/$file"
  refresh_remote "$bad" "$file"
  case "$file" in
    records.jsonl|worker-start.json|worker-exit.json|runtime-provenance.json|build-info.json|binary.sha256|stdout.log|stderr.log|timeout-events.log)
      refresh_worker_sums "$bad"
      ;;
  esac
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$bad" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: authenticated mutation $name was accepted" >&2
    exit 1
  fi
}
expect_partial_infrastructure() {
  local name=$1 dir=$tmp/$1
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  case "$name" in
    timeout-without-exit)
      printf "timeout: sending signal TERM to command 'bash'\n" > "$dir/timeout-events.log"
      refresh_remote "$dir" timeout-events.log
      drop_remote "$dir" worker-exit.json
      drop_remote "$dir" SHA256SUMS
      ;;
    deadline-exit-without-timeout)
      jq '.terminal_class="deadline" | .deadline_reached=true | .timeout_fired=true |
        .exit_code=124 | .child_elapsed_seconds=3600' "$dir/worker-exit.json" > "$dir/worker-exit.next"
      mv "$dir/worker-exit.next" "$dir/worker-exit.json"
      jq '.job.state="failed"' "$dir/status.json" > "$dir/status.next"
      mv "$dir/status.next" "$dir/status.json"
      jq '.status.containerStatuses[0].state.terminated.exitCode=124 |
        .status.containerStatuses[0].state.terminated.reason="Error"' "$dir/pod.json" > "$dir/pod.next"
      mv "$dir/pod.next" "$dir/pod.json"
      refresh_remote "$dir" worker-exit.json
      drop_remote "$dir" timeout-events.log
      drop_remote "$dir" worker-upload.json
      drop_remote "$dir" SHA256SUMS
      ;;
    payload-claim-without-records)
      drop_remote "$dir" records.jsonl
      ;;
  esac
  "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$dir" >/dev/null
  jq -e '.terminal_class == "infrastructure_failure"' \
    "$dir/completion-receipt.json" >/dev/null \
    || { echo "collector-adversarial-tests: partial $name was not preserved as infrastructure" >&2; exit 1; }
  if [[ "$name" == timeout-without-exit ]]; then
    jq -e '.deadline_reached == false and .exit_code == 0 and
      .started_at == null and .finished_at == null' \
      "$dir/completion-receipt.json" >/dev/null \
      || { echo "collector-adversarial-tests: incomplete worker summary drifted" >&2; exit 1; }
  fi
  if [[ "$name" == payload-claim-without-records ]]; then
    jq -e '.payload.present == false' "$dir/completion-receipt.json" >/dev/null
  fi
}
expect_timeout_tamper_reject() {
  local dir=$tmp/timeout-claim-mismatch
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  jq '.terminal_class="deadline" | .deadline_reached=true | .timeout_fired=true | .exit_code=124' \
    "$dir/worker-exit.json" > "$dir/worker-exit.next"
  mv "$dir/worker-exit.next" "$dir/worker-exit.json"
  refresh_remote "$dir" worker-exit.json
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$dir" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: mismatched complete timeout claim was accepted" >&2
    exit 1
  fi
}
expect_checksum_manifest_reject() {
  local mode=$1 dir=$tmp/checksum-$1 duplicate_line unicode_separator
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  case "$mode" in
    duplicate)
      duplicate_line=$(awk '$2 == "./worker-start.json" {print; exit}' "$dir/SHA256SUMS")
      printf '%s\n' "$duplicate_line" >> "$dir/SHA256SUMS"
      ;;
    trailing-field)
      awk '{if ($2 == "./worker-start.json") print $0 " extra"; else print}' \
        "$dir/SHA256SUMS" > "$dir/SHA256SUMS.next"
      mv "$dir/SHA256SUMS.next" "$dir/SHA256SUMS"
      ;;
    crlf)
      awk '{if ($2 == "./worker-start.json") printf "%s\r\n", $0; else print}' \
        "$dir/SHA256SUMS" > "$dir/SHA256SUMS.next"
      mv "$dir/SHA256SUMS.next" "$dir/SHA256SUMS"
      ;;
    unicode-separator)
      unicode_separator=$'\u2003'
      LC_ALL=C awk -v separator="$unicode_separator" '
        {if ($2 == "./worker-start.json") print $1 separator $2; else print}
      ' "$dir/SHA256SUMS" > "$dir/SHA256SUMS.next"
      mv "$dir/SHA256SUMS.next" "$dir/SHA256SUMS"
      ;;
    invalid-utf8)
      printf 'unrelated-\377\n' >> "$dir/SHA256SUMS"
      ;;
    unrelated-unicode)
      printf 'é\n' >> "$dir/SHA256SUMS"
      ;;
  esac
  refresh_remote "$dir" SHA256SUMS
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$dir" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: checksum manifest $mode mutation was accepted" >&2
    exit 1
  fi
}
expect_dangling_output_reject() {
  local dir=$tmp/dangling-output target=$tmp/dangling-output-target.json
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  ln -s "$target" "$dir/completion-receipt.json"
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$dir" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: dangling completion output symlink was accepted" >&2
    exit 1
  fi
  [[ -L "$dir/completion-receipt.json" && ! -e "$target" ]] || {
    echo "collector-adversarial-tests: dangling completion output symlink was followed" >&2
    exit 1
  }
}
expect_dangling_optional_artifact_reject() {
  local dir=$tmp/dangling-optional target=$tmp/dangling-worker-exit-target.json
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  drop_remote "$dir" worker-exit.json
  ln -s "$target" "$dir/worker-exit.json"
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$dir" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: dangling optional worker artifact was accepted" >&2
    exit 1
  fi
  [[ -L "$dir/worker-exit.json" && ! -e "$target" ]] || {
    echo "collector-adversarial-tests: dangling optional worker artifact was followed" >&2
    exit 1
  }
}
expect_zero_pod_dangling_snapshot_reject() {
  local dir=$tmp/zero-pod-dangling target=$tmp/zero-pod-target.json name
  cp -a "$evidence" "$dir"
  unlink "$dir/completion-receipt.json"
  for name in records.jsonl worker-start.json worker-exit.json worker-upload.json \
    runtime-provenance.json build-info.json binary.sha256 stdout.log stderr.log \
    SHA256SUMS timeout-events.log; do
    drop_remote "$dir" "$name"
  done
  jq '.job.state="failed" | .pods=[]' "$dir/status.json" > "$dir/status.next"
  mv "$dir/status.next" "$dir/status.json"
  unlink "$dir/pod.json"
  ln -s "$target" "$dir/pod.json"
  if "$root/lead15-collect.sh" 257 "$tmp/source-manifest.json" "$dir" >/dev/null 2>&1; then
    echo "collector-adversarial-tests: zero-Pod dangling pod snapshot was accepted" >&2
    exit 1
  fi
  [[ -L "$dir/pod.json" && ! -e "$target" ]] || {
    echo "collector-adversarial-tests: zero-Pod dangling pod snapshot was followed" >&2
    exit 1
  }
}
expect_reject request-deadline '.arguments.seconds=3600' request.json
expect_reject job-deadline '.spec.activeDeadlineSeconds=3600' job.json
expect_reject job-generation '.metadata.generation=2' job.json
expect_reject job-generation-type '.metadata.generation="1"' job.json
expect_reject job-template-name '.spec.template.metadata.name="mutable"' job.json
expect_reject job-template-namespace '.spec.template.metadata.namespace="isolab"' job.json
expect_reject job-template-creation-time '
  .spec.template.metadata.creationTimestamp="2026-11-01T00:00:00Z"' job.json
expect_reject job-args '.spec.template.spec.containers[0].args=["--replace-command"]' job.json
expect_reject job-args-false '.spec.template.spec.containers[0].args=false' job.json
expect_reject job-volume-overlay '
  .spec.template.spec.volumes=[{name:"overlay",emptyDir:{}}] |
  .spec.template.spec.containers[0].volumeMounts=[{name:"overlay",mountPath:"/opt/lead15"}]' job.json
expect_reject job-init-container '
  .spec.template.spec.initContainers=[.spec.template.spec.containers[0]]' job.json
expect_reject job-ephemeral-container '
  .spec.template.spec.ephemeralContainers=[.spec.template.spec.containers[0]]' job.json
expect_reject job-env-from '
  .spec.template.spec.containers[0].envFrom=[{configMapRef:{name:"mutable"}}]' job.json
expect_reject job-fieldref-version-null '
  .spec.template.spec.containers[0].env[3].valueFrom.fieldRef.apiVersion=null' job.json
expect_reject job-lifecycle '
  .spec.template.spec.containers[0].lifecycle={postStart:{exec:{command:["/bin/false"]}}}' job.json
expect_reject job-working-dir '.spec.template.spec.containers[0].workingDir="/tmp"' job.json
expect_reject job-volume-device '
  .spec.template.spec.containers[0].volumeDevices=[{name:"device",devicePath:"/dev/xvda"}]' job.json
expect_reject job-stdin '.spec.template.spec.containers[0].stdin=true' job.json
expect_reject job-stdin-once '.spec.template.spec.containers[0].stdinOnce=true' job.json
expect_reject job-tty '.spec.template.spec.containers[0].tty=true' job.json
expect_reject job-sidecar '
  .spec.template.spec.containers += [.spec.template.spec.containers[0]]' job.json
expect_reject job-device-resource '
  .spec.template.spec.containers[0].resources.limits["example.com/device"]="1"' job.json
expect_reject job-resource-claim '
  .spec.template.spec.containers[0].resources.claims=[{name:"mutable"}]' job.json
expect_reject job-selector '
  .spec.selector.matchLabels["batch.kubernetes.io/controller-uid"]="other"' job.json
expect_reject job-template-label '
  .spec.template.metadata.labels["admission.example/inject"]="true"' job.json
expect_reject job-template-annotation '
  .spec.template.metadata.annotations["sidecar.istio.io/inject"]="true"' job.json
expect_reject job-affinity '.spec.template.spec.affinity={}' job.json
expect_reject job-container-security '
  .spec.template.spec.containers[0].securityContext.readOnlyRootFilesystem=false' job.json
expect_reject job-pod-security '.spec.template.spec.securityContext.runAsUser=0' job.json
expect_reject job-toleration '
  .spec.template.spec.tolerations += [{key:"mutable",operator:"Exists",effect:"NoSchedule"}]' job.json
expect_reject pod-image '.status.containerStatuses[0].imageID="wrong@sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"' pod.json
expect_reject pod-args '.spec.containers[0].args=["--replace-command"]' pod.json
expect_reject pod-args-false '.spec.containers[0].args=false' pod.json
expect_reject pod-volume-overlay '
  .spec.volumes=[{name:"overlay",emptyDir:{}}] |
  .spec.containers[0].volumeMounts=[{name:"overlay",mountPath:"/opt/lead15"}]' pod.json
expect_reject pod-volumes-false '.spec.volumes=false' pod.json
expect_reject pod-init-container '.spec.initContainers=[.spec.containers[0]]' pod.json
expect_reject pod-ephemeral-container '.spec.ephemeralContainers=[.spec.containers[0]]' pod.json
expect_reject pod-env-from '.spec.containers[0].envFrom=[{secretRef:{name:"mutable"}}]' pod.json
expect_reject pod-fieldref-version '
  .spec.containers[0].env[3].valueFrom.fieldRef.apiVersion="v2"' pod.json
expect_reject pod-fieldref-version-null '
  .spec.containers[0].env[3].valueFrom.fieldRef.apiVersion=null' pod.json
expect_reject pod-irsa-role '
  (.spec.containers[0].env[] | select(.name=="AWS_ROLE_ARN").value)="arn:aws:iam::104186811093:role/other"' pod.json
expect_reject pod-irsa-token-path '
  (.spec.containers[0].env[] | select(.name=="AWS_WEB_IDENTITY_TOKEN_FILE").value)="/tmp/token"' pod.json
expect_reject pod-irsa-missing '
  .spec.containers[0].env |= map(select(.name != "AWS_ROLE_ARN")) |
  .spec.containers[0].volumeMounts |= map(select(.name != "aws-iam-token")) |
  .spec.volumes |= map(select(.name != "aws-iam-token"))' pod.json
expect_reject pod-irsa-mount '
  (.spec.containers[0].volumeMounts[] | select(.name=="aws-iam-token").mountPath)="/opt/lead15"' pod.json
expect_reject pod-irsa-volume '
  (.spec.volumes[] | select(.name=="aws-iam-token").projected.sources[0].serviceAccountToken.audience)="other"' pod.json
expect_reject pod-irsa-duplicate-env '
  .spec.containers[0].env += [(.spec.containers[0].env[] | select(.name=="AWS_ROLE_ARN"))]' pod.json
expect_reject pod-irsa-duplicate-mount '
  .spec.containers[0].volumeMounts += [(.spec.containers[0].volumeMounts[] |
    select(.name=="aws-iam-token"))]' pod.json
expect_reject pod-irsa-duplicate-volume '
  .spec.volumes += [(.spec.volumes[] | select(.name=="aws-iam-token"))]' pod.json
expect_reject pod-extra-env '
  .spec.containers[0].env += [{name:"BASH_ENV",value:"/work/injected"}]' pod.json
expect_reject pod-extra-mount '
  .spec.containers[0].volumeMounts += [{name:"work",mountPath:"/opt/lead15"}]' pod.json
expect_reject pod-extra-volume '
  .spec.volumes += [{name:"mutable",emptyDir:{}}]' pod.json
expect_reject pod-lifecycle '
  .spec.containers[0].lifecycle={postStart:{exec:{command:["/bin/false"]}}}' pod.json
expect_reject pod-working-dir '.spec.containers[0].workingDir="/tmp"' pod.json
expect_reject pod-volume-device '
  .spec.containers[0].volumeDevices=[{name:"device",devicePath:"/dev/xvda"}]' pod.json
expect_reject pod-stdin '.spec.containers[0].stdin=true' pod.json
expect_reject pod-stdin-once '.spec.containers[0].stdinOnce=true' pod.json
expect_reject pod-tty '.spec.containers[0].tty=true' pod.json
expect_reject pod-sidecar '.spec.containers += [.spec.containers[0]]' pod.json
expect_reject pod-device-resource '.spec.containers[0].resources.limits["example.com/device"]="1"' pod.json
expect_reject pod-resource-claim '.spec.containers[0].resources.claims=[{name:"mutable"}]' pod.json
expect_reject pod-type-meta '.apiVersion=false' pod.json
expect_reject pod-generation '.metadata.generation=2' pod.json
expect_reject pod-generation-type '.metadata.generation="1"' pod.json
expect_reject pod-observed-generation '.status.observedGeneration=2' pod.json
expect_reject pod-observed-generation-type '.status.observedGeneration="1"' pod.json
expect_reject pod-observed-generation-missing 'del(.status.observedGeneration)' pod.json
expect_reject pod-node-name-false '.spec.nodeName=false' pod.json
expect_reject pod-node-name-number '.spec.nodeName=1' pod.json
expect_reject pod-node-name-object '.spec.nodeName={}' pod.json
expect_reject pod-node-name-empty '.spec.nodeName=""' pod.json
expect_reject pod-owner '.metadata.ownerReferences[0].name="other"' pod.json
expect_reject pod-label '.metadata.labels["admission.example/inject"]="true"' pod.json
expect_reject pod-topology-zone '.metadata.labels["topology.kubernetes.io/zone"]="us-west-2a"' pod.json
expect_reject pod-annotation '.metadata.annotations["sidecar.istio.io/inject"]="true"' pod.json
expect_reject pod-affinity '.spec.affinity={}' pod.json
expect_reject pod-container-security '.spec.containers[0].securityContext.readOnlyRootFilesystem=false' pod.json
expect_reject pod-pod-security '.spec.securityContext.runAsUser=0' pod.json
expect_reject pod-toleration '.spec.tolerations += [{key:"mutable",operator:"Exists",effect:"NoSchedule"}]' pod.json
expect_reject pod-status-name '.status.containerStatuses[0].name="sidecar"' pod.json
expect_aligned_job_pod_reject aligned-device-resource \
  '.spec.template.spec.containers[0].resources.limits["example.com/device"]="1"' \
  '.spec.containers[0].resources.limits["example.com/device"]="1"'
expect_aligned_job_pod_reject aligned-resource-claim \
  '.spec.template.spec.containers[0].resources.claims=[{name:"mutable"}]' \
  '.spec.containers[0].resources.claims=[{name:"mutable"}]'
expect_reject status-pods-type '.pods={}' status.json
expect_reject status-pod-name-empty '.pods[0].name=""' status.json
expect_reject status-pod-uid-empty '.pods[0].uid=""' status.json
expect_reject status-pod-node-false '.pods[0].node=false' status.json
expect_reject status-pod-node-number '.pods[0].node=1' status.json
expect_reject status-pod-node-object '.pods[0].node={}' status.json
expect_reject status-pod-node-empty '.pods[0].node=""' status.json
expect_reject missing-version '.objects["records.jsonl"].VersionId=null' object-heads.json
expect_reject wrong-object-checksum '.objects["records.jsonl"].ChecksumSHA256="AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="' object-heads.json
expect_reject wrong-request-checksum '.ChecksumSHA256="AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="' request-head.json
expect_reject one-character-request-etag '.ETag="x"' request-head.json
expect_reject unicode-one-character-request-etag '.ETag="é"' request-head.json
expect_short_remote_etag_reject one-character x
expect_short_remote_etag_reject unicode-one-character é
expect_noncanonical_checksum_reject request
expect_noncanonical_checksum_reject remote
expect_reject copied-payload-hash '.payload.sha256="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"' worker-exit.json
expect_manifest_admission_reject argv-0-executable \
  '.jobs[0].argv_template[0]="/opt/lead15/not-worker"'
expect_manifest_admission_reject argv-1-q '.jobs[0].argv_template[1]="2053"'
expect_manifest_admission_reject argv-2-manifest-placeholder \
  '.jobs[0].argv_template[2]="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"'
expect_manifest_admission_reject argv-3-commit \
  '.jobs[0].argv_template[3]="dddddddddddddddddddddddddddddddddddddddd"'
expect_manifest_admission_reject argv-4-protocol \
  '.jobs[0].argv_template[4]="dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"'
expect_manifest_admission_reject argv-5-binary \
  '.jobs[0].argv_template[5]="eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"'
expect_manifest_admission_reject argv-6-image \
  '.jobs[0].argv_template[6]="sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"'
expect_manifest_admission_reject argv-7-deadline '.jobs[0].argv_template[7]="3599"'
expect_authenticated_reject exit-code-coherence '.exit_code=1' worker-exit.json
expect_authenticated_reject worker-start-timestamp '.started_at="not-a-date-time"' worker-start.json
expect_authenticated_reject worker-start-pre-epoch '.started_at="1969-12-31T23:59:59Z"' worker-start.json
expect_authenticated_reject worker-exit-start-timestamp '.started_at="not-a-date-time"' worker-exit.json
expect_authenticated_reject worker-exit-finish-timestamp '.finished_at="2026-02-29T00:00:00Z"' worker-exit.json
expect_authenticated_reject worker-exit-zero-year '.finished_at="0000-01-01T00:00:00Z"' worker-exit.json
expect_checksum_manifest_reject duplicate
expect_checksum_manifest_reject trailing-field
expect_checksum_manifest_reject crlf
expect_checksum_manifest_reject unicode-separator
expect_checksum_manifest_reject invalid-utf8
expect_checksum_manifest_reject unrelated-unicode
expect_authenticated_reject exit-deadline-elapsed-coherence \
  '.terminal_class="deadline" | .deadline_reached=true | .timeout_fired=true |
   .exit_code=124 | .child_elapsed_seconds=3598' worker-exit.json
expect_authenticated_reject exit-unknown-field '.untrusted=true' worker-exit.json
expect_authenticated_reject exit-payload-unknown-field '.payload.untrusted=true' worker-exit.json
expect_authenticated_reject upload-flag-exit-coherence '.uploaded=false' worker-upload.json
expect_authenticated_reject upload-exit-range '.uploaded=false | .upload_exit=256' worker-upload.json
expect_authenticated_reject upload-unknown-field '.untrusted=true' worker-upload.json
expect_authenticated_reject runtime-pod-name '.pod_name="different-pod"' runtime-provenance.json
expect_authenticated_reject runtime-noninteger-cpu '.logical_cpus=4.5' runtime-provenance.json
expect_authenticated_reject runtime-unknown-field '.untrusted=true' runtime-provenance.json
expect_authenticated_reject start-unknown-field '.untrusted=true' worker-start.json
expect_authenticated_reject mixed-start-time \
  '.started_at="2026-11-01T00:00:02Z"' worker-start.json
expect_infrastructure pod-oom-reason '.status.containerStatuses[0].state.terminated.reason="OOMKilled"' pod.json
expect_partial_infrastructure timeout-without-exit
expect_partial_infrastructure deadline-exit-without-timeout
expect_partial_infrastructure payload-claim-without-records
expect_timeout_tamper_reject
expect_dangling_output_reject
expect_dangling_optional_artifact_reject
expect_zero_pod_dangling_snapshot_reject

echo "collector adversarial tests passed"
