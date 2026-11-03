#!/usr/bin/env bash
# Identity regression tests; no cryptographic computation or timing.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
mkdir "$tmp/artifacts"
image=$(jq -r .image_uri "$here/../image.json")
revision=$(printf 'a%.0s' {1..40})
binary=$(printf 'b%.0s' {1..64})
jq -n --arg image "$image" --arg revision "$revision" --arg binary "$binary" \
  '{job:{build:{image_uri:$image,source_revision:$revision,ecbench_sha256:$binary},spec:{schema:"fixture"}}}' > "$tmp/request.json"
jq -n --arg image "$image" \
  '{pods:[{uid:"pod-1",status:{containerStatuses:[{name:"task",imageID:$image,state:{terminated:{exitCode:0}}}]}}]}' > "$tmp/status.json"
jq '.job.build' "$tmp/request.json" > "$tmp/artifacts/build-info.json"
jq '.job.spec' "$tmp/request.json" > "$tmp/artifacts/input-spec.json"
printf '%s  /usr/local/bin/ecbench\n' "$binary" > "$tmp/artifacts/binary.sha256"
"$here/verify-build" "$tmp/request.json" "$tmp/status.json" pod-1 "$tmp/artifacts"
reject() {
  if "$here/verify-build" "$tmp/request.json" "$tmp/status.json" "$1" "$tmp/artifacts" >/dev/null 2>&1; then
    echo "Accepted mismatched $2" >&2
    exit 1
  fi
}
reject other-pod 'pod UID'
cp "$tmp/status.json" "$tmp/original-status.json"
jq '.pods[0].status.containerStatuses[0].imageID="wrong@sha256:deadbeef"' "$tmp/original-status.json" > "$tmp/status.json"
reject pod-1 'running image'
cp "$tmp/original-status.json" "$tmp/status.json"
jq '.pods[0].status.containerStatuses[0].state.terminated.exitCode=42' "$tmp/original-status.json" > "$tmp/status.json"
reject pod-1 'container exit'
cp "$tmp/original-status.json" "$tmp/status.json"
jq '.job.build | .source_revision="wrong"' "$tmp/request.json" > "$tmp/artifacts/build-info.json"
reject pod-1 'built commit'
jq '.job.build' "$tmp/request.json" > "$tmp/artifacts/build-info.json"
printf 'wrong  /usr/local/bin/ecbench\n' > "$tmp/artifacts/binary.sha256"
reject pod-1 'actual binary hash'
printf '%s  /usr/local/bin/ecbench\n' "$binary" > "$tmp/artifacts/binary.sha256"
printf '{"schema":"different"}\n' > "$tmp/artifacts/input-spec.json"
reject pod-1 'measured spec'
printf '{"job":{"job_name":"legacy"}}\n' > "$tmp/legacy.json"
"$here/verify-build" "$tmp/legacy.json" "$tmp/status.json" pod-1 "$tmp/artifacts"
echo 'Build identity: valid and legacy requests accepted; six mismatches rejected'
