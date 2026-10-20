#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
valid='{"schema":"isolab.eks.request/v1","job":{"job_name":"contract-test"}}'
printf '%s\n' "$valid" > "$tmp/request.json"
"$here/validate-request" "$tmp/request.json" | jq -e '.job.instance_type=="c8a.2xlarge" and .job.seconds==900 and .job.memory=="4Gi"' >/dev/null
cases=(
  '.schema="unknown"'
  '.job.job_name="../../escape"'
  '.job.job_name="-option"'
  '.job.seconds=0'
  '.job.seconds=3601'
  '.job.seconds=1.5'
  '.job.seconds=false'
  '.job.seconds=null'
  '.job.memory="0Gi"'
  '.job.instance_type="c8a.2xlarge;true"'
  '.job.image="unapproved:latest"'
  '.job.retries=1'
  '.job.spec="not an object"'
  '.provenance={unknown:"x"}'
  '.provenance={harness:42}'
  '.unknown=true'
)
for change in "${cases[@]}"; do
  printf '%s\n' "$valid" | jq "$change" > "$tmp/request.json"
  if "$here/validate-request" "$tmp/request.json" > /dev/null 2>&1; then
    printf 'Incorrectly accepted: %s\n' "$change" >&2
    exit 1
  fi
done
printf 'Contract defaults and %s rejection cases passed\n' "${#cases[@]}"
image=$(jq -r .image_uri "$here/../image.json")
revision=$(printf 'a%.0s' {1..40})
binary=$(printf 'b%.0s' {1..64})
printf '%s\n' "$valid" | jq --arg image "$image" --arg revision "$revision" --arg binary "$binary" \
  '.job.build={image_uri:$image,source_revision:$revision,ecbench_sha256:$binary}
   | .provenance={source_revision:$revision}' > "$tmp/build-request.json"
"$here/validate-request" "$tmp/build-request.json" >/dev/null
cases=(
  '.job.build=null'
  '.job.build={}'
  '.job.build.image_uri="busybox:latest"'
  '.job.build.image_uri|=sub("104186811093";"123456789012")'
  '.job.build.image_uri|=sub("/isolab/tasks";"/other/tasks")'
  '.job.build.source_revision="main"'
  '.job.build.ecbench_sha256="wrong"'
  '.job.build.image_uri+="\n"'
  '.job.build.source_revision+="\n" | .provenance.source_revision=.job.build.source_revision'
  '.job.build.ecbench_sha256+="\n"'
  '.job.build.image_uri|=sub("@sha256:";"@sha256:extra@sha256:")'
  '.job.build.command="true"'
  'del(.job.build.ecbench_sha256)'
  'del(.provenance)'
  '.provenance.source_revision="different"'
)
for change in "${cases[@]}"; do
  jq "$change" "$tmp/build-request.json" > "$tmp/request.json"
  if "$here/validate-request" "$tmp/request.json" >/dev/null 2>&1; then
    printf 'Incorrectly accepted build: %s\n' "$change" >&2
    exit 1
  fi
done
printf 'Commit-build request and %s rejection cases passed\n' "${#cases[@]}"
