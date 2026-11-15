#!/usr/bin/env bash
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

image=$(jq -r .image_uri "$here/../image.json")
source_revision=$(printf 'a%.0s' {1..40})
binary=$(printf 'b%.0s' {1..64})
jq -n --arg image "$image" --arg source "$source_revision" --arg binary "$binary" '
  {schema:"isolab.eks.request/v1",job:{job_name:"gh-123-1",seconds:1,
    build:{image_uri:$image,source_revision:$source,ecbench_sha256:$binary}},
    provenance:{source_revision:$source}}
' > "$tmp/request.json"
"$here/validate-request" "$tmp/request.json" > "$tmp/normalized.json"
request_sha=$(sha256sum "$tmp/normalized.json" | cut -d ' ' -f1)
launcher_id=11111111-2222-4333-8444-555555555555
result_dir="$tmp/s3/fixture-bucket/runs/gh-123-1/launcher/$launcher_id"
mkdir -p "$result_dir"
jq -n --arg request "$request_sha" --arg image "$image" --arg source "$source_revision" \
  --arg binary "$binary" '
  {schema:"isolab.eks.result/v1",state:"succeeded",exit_code:0,job_name:"gh-123-1",
   request_sha256:$request,
   build:{image_uri:$image,source_revision:$source,ecbench_sha256:$binary}}
' > "$result_dir/result.json"

PATH="$tmp/bin:$PATH" FAKE_S3_ROOT="$tmp/s3" START_REQUEST_CAPTURE="$tmp/start.json" \
  LAUNCHER_ID="$launcher_id" ISOLAB_BUCKET=fixture-bucket \
  ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher AWS_REGION=us-east-2 \
  ISOLAB_CLIENT_OUTPUT="$tmp/client" "$here/launch-remote" "$tmp/request.json" >/dev/null

run_sha=$(sha256sum "$here/run-request" | cut -d ' ' -f1)
buildspec_sha=$(sha256sum "$here/current-run-request-buildspec.yml" | cut -d ' ' -f1)
jq -e --arg run_sha "$run_sha" --arg buildspec \
  "arn:aws:s3:::fixture-bucket/inputs/requests/$buildspec_sha.buildspec.yml" '
  .projectName=="isolab-experiment-launcher" and .timeoutInMinutesOverride==100
  and .buildspecOverride==$buildspec and (.idempotencyToken|test("^[a-f0-9]{64}$"))
  and any(.environmentVariablesOverride[];
    .name=="ISOLAB_RUN_REQUEST_SHA256" and .value==$run_sha)
' "$tmp/start.json" >/dev/null
[[ -f "$tmp/s3/fixture-bucket/inputs/requests/$run_sha.run-request" ]]
[[ -f "$tmp/s3/fixture-bucket/inputs/requests/$buildspec_sha.buildspec.yml" ]]
printf 'Portable launch pins the reviewed run-request and 100-minute terminal bound\n'
