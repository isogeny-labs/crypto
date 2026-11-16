#!/usr/bin/env bash
# Administrative Runpod client. Credentials are paths, never copied to the pod.
set -euo pipefail
test "$#" -eq 5 || { echo 'usage: launch.sh SOURCE CHECKPOINT_RECEIPT UPLOAD_TOKEN_JSON CURL_AUTH_CONFIG PRIVATE_OUTPUT_DIR' >&2; exit 2; }
source_revision=$1 checkpoint=$2 upload=$3 auth=$4 out=$5
[[ $source_revision =~ ^[0-9a-f]{40}$ ]] || exit 2
umask 077
mkdir -p "$out"
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
protocol_hash=$(sha256sum "$here/PROTOCOL.json" | cut -d' ' -f1)
closure_hash=$(sha256sum "$here/SOURCES_v3.sha256" | cut -d' ' -f1)
jq -e --arg source "$source_revision" --arg protocol "$protocol_hash" --arg closure "$closure_hash" \
  '.gate_exit==0 and .source_revision==$source and .protocol_sha256==$protocol and .source_manifest_sha256==$closure and (.checkpoint_state|contains("s3://setec-astronomy-artifacts/"))' "$checkpoint" >/dev/null
test "$(git rev-parse HEAD)" = "$source_revision"
test -z "$(git status --porcelain)"
sha256sum -c "$here/SOURCES_v3.sha256" > "$out/source-prelaunch-check.log"
curl --config "$auth" --fail --silent --show-error --max-time 20 --header 'Content-Type: application/json' \
  --data '{"query":"query { myself { clientBalance currentSpendPerHr } }"}' https://api.runpod.io/graphql > "$out/balance.private.json"
jq -e '.data.myself|.clientBalance>=1' "$out/balance.private.json" >/dev/null || { echo 'Insufficient live credit for the combined one-dollar task allowance' >&2; exit 3; }
name="trace-incidence-20261116-${source_revision:0:10}"
curl --config "$auth" --fail --silent --show-error --max-time 20 https://rest.runpod.io/v1/pods > "$out/pods-before.private.json"
jq -e --arg name "$name" 'all(.[];.name!=$name)' "$out/pods-before.private.json" >/dev/null || { echo 'Existing matching request found; reconcile it instead of creating a duplicate' >&2; exit 4; }
nonce=$(openssl rand -hex 24)
bootstrap_hash=$(sha256sum "$here/bootstrap-v2.sh" | cut -d' ' -f1)
start_command='set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
mkdir -p "/results/$TRACE_RESULT_NONCE/raw"
apt-get update > "/results/$TRACE_RESULT_NONCE/raw/initial-install.log" 2>&1
apt-get install -y --no-install-recommends ca-certificates curl busybox jq >> "/results/$TRACE_RESULT_NONCE/raw/initial-install.log" 2>&1
busybox httpd -p 8000 -h /results
curl --fail --silent --show-error "https://raw.githubusercontent.com/isogeny-labs/crypto/$TRACE_SOURCE_REVISION/research/trace_incidence_20261116/runpod/bootstrap-v2.sh" -o /tmp/trace-bootstrap.sh
printf "%s  /tmp/trace-bootstrap.sh\n" "$TRACE_BOOTSTRAP_SHA256" | sha256sum -c -
timeout -k 15 1500 bash /tmp/trace-bootstrap.sh || true
tail -f /dev/null'
jq -n --arg name "$name" --arg source "$source_revision" --arg nonce "$nonce" --arg bootstrap "$bootstrap_hash" --arg command "$start_command" \
  '{name:$name,computeType:"CPU",cloudType:"SECURE",cpuFlavorIds:["cpu5c"],cpuFlavorPriority:"custom",vcpuCount:8,containerDiskInGb:20,volumeInGb:0,interruptible:false,imageName:"debian:trixie-slim@sha256:7378260c0a098d27ac206225db5e7d92b8c7c84ea7afccc86eeb513756bbd392",ports:["8000/http"],dockerEntrypoint:["bash","-lc"],dockerStartCmd:[$command],env:{TRACE_SOURCE_REVISION:$source,TRACE_RESULT_NONCE:$nonce,TRACE_BOOTSTRAP_SHA256:$bootstrap}}' > "$out/request.private.json"
created=$(date +%s)
printf '%s\n' "$((created+1800))" > "$out/original-deadline-epoch.txt"
status=$(curl --config "$auth" --silent --show-error --max-time 45 --header 'Content-Type: application/json' --data-binary @"$out/request.private.json" --output "$out/create.private.json" --write-out '%{http_code}' https://rest.runpod.io/v1/pods) || status=000
if [[ $status != 200 && $status != 201 ]]; then
  curl --config "$auth" --fail --silent --show-error --max-time 20 https://rest.runpod.io/v1/pods > "$out/reconcile.private.json"
  jq --arg name "$name" '[.[]|select(.name==$name)]|if length==1 then .[0] else empty end' "$out/reconcile.private.json" > "$out/create.private.json"
fi
pod_id=$(jq -er '.id' "$out/create.private.json") || { echo 'No unique created pod; do not retry automatically' >&2; exit 5; }
printf '%s\n' "$pod_id" > "$out/pod-id.txt"
trap 'curl --config "$auth" --silent --show-error --max-time 20 -X DELETE "https://rest.runpod.io/v1/pods/$pod_id" > "$out/launcher-error-delete.private.txt" || true' EXIT
if ! jq -e '(.costPerHr|tonumber)<=0.5 and .vcpuCount==8 and .memoryInGb<=16' "$out/create.private.json" >/dev/null; then
  curl --config "$auth" --silent --show-error --max-time 20 -X DELETE "https://rest.runpod.io/v1/pods/$pod_id" > "$out/admission-delete.private.json"
  echo 'Allocated pod exceeded the frozen price/resource ceiling and was deleted' >&2
  exit 6
fi
jq --arg source "$source_revision" --argjson created "$created" \
  '{schema:"trace-runpod-deployment/v1",source_revision:$source,pod_id:.id,pod_name:.name,image:.imageName,price_per_hour_usd:.costPerHr,vcpu_count:.vcpuCount,memory_gb:.memoryInGb,created_epoch:$created,deadline_epoch:($created+1800),cost_ceiling_usd:1}' "$out/create.private.json" > "$out/deployment.json"
printf '%s\n' "$nonce" > "$out/result-namespace.private.txt"
if [[ $(readlink -f "$checkpoint") != $(readlink -f "$out/checkpoint-receipt.json") ]]; then
  cp "$checkpoint" "$out/checkpoint-receipt.json"
fi
printf 'Pod %s allocated; starting bounded collection and cleanup supervisor.\n' "$pod_id"
exec bash "$here/supervise-v2.sh" "$out" "$auth" "$upload"
