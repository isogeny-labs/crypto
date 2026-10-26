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
