#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
bucket=fixture-bucket job=gh-41260778856-1
launcher=47bce409-b8ea-4c00-8660-c0bc989ecb02
request=0fc352b643f362f84fdc69c7d1062a1502dd228ae25b435ad72879c6b254ef55
source=b4d751706eb2244314ebc9b84ba9545485ba4ac7
binary=0784fe47212d64dd2ee81510d3c28d2f604782086500135a3114c61738f854b7
pod=aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee
launcher_dir="$tmp/s3/$bucket/runs/$job/launcher/$launcher"
pod_dir="$tmp/s3/$bucket/runs/$job/$pod"
mkdir -p "$launcher_dir" "$pod_dir" "$tmp/bin"
ln -s "$(realpath "$here/../../../../scripts/tests/fake-aws.sh")" "$tmp/bin/aws"

jq -n --arg job "$job" --arg source "$source" --arg binary "$binary" \
  '{schema:"isolab.eks.request/v1",job:{job_name:$job,build:{image_uri:"104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",source_revision:$source,ecbench_sha256:$binary}},provenance:{source_revision:$source}}' \
  > "$launcher_dir/request.json"
actual_request=$(sha256sum "$launcher_dir/request.json" | cut -d ' ' -f1)
# The test exercises identity validation with the request's real normalized hash.
request=$actual_request
jq -n --arg job "$job" --arg launcher "$launcher" --arg request "$request" \
  --arg source "$source" --arg binary "$binary" \
  '{schema:"isolab.eks.result/v1",state:"failed",job_name:$job,job_uid:"11111111-2222-4333-8444-555555555555",pod_uid:"",results_uri:"",receipt_uri:"s3://fixture",request_sha256:$request,launcher:("isolab-experiment-launcher:"+$launcher),exit_code:23,build:{image_uri:"104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",source_revision:$source,ecbench_sha256:$binary}}' \
  > "$launcher_dir/result.json"
jq -n --arg job "$job" --arg pod "$pod" \
  '{job:{job_name:$job,uid:"11111111-2222-4333-8444-555555555555",state:"failed"},
    pods:[{uid:$pod,status:{phase:"Failed",containerStatuses:[{state:{terminated:{exitCode:42,reason:"Error"}}}]}}]}' \
  > "$launcher_dir/status.json"

jq -n --arg job "$job" --arg pod "$pod" '{exit_code:42,job_name:$job,pod_uid:$pod,node_name:"fixture"}' > "$pod_dir/job-exit.json"
jq -n --arg source "$source" --arg binary "$binary" '{source_revision:$source,ecbench_sha256:$binary}' > "$pod_dir/build-info.json"
printf '%s  /usr/local/bin/ecbench\n' "$binary" > "$pod_dir/binary.sha256"
printf '{}\n' > "$pod_dir/input-spec.json"
jq -n '{accepted:false,executions:2,below_l2:[{seq:1}],invalid_outcomes:[],grades:[{level:"L1",count:2}]}' > "$pod_dir/l2-gate.json"
jq -n '{wall:{status:"rejected",reasons:["private control detail"],pairs:5,
  pairs_below_level:1,aa_pairs:5,aa_ci95:[123.456,234.567],outside_noise:123.456}}' \
  > "$pod_dir/control-comparison.json"
printf 'private timing line\n' > "$pod_dir/run.stderr"
(cd "$pod_dir" && sha256sum job-exit.json build-info.json binary.sha256 input-spec.json l2-gate.json \
  control-comparison.json run.stderr) > "$pod_dir/SHA256SUMS"
jq -n --arg job "$job" --arg pod "$pod" '{next_token:null,objects:[
  {Key:("runs/"+$job+"/"+$pod+"/SHA256SUMS")},
  {Key:("runs/"+$job+"/"+$pod+"/job-exit.json")},
  {Key:("runs/"+$job+"/"+$pod+"/build-info.json")},
  {Key:("runs/"+$job+"/"+$pod+"/binary.sha256")},
  {Key:("runs/"+$job+"/"+$pod+"/input-spec.json")},
  {Key:("runs/"+$job+"/"+$pod+"/l2-gate.json")},
  {Key:("runs/"+$job+"/"+$pod+"/control-comparison.json")},
  {Key:("runs/"+$job+"/"+$pod+"/run.stderr")}],results_uri:("s3://fixture/runs/"+$job+"/")}' > "$launcher_dir/objects.json"

PATH="$tmp/bin:$PATH" FAKE_S3_ROOT="$tmp/s3" ISOLAB_BUCKET="$bucket" \
  "$here/diagnose-failed-run" "$job" "$launcher" "$request" "$source" "$binary" 41260778856 "$tmp/out.json"
jq -e --arg pod "$pod" '
  .schema=="isolab.eks.failed-diagnosis/v1" and .experiment_retry==false
  and .pods==[{pod_uid:$pod,classification:"l2_isolation_gate",exit_code:42,
    exit_source:"hash_verified_job_exit",artifact_access:"selected_objects_hash_verified",manifest_verified:true,
    l2:{accepted:false,executions:2,below_l2:1,invalid_outcomes:0,grades:[{level:"L1",count:2}]},
    audit:null,control:{wall:{status:"rejected",pairs:5,pairs_below_level:1,aa_pairs:5,aa_ci95_present:true}},
    owner_checks:{run:false,replay:false},
    private_stderr:{run_sha256:.pods[0].private_stderr.run_sha256,audit_sha256:null,content_published:false}}]
  and (.pods[0].private_stderr.run_sha256|length)==64' "$tmp/out.json" >/dev/null
! rg -q 'private timing line' "$tmp/out.json"
! rg -q 'private control detail|123\.456|234\.567' "$tmp/out.json"

# A complete artifact-upload failure remains classifiable from the retained
# Kubernetes status without inventing hash verification or exposing logs.
rm -rf "$pod_dir"
jq -n --arg job "$job" '{next_token:null,objects:[],results_uri:("s3://fixture/runs/"+$job+"/")}' \
  > "$launcher_dir/objects.json"
PATH="$tmp/bin:$PATH" FAKE_S3_ROOT="$tmp/s3" ISOLAB_BUCKET="$bucket" \
  "$here/diagnose-failed-run" "$job" "$launcher" "$request" "$source" "$binary" 41260778856 "$tmp/status-only.json"
jq -e --arg pod "$pod" '.pods==[{pod_uid:$pod,classification:"l2_isolation_gate",exit_code:42,
  exit_source:"kubernetes_status",artifact_access:"manifest_not_listed",manifest_verified:false,
  l2:null,audit:null,control:null,
  owner_checks:{run:false,replay:false},
  private_stderr:{run_sha256:null,audit_sha256:null,content_published:false}}]' "$tmp/status-only.json" >/dev/null

# A listed payload that this identity cannot read still returns the identity-
# checked Kubernetes classification and records the narrower evidence source.
jq -n --arg job "$job" --arg pod "$pod" '{next_token:null,objects:[
  {Key:("runs/"+$job+"/"+$pod+"/SHA256SUMS")},
  {Key:("runs/"+$job+"/"+$pod+"/job-exit.json")}],results_uri:("s3://fixture/runs/"+$job+"/")}' \
  > "$launcher_dir/objects.json"
PATH="$tmp/bin:$PATH" FAKE_S3_ROOT="$tmp/s3" ISOLAB_BUCKET="$bucket" \
  "$here/diagnose-failed-run" "$job" "$launcher" "$request" "$source" "$binary" 41260778856 \
  "$tmp/read-unavailable.json" 2> "$tmp/read-unavailable.stderr"
jq -e --arg pod "$pod" '.pods==[{pod_uid:$pod,classification:"l2_isolation_gate",exit_code:42,
  exit_source:"kubernetes_status",artifact_access:"payload_read_unavailable",manifest_verified:false,
  l2:null,audit:null,control:null,owner_checks:{run:false,replay:false},
  private_stderr:{run_sha256:null,audit_sha256:null,content_published:false}}]' \
  "$tmp/read-unavailable.json" >/dev/null

if PATH="$tmp/bin:$PATH" FAKE_S3_ROOT="$tmp/s3" ISOLAB_BUCKET="$bucket" \
  "$here/diagnose-failed-run" "$job" "$launcher" "$(printf 'f%.0s' {1..64})" "$source" "$binary" 41260778856 "$tmp/bad.json" >/dev/null 2>&1; then
  echo 'diagnosis accepted the wrong request hash' >&2
  exit 1
fi
printf 'Fixed failed-run diagnosis, hash verification and privacy controls passed\n'
