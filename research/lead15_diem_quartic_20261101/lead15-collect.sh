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
for tool in jq sha256sum stat awk base64 od tr; do
  command -v "$tool" >/dev/null || die "required local tool is missing: $tool"
done
q=$1
manifest=$2
evidence=$3
case "$q" in 257|2053|32771|4194301) ;; *) usage ;; esac
[[ -f "$manifest" && ! -L "$manifest" ]] || die "manifest must be a regular, non-symlink file"
[[ -d "$evidence" && ! -L "$evidence" ]] || die "evidence directory must be a real directory"
evidence=$(cd "$evidence" && pwd -P)
output=$evidence/completion-receipt.json
[[ ! -e "$output" && ! -L "$output" ]] || die "$output already exists"

required=(request.json request-head.json submission.json job.json status.json results.json object-heads.json)
for name in "${required[@]}"; do
  [[ -f "$evidence/$name" && ! -L "$evidence/$name" ]] || die "missing regular evidence file $name"
  jq -e . "$evidence/$name" >/dev/null || die "invalid JSON in $name"
done
jq -e . "$manifest" >/dev/null || die "invalid source manifest JSON"

sha() { sha256sum "$1" | awk '{print $1}'; }
size() { stat -c %s "$1"; }
checksum_sha256_hex() {
  local encoded=$1 canonical decoded
  canonical=$(printf '%s' "$encoded" | base64 --decode 2>/dev/null | base64 -w0) \
    || return 1
  [[ "$canonical" == "$encoded" ]] || return 1
  decoded=$(printf '%s' "$encoded" | base64 --decode 2>/dev/null | od -An -v -tx1 | tr -d ' \n') \
    || return 1
  [[ "$decoded" =~ ^[0-9a-f]{64}$ ]] || return 1
  printf '%s\n' "$decoded"
}
checksum_manifest_hash() {
  local manifest=$1 path=$2
  LC_ALL=C awk -v path="$path" '
    $2 == path {
      if (NF != 2 || seen) exit 2
      hash = $1
      seen = 1
    }
    END {
      if (!seen) exit 3
      print hash
    }
  ' "$manifest"
}
file_is_ascii() {
  LC_ALL=C od -An -v -tu1 "$1" | awk '
    { for (i = 1; i <= NF; i++) if ($i > 127) invalid = 1 }
    END { exit invalid }
  '
}
local_descriptor() {
  local name=$1 path=$evidence/$1
  jq -cn --arg path "$name" --arg sha "$(sha "$path")" --argjson bytes "$(size "$path")" \
    '{path:$path,sha256:$sha,bytes:$bytes}'
}
optional_local_descriptor() {
  local name=$1
  if [[ -f "$evidence/$name" && ! -L "$evidence/$name" ]]; then
    local_descriptor "$name"
  elif [[ -e "$evidence/$name" || -L "$evidence/$name" ]]; then
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
  (.instrument_commit | test("^[0-9a-f]{40}$")) and
  (.freeze_commit | test("^[0-9a-f]{40}$")) and
  ([.protocol_sha256,.protocol_file_sha256,.binary_sha256,.build_info_sha256,
    .build_info_schema_sha256,.analysis_source_sha256,.library_source_sha256,
    .worker_source_sha256,.collector_source_sha256,.k4_reference_sha256,
    .result_schema_sha256,.source_manifest_schema_sha256,
    .completion_receipt_schema_sha256,.request_schema_sha256] |
    all(.[]; type == "string" and test("^[0-9a-f]{64}$"))) and
  (.image_uri | test("^[^\\s@]+@sha256:[0-9a-f]{64}$")) and
  (.image_digest | test("^sha256:[0-9a-f]{64}$")) and
  .binary_path_in_image == "/opt/lead15/lead15_diem_quartic" and
  .worker_path_in_image == "/opt/lead15/lead15-worker" and
  .build_info_path_in_image == "/opt/lead15/build-info.json" and
  .launch_tool == "isolab_k8s_run" and .scientific_deadline_seconds == 3600 and
  .outer_job_deadline_seconds == 7200 and .termination_grace_seconds == 60 and
  .retries == 0 and .threads_per_cell == 1 and .isolation == "none" and
  .requested_resources == {cpu:4,memory_gib:16,spot:true} and
  .scientific_environment == {RAYON_NUM_THREADS:"1",OMP_NUM_THREADS:"1"} and
  .collector_source_sha256 == $collector and
  .durable_output_prefix == "s3://isolab-results-104186811093-us-east-2/runs/" and
  (.frozen_at | type == "string" and length > 0) and
  .pre_manifest_package_receipt.scope == "pre_manifest_package_excluding_source_manifest_and_requests" and
  .pre_manifest_package_receipt.s3_uri ==
    "s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/pre-manifest-package.tar" and
  (.pre_manifest_package_receipt.sha256 | test("^[0-9a-f]{64}$")) and
  (.pre_manifest_package_receipt.bytes | type == "number" and floor == . and . > 0) and
  (.pre_manifest_package_receipt.version_id | type == "string" and length > 0) and
  .pre_manifest_package_receipt.readback_verified == true and
  (.jobs | type == "array" and length == 4 and map(.q) == [257,2053,32771,4194301]) and
  (.instrument_commit as $commit |
   .protocol_sha256 as $protocol |
   .binary_sha256 as $binary |
   .image_digest as $image |
   all(.jobs[]; .q as $job_q |
     (.job_name | test("^lead15-dt-e5-q" + ($job_q|tostring) + "-[0-9a-f]{8,12}$")) and
     (.argv_template == ["/opt/lead15/lead15-worker",($job_q|tostring),
       "${SOURCE_MANIFEST_SHA256}",$commit,$protocol,$binary,$image,"3600"]) and
     (.request_artifact_s3_uri ==
       ("s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/requests/q-" + ($job_q|tostring) + ".json")) and
     (.results_prefix ==
       ("s3://isolab-results-104186811093-us-east-2/runs/" + .job_name + "/")))) and
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
request_checksum=$(jq -er '.ChecksumSHA256 | select(type == "string" and length == 44)' "$evidence/request-head.json")
request_checksum_hex=$(checksum_sha256_hex "$request_checksum") \
  || die "request S3 checksum is not a canonical SHA-256 checksum"
[[ "$request_checksum_hex" == "$request_sha" ]] \
  || die "request S3 checksum does not match local request bytes"
jq -e --argjson bytes "$request_bytes" '.ContentLength == $bytes' "$evidence/request-head.json" >/dev/null \
  || die "request S3 head length does not match local bytes"
request_artifact=$(jq -cn --arg path request.json --arg sha "$request_sha" --argjson bytes "$request_bytes" \
  --arg checksum "$request_checksum" --arg uri "$request_uri" --arg etag "$request_etag" \
  --arg version "$request_version" \
  '{path:$path,sha256:$sha,checksum_sha256:$checksum,bytes:$bytes,s3_uri:$uri,
    etag:$etag,version_id:$version,readback_verified:true}')

jq -e --arg job "$job_name" --arg results "$results_root" '
  .job.job_name == $job and (.job.uid | type == "string" and length > 0) and
  .results_uri == $results
' "$evidence/submission.json" >/dev/null || die "submission receipt mismatch"
job_uid=$(jq -er '.job.uid' "$evidence/submission.json")

# Verify the exact raw Kubernetes Job, not a copied summary of selected fields.
jq -e --arg job "$job_name" --arg uid "$job_uid" --arg image "$image_uri" \
  --arg results "${results_root}\$(POD_UID)/" --argjson argv "$expected_argv" '
  def allowed_keys($allowed):
    type == "object" and ((keys - $allowed) | length == 0);
  def empty_array_or_absent($key):
    (has($key) | not) or .[$key] == null or
      ((.[$key] | type) == "array" and (.[$key] | length) == 0);
  def empty_object_or_absent($key):
    (has($key) | not) or .[$key] == null or
      ((.[$key] | type) == "object" and (.[$key] | length) == 0);
  def false_or_absent($key):
    (has($key) | not) or .[$key] == null or .[$key] == false;
  def string_default_or_absent($key;$default):
    (has($key) | not) or .[$key] == null or .[$key] == $default;
  def launcher_resources:
    {requests:{cpu:"4",memory:"16Gi","ephemeral-storage":"10Gi"},
     limits:{cpu:"4",memory:"16Gi","ephemeral-storage":"10Gi"}};
  def launcher_mounts:
    [{name:"work",mountPath:"/work"},{name:"tmp",mountPath:"/tmp"}];
  def launcher_volumes:
    [{name:"work",emptyDir:{sizeLimit:"8Gi"}},{name:"tmp",emptyDir:{sizeLimit:"1Gi"}}];
  def launcher_container_security:
    {allowPrivilegeEscalation:false,readOnlyRootFilesystem:true,capabilities:{drop:["ALL"]}};
  def launcher_pod_security:
    {runAsNonRoot:true,runAsUser:1000,runAsGroup:1000,fsGroup:1000,
     seccompProfile:{type:"RuntimeDefault"}};
  def launcher_affinity:
    {podAntiAffinity:{requiredDuringSchedulingIgnoredDuringExecution:[{
      labelSelector:{matchLabels:{"isolab.dev/job":"true"}},
      topologyKey:"kubernetes.io/hostname"}]}};
  def launcher_toleration:
    {key:"isolab.dev/workload",value:"cpu",operator:"Equal",effect:"NoSchedule"};
  def default_tolerations:
    [{effect:"NoExecute",key:"node.kubernetes.io/not-ready",operator:"Exists",tolerationSeconds:300},
     {effect:"NoExecute",key:"node.kubernetes.io/unreachable",operator:"Exists",tolerationSeconds:300}];
  def sorted_tolerations:
    sort_by([.key // "",.effect // "",.operator // "",.value // "",.tolerationSeconds // -1]);
  def safe_tolerations:
    type == "array" and
      ((sorted_tolerations == ([launcher_toleration] | sorted_tolerations)) or
       (sorted_tolerations == ([launcher_toleration] + default_tolerations | sorted_tolerations)));
  def launcher_labels($job;$uid):
    {"isolab.dev/job":"true","batch.kubernetes.io/controller-uid":$uid,
     "batch.kubernetes.io/job-name":$job,"controller-uid":$uid,"job-name":$job};
  def field_ref_versions_are_v1:
    type == "array" and all(.[];
      (.valueFrom.fieldRef? == null) or
      ((.valueFrom.fieldRef | type) == "object" and
       (((.valueFrom.fieldRef | has("apiVersion")) | not) or
        .valueFrom.fieldRef.apiVersion == "v1")));
  def safe_container:
    allowed_keys(["name","image","command","args","workingDir","env","envFrom","resources",
      "volumeMounts","volumeDevices","lifecycle","stdin","stdinOnce","tty","imagePullPolicy",
      "terminationMessagePath","terminationMessagePolicy","securityContext"]) and
    (.env | field_ref_versions_are_v1) and
    .resources == launcher_resources and .volumeMounts == launcher_mounts and
    .securityContext == launcher_container_security and
    empty_array_or_absent("args") and empty_array_or_absent("envFrom") and
    empty_array_or_absent("volumeDevices") and empty_object_or_absent("lifecycle") and
    false_or_absent("stdin") and false_or_absent("stdinOnce") and false_or_absent("tty") and
    string_default_or_absent("imagePullPolicy";"IfNotPresent") and
    string_default_or_absent("terminationMessagePath";"/dev/termination-log") and
    string_default_or_absent("terminationMessagePolicy";"File");
  def safe_pod_spec($allow_node):
    allowed_keys(["restartPolicy","serviceAccountName","serviceAccount",
      "automountServiceAccountToken","terminationGracePeriodSeconds","nodeSelector","containers",
      "initContainers","ephemeralContainers","volumes","securityContext","dnsPolicy",
      "schedulerName","enableServiceLinks","preemptionPolicy","nodeName","tolerations",
      "priority","priorityClassName","imagePullSecrets","affinity"]) and
    (.containers | type == "array" and length == 1) and all(.containers[]; safe_container) and
    empty_array_or_absent("initContainers") and empty_array_or_absent("ephemeralContainers") and
    .volumes == launcher_volumes and .securityContext == launcher_pod_security and
    .affinity == launcher_affinity and (.tolerations | safe_tolerations) and
    string_default_or_absent("serviceAccount";"runner") and
    string_default_or_absent("dnsPolicy";"ClusterFirst") and
    string_default_or_absent("schedulerName";"default-scheduler") and
    ((has("enableServiceLinks") | not) or .enableServiceLinks == null or .enableServiceLinks == true) and
    string_default_or_absent("preemptionPolicy";"PreemptLowerPriority") and
    ((has("priority") | not) or .priority == null or .priority == 0) and
    string_default_or_absent("priorityClassName";"") and
    empty_array_or_absent("imagePullSecrets") and
    (($allow_node and ((.nodeName? == null) or (.nodeName | type == "string" and length > 0))) or
     (($allow_node | not) and .nodeName? == null));
  allowed_keys(["apiVersion","kind","metadata","spec","status"]) and
  .apiVersion == "batch/v1" and .kind == "Job" and
  .metadata.name == $job and .metadata.uid == $uid and
  (.metadata | allowed_keys(["name","namespace","uid","resourceVersion","generation",
    "creationTimestamp","labels","annotations","managedFields"])) and
  .metadata.namespace == "isolab" and .metadata.generation == 1 and
  .metadata.labels == {"isolab.dev/job":"true"} and
  (.metadata | empty_object_or_absent("annotations")) and
  (.spec | allowed_keys(["backoffLimit","activeDeadlineSeconds","template","parallelism",
    "completions","completionMode","manualSelector","selector","suspend",
    "ttlSecondsAfterFinished","podReplacementPolicy"])) and
  .spec.backoffLimit == 0 and .spec.activeDeadlineSeconds == 7200 and
  (((.spec | has("parallelism")) | not) or .spec.parallelism == null or .spec.parallelism == 1) and
  (((.spec | has("completions")) | not) or .spec.completions == null or .spec.completions == 1) and
  (((.spec | has("completionMode")) | not) or .spec.completionMode == null or
    .spec.completionMode == "NonIndexed") and
  (.spec | false_or_absent("manualSelector")) and (.spec | false_or_absent("suspend")) and
  .spec.selector == {matchLabels:{"batch.kubernetes.io/controller-uid":$uid}} and
  .spec.ttlSecondsAfterFinished == 86400 and
  (.spec | string_default_or_absent("podReplacementPolicy";"TerminatingOrFailed")) and
  (.spec.template | allowed_keys(["metadata","spec"])) and
  (.spec.template.metadata | allowed_keys(["creationTimestamp","labels","annotations",
    "name","namespace"])) and
  (.spec.template.metadata.creationTimestamp? == null) and
  (.spec.template.metadata.name? == null) and
  (.spec.template.metadata.namespace? == null) and
  .spec.template.metadata.annotations == {"karpenter.sh/do-not-disrupt":"true"} and
  .spec.template.metadata.labels == launcher_labels($job;$uid) and
  (.spec.template.spec | safe_pod_spec(false)) and
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
  (.spec.template.spec.containers[0].env | field_ref_versions_are_v1) and
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
  (.pods | type == "array" and length <= 1 and all(.[];
    type == "object" and
    (.name | type == "string" and length > 0) and
    (.uid | type == "string" and length > 0) and
    (((has("node") | not) or .node == null) or
      (.node | type == "string" and length > 0))))
' "$evidence/status.json" >/dev/null || die "status is nonterminal, retried, or belongs to another Job"
job_state=$(jq -er '.job.state' "$evidence/status.json")
attempts=$(jq '.pods | length' "$evidence/status.json")

pod_json=null
pod_uid=''
pod_name=''
node_name=''
pod_restarts=null
pod_exit_code=null
pod_termination_reason=''
if [[ $attempts == 1 ]]; then
  [[ -f "$evidence/pod.json" && ! -L "$evidence/pod.json" ]] || die "raw pod.json is required for a created Pod"
  jq -e . "$evidence/pod.json" >/dev/null || die "invalid pod.json"
  pod_uid=$(jq -er '.pods[0].uid' "$evidence/status.json")
  pod_name=$(jq -er '.pods[0].name' "$evidence/status.json")
  node_name=$(jq -er 'if (.pods[0] | has("node")) and .pods[0].node != null
    then .pods[0].node else "" end' "$evidence/status.json")
  jq -e --slurpfile desired "$evidence/job.json" --arg job "$job_name" --arg job_uid "$job_uid" --arg pod "$pod_name" --arg uid "$pod_uid" \
    --arg node "$node_name" --arg image "$image_uri" --argjson argv "$expected_argv" '
    def allowed_keys($allowed):
      type == "object" and ((keys - $allowed) | length == 0);
    def empty_array_or_absent($key):
      (has($key) | not) or .[$key] == null or
        ((.[$key] | type) == "array" and (.[$key] | length) == 0);
    def empty_object_or_absent($key):
      (has($key) | not) or .[$key] == null or
        ((.[$key] | type) == "object" and (.[$key] | length) == 0);
    def string_default_or_absent($key;$default):
      (has($key) | not) or .[$key] == null or .[$key] == $default;
    def launcher_resources:
      {requests:{cpu:"4",memory:"16Gi","ephemeral-storage":"10Gi"},
       limits:{cpu:"4",memory:"16Gi","ephemeral-storage":"10Gi"}};
    def launcher_mounts:
      [{name:"work",mountPath:"/work"},{name:"tmp",mountPath:"/tmp"}];
    def launcher_volumes:
      [{name:"work",emptyDir:{sizeLimit:"8Gi"}},{name:"tmp",emptyDir:{sizeLimit:"1Gi"}}];
    def launcher_container_security:
      {allowPrivilegeEscalation:false,readOnlyRootFilesystem:true,capabilities:{drop:["ALL"]}};
    def launcher_pod_security:
      {runAsNonRoot:true,runAsUser:1000,runAsGroup:1000,fsGroup:1000,
       seccompProfile:{type:"RuntimeDefault"}};
    def launcher_affinity:
      {podAntiAffinity:{requiredDuringSchedulingIgnoredDuringExecution:[{
        labelSelector:{matchLabels:{"isolab.dev/job":"true"}},
        topologyKey:"kubernetes.io/hostname"}]}};
    def launcher_toleration:
      {key:"isolab.dev/workload",value:"cpu",operator:"Equal",effect:"NoSchedule"};
    def default_tolerations:
      [{effect:"NoExecute",key:"node.kubernetes.io/not-ready",operator:"Exists",tolerationSeconds:300},
       {effect:"NoExecute",key:"node.kubernetes.io/unreachable",operator:"Exists",tolerationSeconds:300}];
    def sorted_tolerations:
      sort_by([.key // "",.effect // "",.operator // "",.value // "",.tolerationSeconds // -1]);
    def safe_tolerations:
      type == "array" and
        ((sorted_tolerations == ([launcher_toleration] | sorted_tolerations)) or
         (sorted_tolerations == ([launcher_toleration] + default_tolerations | sorted_tolerations)));
    def launcher_labels($job;$uid):
      {"isolab.dev/job":"true","batch.kubernetes.io/controller-uid":$uid,
       "batch.kubernetes.io/job-name":$job,"controller-uid":$uid,"job-name":$job};
    def irsa_env:
      [{name:"AWS_ROLE_ARN",value:"arn:aws:iam::104186811093:role/isolab-job"},
       {name:"AWS_WEB_IDENTITY_TOKEN_FILE",
        value:"/var/run/secrets/eks.amazonaws.com/serviceaccount/token"}];
    def safe_irsa_env:
      . as $env |
      (($env | type) == "array") and
      all(irsa_env[]; . as $expected |
        ($env | map(select(. == $expected)) | length) == 1) and
      ([$env[] | select(.name == "AWS_ROLE_ARN" or
        .name == "AWS_WEB_IDENTITY_TOKEN_FILE")] | length) == 2;
    def safe_irsa_mounts:
      . as $mounts |
      (($mounts | type) == "array") and ($mounts | length) == 3 and
      ([$mounts[] | select(.name != "aws-iam-token")] == launcher_mounts) and
      ([$mounts[] | select(.name == "aws-iam-token")] | length) == 1 and
      ([$mounts[] | select(.name == "aws-iam-token")][0] as $token |
        ($token == {name:"aws-iam-token",
          mountPath:"/var/run/secrets/eks.amazonaws.com/serviceaccount",readOnly:true} or
         $token == {name:"aws-iam-token",
          mountPath:"/var/run/secrets/eks.amazonaws.com/serviceaccount",readOnly:true,
          recursiveReadOnly:"Disabled"}));
    def irsa_volume:
      {name:"aws-iam-token",projected:{defaultMode:420,sources:[{serviceAccountToken:{
        audience:"sts.amazonaws.com",expirationSeconds:86400,path:"token"}}]}};
    def safe_irsa_volumes:
      . as $volumes |
      (($volumes | type) == "array") and ($volumes | length) == 3 and
      ([$volumes[] | select(.name != "aws-iam-token")] == launcher_volumes) and
      ([$volumes[] | select(.name == "aws-iam-token")] == [irsa_volume]);
    def field_ref_versions_are_v1:
      type == "array" and all(.[];
        (.valueFrom.fieldRef? == null) or
        ((.valueFrom.fieldRef | type) == "object" and
         (((.valueFrom.fieldRef | has("apiVersion")) | not) or
          .valueFrom.fieldRef.apiVersion == "v1")));
    def safe_pod_labels($job;$uid):
      . as $labels |
      (($labels | type) == "object") and
      (($labels | del(.["topology.kubernetes.io/region"],.["topology.kubernetes.io/zone"]))
        == launcher_labels($job;$uid)) and
      (((($labels | has("topology.kubernetes.io/region")) | not) and
        (($labels | has("topology.kubernetes.io/zone")) | not)) or
       ($labels["topology.kubernetes.io/region"] == "us-east-2" and
        ($labels["topology.kubernetes.io/zone"] | type) == "string" and
        ($labels["topology.kubernetes.io/zone"] | test("^us-east-2[a-z]$"))));
    def safe_container($allow_irsa):
      allowed_keys(["name","image","command","args","workingDir","env","envFrom","resources",
        "volumeMounts","volumeDevices","lifecycle","stdin","stdinOnce","tty","imagePullPolicy",
        "terminationMessagePath","terminationMessagePolicy","securityContext"]) and
      (.env | field_ref_versions_are_v1) and
      (if $allow_irsa then (.env | safe_irsa_env) and (.volumeMounts | safe_irsa_mounts)
       else .volumeMounts == launcher_mounts end) and
      .resources == launcher_resources and
      .securityContext == launcher_container_security and
      empty_array_or_absent("args") and empty_array_or_absent("envFrom") and
      empty_array_or_absent("volumeDevices") and empty_object_or_absent("lifecycle") and
      ((has("stdin") | not) or .stdin == null or .stdin == false) and
      ((has("stdinOnce") | not) or .stdinOnce == null or .stdinOnce == false) and
      ((has("tty") | not) or .tty == null or .tty == false) and
      string_default_or_absent("imagePullPolicy";"IfNotPresent") and
      string_default_or_absent("terminationMessagePath";"/dev/termination-log") and
      string_default_or_absent("terminationMessagePolicy";"File");
    def safe_pod_spec($allow_node;$allow_irsa):
      allowed_keys(["restartPolicy","serviceAccountName","serviceAccount",
        "automountServiceAccountToken","terminationGracePeriodSeconds","nodeSelector","containers",
        "initContainers","ephemeralContainers","volumes","securityContext","dnsPolicy",
        "schedulerName","enableServiceLinks","preemptionPolicy","nodeName","tolerations",
        "priority","priorityClassName","imagePullSecrets","affinity"]) and
      (.containers | type == "array" and length == 1) and
      all(.containers[]; safe_container($allow_irsa)) and
      empty_array_or_absent("initContainers") and empty_array_or_absent("ephemeralContainers") and
      (if $allow_irsa then (.volumes | safe_irsa_volumes)
       else .volumes == launcher_volumes end) and
      .securityContext == launcher_pod_security and
      .affinity == launcher_affinity and (.tolerations | safe_tolerations) and
      string_default_or_absent("serviceAccount";"runner") and
      string_default_or_absent("dnsPolicy";"ClusterFirst") and
      string_default_or_absent("schedulerName";"default-scheduler") and
      ((has("enableServiceLinks") | not) or .enableServiceLinks == null or .enableServiceLinks == true) and
      string_default_or_absent("preemptionPolicy";"PreemptLowerPriority") and
      ((has("priority") | not) or .priority == null or .priority == 0) and
      string_default_or_absent("priorityClassName";"") and
      empty_array_or_absent("imagePullSecrets") and
      (($allow_node and ((.nodeName? == null) or (.nodeName | type == "string" and length > 0))) or
       (($allow_node | not) and .nodeName? == null));
    def normalize_container($allow_irsa):
      (if $allow_irsa then
        .env |= map(select(.name != "AWS_ROLE_ARN" and
          .name != "AWS_WEB_IDENTITY_TOKEN_FILE")) |
        .volumeMounts |= map(select(.name != "aws-iam-token"))
       else . end) |
      (.env |= map(del(.valueFrom.fieldRef.apiVersion))) |
      del(.args,.envFrom,.volumeDevices,.lifecycle,.stdin,.stdinOnce,.tty,
        .imagePullPolicy,.terminationMessagePath,.terminationMessagePolicy);
    def normalize_pod_spec($allow_irsa):
      (.containers |= map(normalize_container($allow_irsa))) |
      (if $allow_irsa then .volumes |= map(select(.name != "aws-iam-token")) else . end) |
      .tolerations = [launcher_toleration] |
      del(.nodeName,.initContainers,.ephemeralContainers,.serviceAccount,
        .dnsPolicy,.schedulerName,.enableServiceLinks,.preemptionPolicy,.priority,
        .priorityClassName,.imagePullSecrets);
    allowed_keys(["apiVersion","kind","metadata","spec","status"]) and
    .apiVersion == "v1" and .kind == "Pod" and
    (.metadata | allowed_keys(["name","generateName","namespace","uid","resourceVersion",
      "generation","creationTimestamp","labels","annotations","ownerReferences","finalizers",
      "managedFields"])) and
    .metadata.name == $pod and .metadata.uid == $uid and
    .metadata.namespace == "isolab" and .metadata.generation == 1 and
    (if $node == "" then
       ((has("status") | not) or
        ((.status | type) == "object" and
         (((.status | has("observedGeneration")) | not) or
          .status.observedGeneration == .metadata.generation)))
     else
       ((.status | type) == "object" and
        .status.observedGeneration == .metadata.generation)
     end) and
    (.metadata.labels | safe_pod_labels($job;$job_uid)) and
    .metadata.annotations == {"karpenter.sh/do-not-disrupt":"true"} and
    .metadata.ownerReferences == [{apiVersion:"batch/v1",kind:"Job",name:$job,uid:$job_uid,
      controller:true,blockOwnerDeletion:true}] and
    ((.metadata.finalizers? == null) or .metadata.finalizers == [] or
      .metadata.finalizers == ["batch.kubernetes.io/job-tracking"]) and
    ((.metadata.generateName? == null) or .metadata.generateName == ($job + "-")) and
    (.spec | safe_pod_spec(true;true)) and
    ($desired[0].spec.template.spec | safe_pod_spec(false;false)) and
    ((.spec | normalize_pod_spec(true)) ==
      ($desired[0].spec.template.spec | normalize_pod_spec(false))) and
    .spec.containers[0].image == $image and
    .spec.containers[0].command == $argv and
    (.spec.containers[0].env |
      map(select(.name != "AWS_ROLE_ARN" and .name != "AWS_WEB_IDENTITY_TOKEN_FILE")) |
      map(del(.valueFrom.fieldRef.apiVersion))) ==
      ($desired[0].spec.template.spec.containers[0].env |
        map(del(.valueFrom.fieldRef.apiVersion))) and
    .spec.containers[0].resources == $desired[0].spec.template.spec.containers[0].resources
  ' "$evidence/pod.json" >/dev/null || die "raw Pod bytes violate Job/image/command identity"
  if [[ -n "$node_name" ]]; then
    jq -e --arg node "$node_name" --arg digest "$image_digest" '
      .spec.nodeName == $node and (.status.containerStatuses | length) == 1 and
      .status.containerStatuses[0].name == "task" and
      (.status.containerStatuses[0].image | type == "string" and length > 0) and
      .status.containerStatuses[0].restartCount == 0 and
      (.status.containerStatuses[0].imageID | endswith("@" + $digest)) and
      (.status.containerStatuses[0].state.terminated.exitCode | type == "number" and
        floor == . and . >= 0 and . <= 255) and
      (.status.containerStatuses[0].state.terminated.reason | type == "string" and length > 0)
    ' "$evidence/pod.json" >/dev/null || die "scheduled Pod placement/runtime image mismatch"
    pod_exit_code=$(jq -r '.status.containerStatuses[0].state.terminated.exitCode' "$evidence/pod.json")
    pod_termination_reason=$(jq -r '.status.containerStatuses[0].state.terminated.reason' "$evidence/pod.json")
    pod_restarts=0
    pod_json=$(jq -cn --arg name "$pod_name" --arg uid "$pod_uid" --arg node "$node_name" \
      '{name:$name,uid:$uid,node_name:$node}')
  else
    jq -e '.spec.nodeName == null' "$evidence/pod.json" >/dev/null || die "Pod placement snapshots disagree"
    pod_json=$(jq -cn --arg name "$pod_name" --arg uid "$pod_uid" '{name:$name,uid:$uid,node_name:null}')
  fi
else
  [[ ! -e "$evidence/pod.json" && ! -L "$evidence/pod.json" ]] \
    || die "pod.json exists although status reports no Pod"
fi

jq -e --arg results "$results_root" '
  .schema == "lead15.object-heads/v1" and .results_prefix == $results and (.objects | type == "object")
' "$evidence/object-heads.json" >/dev/null || die "object head bundle mismatch"
if [[ -n "$pod_uid" ]]; then
  result_key_prefix="runs/$job_name/$pod_uid/"
  jq -e --slurpfile heads "$evidence/object-heads.json" --arg results "$results_root" \
    --arg prefix "$result_key_prefix" '
    ["records.jsonl","worker-start.json","worker-exit.json","worker-upload.json",
     "runtime-provenance.json","build-info.json","binary.sha256","stdout.log",
     "stderr.log","SHA256SUMS","timeout-events.log"] as $allowed |
    .results_uri == $results and (.objects | type == "array") and .next_token == null and
    ([.objects[].Key] | length == (unique | length)) and
    all(.objects[]; .Key as $key |
      ($key | startswith($prefix)) and
      (($key[($prefix|length):]) as $name | ($allowed | index($name)) != null)) and
    (([.objects[].Key[($prefix|length):]] | sort) == ($heads[0].objects | keys | sort))
  ' "$evidence/results.json" >/dev/null \
    || die "results listing/head set is incomplete, duplicated, unknown, or misbound"
  while IFS= read -r name; do
    [[ -f "$evidence/$name" && ! -L "$evidence/$name" ]] \
      || die "listed worker object was not retained locally: $name"
  done < <(jq -r '.objects | keys[]' "$evidence/object-heads.json")
else
  jq -e --slurpfile heads "$evidence/object-heads.json" --arg results "$results_root" '
    .results_uri == $results and .next_token == null and .objects == [] and
    $heads[0].objects == {}
  ' "$evidence/results.json" >/dev/null \
    || die "zero-Pod result listing or object heads are nonempty"
fi

remote_descriptor() {
  local name=$1 path=$evidence/$1 key
  [[ -n "$pod_uid" ]] || die "$name exists without a Pod identity"
  key="runs/$job_name/$pod_uid/$name"
  [[ -f "$path" && ! -L "$path" ]] || die "$name must be a regular, non-symlink file"
  local bytes digest etag version checksum checksum_hex
  bytes=$(size "$path")
  digest=$(sha "$path")
  jq -e --arg name "$name" --argjson bytes "$bytes" '
    .objects[$name].ContentLength == $bytes and
    (.objects[$name].ETag | type == "string" and length > 1) and
    (.objects[$name].VersionId | type == "string" and length > 0) and
    (.objects[$name].ChecksumSHA256 | type == "string" and length == 44)
  ' "$evidence/object-heads.json" >/dev/null || die "$name S3 head/version does not match local bytes"
  etag=$(jq -er --arg name "$name" '.objects[$name].ETag' "$evidence/object-heads.json")
  version=$(jq -er --arg name "$name" '.objects[$name].VersionId' "$evidence/object-heads.json")
  checksum=$(jq -er --arg name "$name" '.objects[$name].ChecksumSHA256' "$evidence/object-heads.json")
  checksum_hex=$(checksum_sha256_hex "$checksum") \
    || die "$name S3 checksum is not a canonical SHA-256 checksum"
  [[ "$checksum_hex" == "$digest" ]] || die "$name S3 checksum does not match local bytes"
  jq -e --arg key "$key" --arg etag "$etag" --argjson bytes "$bytes" '
    ([.objects[] | select(.Key == $key and .Size == $bytes and .ETag == $etag and
      (.ChecksumAlgorithm | type == "array" and index("SHA256") != null))] | length) == 1
  ' "$evidence/results.json" >/dev/null || die "$name missing or mismatched in the complete results listing"
  jq -cn --arg path "$name" --arg sha "$digest" --argjson bytes "$bytes" \
    --arg checksum "$checksum" --arg uri "${results_root}${pod_uid}/$name" \
    --arg etag "$etag" --arg version "$version" \
    '{path:$path,sha256:$sha,checksum_sha256:$checksum,bytes:$bytes,s3_uri:$uri,
      etag:$etag,version_id:$version,readback_verified:true}'
}
optional_remote_descriptor() {
  local name=$1
  if [[ -f "$evidence/$name" && ! -L "$evidence/$name" ]]; then
    remote_descriptor "$name"
  elif [[ -e "$evidence/$name" || -L "$evidence/$name" ]]; then
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
binary_hash_remote=$(optional_remote_descriptor binary.sha256)
stdout_remote=$(optional_remote_descriptor stdout.log)
stderr_remote=$(optional_remote_descriptor stderr.log)
sums_remote=$(optional_remote_descriptor SHA256SUMS)
timeout_remote=$(optional_remote_descriptor timeout-events.log)

# Every recovered byte is independently admitted even when interruption left a
# strict prefix of the normal worker set. Only a complete coherent set can
# establish a scientific terminal class; a proper subset remains infrastructure
# failure while preserving its hashes and S3 versions in the receipt.
worker_complete=false
if [[ "$start_remote" != null && "$exit_remote" != null && "$upload_remote" != null && \
      "$runtime_remote" != null && "$build_remote" != null && "$sums_remote" != null && \
      "$binary_hash_remote" != null && "$stdout_remote" != null && "$stderr_remote" != null && \
      "$timeout_remote" != null ]]; then
  worker_complete=true
fi
for name in worker-start.json worker-exit.json worker-upload.json runtime-provenance.json build-info.json; do
  if [[ -f "$evidence/$name" ]]; then
    jq -e . "$evidence/$name" >/dev/null || die "invalid JSON in $name"
  fi
done

terminal_class=infrastructure_failure
deadline_reached=false
timeout_fired=false
exit_code=null
payload_sha=null
payload_checksum=null
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
  payload_checksum=$(jq -c '.checksum_sha256' <<<"$payload_remote")
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
    def utc_timestamp:
      type == "string" and
      test("^[0-9]{4}-(0[1-9]|1[0-2])-([0-2][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:([0-5][0-9])Z$") and
      (. as $value | try (fromdateiso8601 | strftime("%Y-%m-%dT%H:%M:%SZ") == $value)
        catch false);
    ((keys | sort) == (["schema","q","source_manifest_sha256","instrument_commit",
      "protocol_sha256","binary_sha256","image_digest","worker_sha256",
      "scientific_deadline_seconds","job_name","pod_uid","node_name",
      "scientific_environment","started_at"] | sort)) and
    .schema == "lead15.worker-start/v1" and .q == $q and
    .source_manifest_sha256 == $manifest and .instrument_commit == $commit and
    .protocol_sha256 == $protocol and .binary_sha256 == $binary and
    .image_digest == $image and .worker_sha256 == $worker and
    .scientific_deadline_seconds == 3600 and
    .scientific_environment == {RAYON_NUM_THREADS:"1",OMP_NUM_THREADS:"1"} and
    .job_name == $job and .pod_uid == $pod and .node_name == $node and
    (.started_at | utc_timestamp)
  ' "$evidence/worker-start.json" >/dev/null || die "worker-start identity mismatch"
fi
if [[ "$build_remote" != null ]]; then
  [[ "$(sha "$evidence/build-info.json")" == "$build_info_sha" ]] || die "build-info byte hash mismatch"
  jq -e --arg commit "$instrument_commit" --arg binary "$binary_sha" --arg worker "$worker_sha" '
    ((keys | sort) == (["schema","source_revision","cargo_lock_sha256","rustc_verbose",
      "target_triple","base_image_uri","image_recipe_sha256","build_command",
      "lead15_binary_sha256","lead15_worker_sha256","binary_path","worker_path"] | sort)) and
    .schema == "lead15.build-info/v1" and .source_revision == $commit and
    .lead15_binary_sha256 == $binary and .lead15_worker_sha256 == $worker and
    .binary_path == "/opt/lead15/lead15_diem_quartic" and
    .worker_path == "/opt/lead15/lead15-worker" and
    .build_command == ["cargo","build","--locked","--release","--example","lead15_diem_quartic"]
  ' "$evidence/build-info.json" >/dev/null || die "build-info semantic mismatch"
fi
if [[ "$binary_hash_remote" != null ]]; then
  expected_binary_hash_line="$binary_sha  /opt/lead15/lead15_diem_quartic"
  [[ "$(<"$evidence/binary.sha256")" == "$expected_binary_hash_line" && \
      "$(size "$evidence/binary.sha256")" == $((${#expected_binary_hash_line} + 1)) ]] \
    || die "binary.sha256 bytes mismatch"
fi
if [[ "$exit_remote" != null ]]; then
  jq -e --arg job "$job_name" --arg pod "$pod_uid" --arg node "$node_name" --arg results "${results_root}${pod_uid}/" '
    def utc_timestamp:
      type == "string" and
      test("^[0-9]{4}-(0[1-9]|1[0-2])-([0-2][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:([0-5][0-9])Z$") and
      (. as $value | try (fromdateiso8601 | strftime("%Y-%m-%dT%H:%M:%SZ") == $value)
        catch false);
    ((keys | sort) == (["schema","terminal_class","deadline_reached","timeout_fired",
      "exit_code","child_elapsed_seconds","payload","started_at","finished_at",
      "results_uri","job_name","pod_uid","node_name"] | sort)) and
    ((.payload | keys | sort) == (["path","sha256","bytes","final_newline"] | sort)) and
    .schema == "lead15.worker-exit/v1" and .job_name == $job and .pod_uid == $pod and
    .node_name == $node and .results_uri == $results and
    (.deadline_reached | type == "boolean") and
    (.timeout_fired | type == "boolean") and
    (.exit_code | type == "number" and floor == . and . >= 0 and . <= 255) and
    (.child_elapsed_seconds | type == "number" and floor == . and . >= 0) and
    (.started_at | utc_timestamp) and
    (.finished_at | utc_timestamp) and
    .payload.path == "records.jsonl" and
    (.payload.sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
    (.payload.bytes | type == "number" and floor == . and . >= 0) and
    (.payload.final_newline | type == "boolean") and
    ((.terminal_class == "completed" and .deadline_reached == false and
       .timeout_fired == false and .exit_code == 0) or
     (.terminal_class == "deadline" and .deadline_reached == true and
       .timeout_fired == true and (.exit_code == 124 or .exit_code == 137) and
       .child_elapsed_seconds >= 3599) or
     (.terminal_class == "scientific_failure" and .deadline_reached == false and
       .timeout_fired == false and .exit_code != 0 and
       .exit_code != 130 and .exit_code != 137 and .exit_code != 143) or
     (.terminal_class == "infrastructure_failure" and .deadline_reached == false and
       (.timeout_fired == false or
        ((.exit_code == 124 or .exit_code == 137) and .child_elapsed_seconds >= 3599))))
  ' "$evidence/worker-exit.json" >/dev/null || die "worker-exit identity/class mismatch"
  worker_terminal=$(jq -er '.terminal_class' "$evidence/worker-exit.json")
  deadline_reached=$(jq -r '.deadline_reached' "$evidence/worker-exit.json")
  timeout_fired=$(jq -r '.timeout_fired' "$evidence/worker-exit.json")
  exit_code=$(jq -r '.exit_code' "$evidence/worker-exit.json")
  started_at=$(jq -c '.started_at' "$evidence/worker-exit.json")
  finished_at=$(jq -c '.finished_at' "$evidence/worker-exit.json")
  if [[ "$payload_remote" != null ]]; then
    jq -e --arg sha "$payload_sha" --argjson bytes "$payload_bytes" --argjson newline "$final_newline" '
      .payload.sha256 == $sha and .payload.bytes == $bytes and .payload.final_newline == $newline
    ' "$evidence/worker-exit.json" >/dev/null || die "worker-exit payload metadata mismatch"
  else
    if ! jq -e --arg empty "$empty_sha" \
      '.payload.bytes == 0 and .payload.sha256 == $empty and .payload.final_newline == false' \
      "$evidence/worker-exit.json" >/dev/null; then
      # The recursive upload can stop after worker-exit but before records.jsonl.
      # Preserve that authenticated strict prefix, but never promote it to a
      # scientific terminal classification.
      worker_complete=false
    fi
  fi
fi
if [[ "$start_remote" != null && "$exit_remote" != null ]]; then
  jq -e --slurpfile start "$evidence/worker-start.json" \
    '.started_at == $start[0].started_at' "$evidence/worker-exit.json" >/dev/null \
    || die "worker start/exit timestamps come from different executions"
fi

if [[ "$timeout_remote" != null ]]; then
  timeout_event_text=$(<"$evidence/timeout-events.log")
  timeout_event_bytes=$(size "$evidence/timeout-events.log")
  term_event="timeout: sending signal TERM to command 'bash'"
  kill_event="timeout: sending signal KILL to command 'bash'"
  timeout_event_kind=invalid
  if [[ $timeout_event_bytes == 0 ]]; then
    timeout_event_kind=none
  elif [[ $timeout_event_bytes == $((${#term_event} + 1)) && "$timeout_event_text" == "$term_event" ]]; then
    timeout_event_kind=term
  elif [[ $timeout_event_bytes == $((${#term_event} + ${#kill_event} + 2)) && \
          "$timeout_event_text" == "$term_event"$'\n'"$kill_event" ]]; then
    timeout_event_kind=term_kill
  else
    die "timeout event log has noncanonical bytes"
  fi
fi
if [[ "$timeout_remote" != null && "$exit_remote" != null ]]; then
  expected_timeout_fired=false
  if [[ "$exit_code" == 124 && "$timeout_event_kind" == term ]]; then
    expected_timeout_fired=true
  elif [[ "$exit_code" == 137 && "$timeout_event_kind" == term_kill ]]; then
    expected_timeout_fired=true
  elif [[ "$timeout_event_kind" != none ]]; then
    die "timeout event log does not match the terminal exit"
  fi
  [[ "$timeout_fired" == "$expected_timeout_fired" ]] \
    || die "worker timeout-fired claim does not match timeout event bytes"
fi
if [[ "$runtime_remote" != null ]]; then
  jq -e --arg job "$job_name" --arg pod_name "$pod_name" --arg pod "$pod_uid" --arg node "$node_name" '
    ((keys | sort) == (["schema","cpu_model","logical_cpus","memory_bytes","kernel",
      "job_name","pod_name","pod_uid","node_name"] | sort)) and
    .schema == "lead15.runtime-provenance/v1" and .job_name == $job and
    .pod_name == $pod_name and .pod_uid == $pod and .node_name == $node and
    (.cpu_model | type == "string" and length > 0) and
    (.logical_cpus | type == "number" and floor == . and . > 0) and
    (.memory_bytes | type == "number" and floor == . and . > 0) and
    (.kernel | type == "string" and length > 0)
  ' "$evidence/runtime-provenance.json" >/dev/null || die "runtime provenance mismatch"
  resource_provenance=$(jq -c '{cpu_model,logical_cpus,memory_bytes,kernel,node_name}' "$evidence/runtime-provenance.json")
fi
if [[ "$upload_remote" != null ]]; then
  jq -e --arg results "${results_root}${pod_uid}/" '
    ((keys | sort) == (["schema","uploaded","upload_exit","terminal_class",
      "child_exit_code","results_uri","payload_sha256","payload_bytes"] | sort)) and
    .schema == "lead15.worker-upload/v1" and .results_uri == $results and
    (.uploaded | type == "boolean") and
    (.upload_exit | type == "number" and floor == . and . >= 0 and . <= 255) and
    (.uploaded == (.upload_exit == 0)) and
    (.terminal_class == "completed" or .terminal_class == "deadline" or
     .terminal_class == "scientific_failure" or .terminal_class == "infrastructure_failure") and
    (.child_exit_code | type == "number" and floor == . and . >= 0 and . <= 255) and
    (.payload_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
    (.payload_bytes | type == "number" and floor == . and . >= 0)
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
    if ! jq -e --arg empty "$empty_sha" '.payload_sha256 == $empty and .payload_bytes == 0' \
      "$evidence/worker-upload.json" >/dev/null; then
      worker_complete=false
    fi
  fi
fi

if [[ "$exit_remote" != null && "$upload_remote" != null ]]; then
  jq -e --arg terminal "$worker_terminal" --argjson exit "$exit_code" '
    .terminal_class == $terminal and .child_exit_code == $exit
  ' "$evidence/worker-upload.json" >/dev/null \
    || die "worker exit/upload terminal metadata mismatch"
fi

if [[ "$sums_remote" != null ]]; then
  # Do not execute paths from the worker checksum file. Compare only fixed names.
  file_is_ascii "$evidence/SHA256SUMS" || die "SHA256SUMS must be ASCII"
  for name in worker-start.json worker-exit.json runtime-provenance.json build-info.json \
    binary.sha256 stdout.log stderr.log timeout-events.log; do
    if [[ -f "$evidence/$name" ]]; then
      expected=$(checksum_manifest_hash "$evidence/SHA256SUMS" "./$name") \
        || die "SHA256SUMS entry count/shape mismatch for $name"
      [[ "$expected" =~ ^[0-9a-f]{64}$ && "$expected" == "$(sha "$evidence/$name")" ]] \
        || die "SHA256SUMS mismatch for $name"
    fi
  done
  if [[ "$payload_remote" != null ]]; then
    expected=$(checksum_manifest_hash "$evidence/SHA256SUMS" "./records.jsonl") \
      || die "SHA256SUMS entry count/shape mismatch for records.jsonl"
    [[ "$expected" == "$payload_sha" ]] || die "SHA256SUMS mismatch for records.jsonl"
  fi
fi

if [[ $worker_complete == true ]]; then
  pod_exit_matches=false
  if [[ "$pod_exit_code" != null && "$pod_exit_code" == "$exit_code" ]]; then
    pod_exit_matches=true
  fi
  if [[ "$worker_terminal" == completed && "$job_state" == succeeded && "$exit_code" == 0 && \
        "$timeout_fired" == false && "$pod_exit_matches" == true && \
        "$pod_termination_reason" == Completed && "$bulk_uploaded" == true && \
        "$payload_readback" == true && "$final_newline" == true ]]; then
    terminal_class=completed
  elif [[ "$worker_terminal" == deadline && "$job_state" == failed && \
          "$deadline_reached" == true && "$timeout_fired" == true && \
          ( "$exit_code" == 124 || "$exit_code" == 137 ) && "$pod_exit_matches" == true && \
          "$pod_termination_reason" == Error && "$bulk_uploaded" == true ]]; then
    terminal_class=deadline
  elif [[ "$worker_terminal" == scientific_failure && "$job_state" == failed && "$deadline_reached" == false && \
          "$timeout_fired" == false && "$exit_code" != 0 && "$pod_exit_matches" == true && \
          "$pod_termination_reason" == Error && "$bulk_uploaded" == true ]]; then
    terminal_class=scientific_failure
  else
    terminal_class=infrastructure_failure
  fi
elif [[ "$exit_remote" == null ]]; then
  # Use the raw Pod's terminated status only as infrastructure provenance. It
  # can never manufacture a scientific terminal classification.
  if [[ $attempts == 1 && -n "$node_name" ]]; then
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
  --argjson checksum "$payload_checksum" \
  --argjson bytes "$payload_bytes" --argjson etag "$payload_etag" --argjson version "$payload_version" \
  --argjson uploaded "$payload_uploaded" --argjson readback "$payload_readback" --argjson newline "$final_newline" \
  '{present:$present,s3_uri:$uri,sha256:$sha,checksum_sha256:$checksum,bytes:$bytes,etag:$etag,version_id:$version,
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
  --argjson binary_sha256 "$(optional_local_descriptor binary.sha256)" \
  --argjson stdout "$(optional_local_descriptor stdout.log)" \
  --argjson stderr "$(optional_local_descriptor stderr.log)" \
  --argjson sums "$(optional_local_descriptor SHA256SUMS)" \
  --argjson timeout_events "$(optional_local_descriptor timeout-events.log)" \
  '{request:$request,request_head:$request_head,submission:$submission,job:$job,pod:$pod,
    status:$status,results:$results,object_heads:$heads,payload:$payload,worker_start:$start,
    worker_exit:$exit,worker_upload:$upload,runtime_provenance:$runtime,build_info:$build,
    binary_sha256:$binary_sha256,stdout:$stdout,stderr:$stderr,
    sha256sums:$sums,timeout_events:$timeout_events}')

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
