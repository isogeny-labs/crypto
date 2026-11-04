#!/usr/bin/env bash
# Offline contract test for launch-remote-command, run-command's request
# validator and status-command's request validator (stubbed aws; no cluster).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/bin" "$tmp/s3/fixture-bucket" "$tmp/client"
cat > "$tmp/bin/aws" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
loc() { case $1 in s3://*) printf '%s/%s\n' "$FAKE_S3_ROOT" "${1#s3://}" ;; *) printf '%s\n' "$1" ;; esac; }
case "$1 $2" in
  's3 cp')
    source=$(loc "$3"); destination=$(loc "$4")
    mkdir -p "$(dirname "$destination")"
    cp "$source" "$destination"
    ;;
  'codebuild start-build')
    cp "${4#file://}" "$START_REQUEST_CAPTURE"
    jq -n --arg id "isolab-experiment-launcher:$LAUNCHER_ID" '{build:{id:$id}}'
    ;;
  'codebuild batch-get-builds')
    jq -n '{builds:[{buildStatus:"SUCCEEDED"}]}'
    ;;
  *) exit 90 ;;
esac
STUB
chmod +x "$tmp/bin/aws"
export PATH="$tmp/bin:$PATH" FAKE_S3_ROOT="$tmp/s3" LAUNCHER_ID=cmd-test-1 \
  START_REQUEST_CAPTURE="$tmp/start.json" ISOLAB_BUCKET=fixture-bucket \
  ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher AWS_REGION=us-east-2 ISOLAB_CLIENT_OUTPUT="$tmp/client"
image="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:$(printf 'a%.0s' $(seq 64))"
rev=$(printf 'b%.0s' $(seq 40))
# The validators are the inline jq programs of the two launcher-side scripts.
extract_validator() { awk "/^jq -e .*'\$/{f=1;next} f&&/^' \"\\\$receipt_dir\/request.json\"/{exit} f" "$1"; }
extract_validator "$here/run-command" > "$tmp/command.jq"
awk "/^jq -e 'type==\"object\"/{sub(/^jq -e '/,\"\");f=1} f{print} f&&/\\\$receipt_dir\/request.json/{exit}" "$here/status-command" \
  | sed '$d' > "$tmp/status.jq"
[[ -s "$tmp/command.jq" && -s "$tmp/status.jq" ]]
valid=$(jq -n --arg image "$image" --arg rev "$rev" '{schema:"isolab.eks.command-request/v1",
  job:{job_name:"arity-test",image:$image,command:["/opt/study/launch/job-koblitz.sh"],cpu:4,memory:"12Gi",
       seconds:60900,isolation_profile:"l2",instance_type:"c8a.2xlarge",retries:0,wait_seconds:600},
  provenance:{harness:"test",source_revision:$rev,study:"koblitz_m5_m6"}}')
repo=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks
jq -e --arg repository "$repo" -f "$tmp/command.jq" <<< "$valid" >/dev/null
for bad in '.job.cpu=3' '.job.retries=1' '.job.wait_seconds=3001' '.job.seconds=604801' '.job.isolation_profile="strict"' \
           '.job.command=[]' '.job.command=["a\nb"]' '.job.image="docker.io/library/ubuntu:24.04"' \
           '.job.extra=1' '.provenance.source_revision="abc"' 'del(.provenance)' '.job.instance_type="C8A.2xlarge"'; do
  if jq -e --arg repository "$repo" -f "$tmp/command.jq" <<< "$(jq "$bad" <<< "$valid")" >/dev/null 2>&1; then
    echo "validator accepted invalid request: $bad" >&2; exit 1
  fi
done
jq -e -f "$tmp/status.jq" <<< '{"schema":"isolab.eks.command-status/v1","job_name":"arity-test","log_tail_lines":200}' >/dev/null
! jq -e -f "$tmp/status.jq" <<< '{"schema":"isolab.eks.command-status/v1","job_name":"Arity","log_tail_lines":200}' >/dev/null 2>&1
# launch-remote-command: a submission run uploads request, run-command and buildspec by sha and starts one build.
printf '%s\n' "$valid" > "$tmp/request.json"
expected_digest=$(jq -S . "$tmp/request.json" | sha256sum | cut -d ' ' -f1)
run_sha=$(sha256sum "$here/run-command" | cut -d ' ' -f1)
spec_sha=$(sha256sum "$here/current-run-request-buildspec.yml" | cut -d ' ' -f1)
mkdir -p "$tmp/s3/fixture-bucket/runs/arity-test/launcher/cmd-test-1"
jq -n --arg digest "$expected_digest" --arg rev "$rev" '{schema:"isolab.eks.command-result/v1",state:"submitted",job_name:"arity-test",
  job_uid:"11111111-2222-3333-4444-555555555555",pods:[],request_sha256:$digest,exit_code:0}' \
  > "$tmp/s3/fixture-bucket/runs/arity-test/launcher/cmd-test-1/result.json"
"$here/launch-remote-command" "$tmp/request.json" > "$tmp/launch.out"
[[ -f "$tmp/s3/fixture-bucket/inputs/requests/$expected_digest.json" ]]
[[ -f "$tmp/s3/fixture-bucket/inputs/requests/$run_sha.run-request" ]]
[[ -f "$tmp/s3/fixture-bucket/inputs/requests/$spec_sha.buildspec.yml" ]]
jq -e --arg key "inputs/requests/$expected_digest.json" --arg sha "$run_sha" --arg spec "arn:aws:s3:::fixture-bucket/inputs/requests/$spec_sha.buildspec.yml" '
  .projectName=="isolab-experiment-launcher" and .buildspecOverride==$spec and .timeoutInMinutesOverride==30
  and (.environmentVariablesOverride|map({(.name):.value})|add)=={ISOLAB_REQUEST_KEY:$key,ISOLAB_RUN_REQUEST_SHA256:$sha}' "$tmp/start.json" >/dev/null
grep -q 'Launcher: isolab-experiment-launcher:cmd-test-1' "$tmp/launch.out"
# A failed submission receipt makes the client exit non-zero.
jq '.state="job_exists"' "$tmp/s3/fixture-bucket/runs/arity-test/launcher/cmd-test-1/result.json" > "$tmp/r.json"
mv "$tmp/r.json" "$tmp/s3/fixture-bucket/runs/arity-test/launcher/cmd-test-1/result.json"
if "$here/launch-remote-command" "$tmp/request.json" >/dev/null 2>&1; then echo 'job_exists receipt accepted' >&2; exit 1; fi
# A status request selects status-command and a per-minute idempotency salt.
"$here/../../../../research/arity_completion_20261104/launch/status-request.sh" arity-test 50 > "$tmp/status.json" 2>/dev/null \
  || jq -n '{schema:"isolab.eks.command-status/v1",job_name:"arity-test",log_tail_lines:50}' > "$tmp/status.json"
status_digest=$(jq -S . "$tmp/status.json" | sha256sum | cut -d ' ' -f1)
status_sha=$(sha256sum "$here/status-command" | cut -d ' ' -f1)
jq -n --arg digest "$status_digest" '{schema:"isolab.eks.command-status-result/v1",state:"active",job_name:"arity-test",request_sha256:$digest,exit_code:0}' \
  > "$tmp/s3/fixture-bucket/runs/arity-test/launcher/cmd-test-1/result.json"
"$here/launch-remote-command" "$tmp/status.json" > "$tmp/status.out"
[[ -f "$tmp/s3/fixture-bucket/inputs/requests/$status_sha.run-request" ]]
jq -e --arg sha "$status_sha" '(.environmentVariablesOverride|map({(.name):.value})|add).ISOLAB_RUN_REQUEST_SHA256==$sha and .timeoutInMinutesOverride==20' "$tmp/start.json" >/dev/null
echo 'launch-remote-command contract OK'
