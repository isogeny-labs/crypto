#!/usr/bin/env bash
# Live infrastructure check: uses a Spot worker and writes small S3 artifacts.
set -euo pipefail
: "${KUBECONFIG:?Set the dedicated Iso Lab kubeconfig}"
: "${ISOLAB_KUBE_CONTEXT:?Set the explicit Iso Lab context}"
: "${ISOLAB_MCP_BIN:?Set the installed isolab-eks-mcp executable}"
[[ $# == 1 && "$1" == /* ]] || { echo 'Usage: smoke.sh ABSOLUTE_EVIDENCE_DIRECTORY' >&2; exit 2; }
mkdir -p "$1"
out=$1
name="mcp-plan-$(date -u +%Y%m%d-%H%M%S)"
rpc() {
  local tool=$1 args=$2 file=$3
  jq -nc --arg tool "$tool" --argjson args "$args" '
    {jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:"2025-11-25",capabilities:{},clientInfo:{name:"isolab-live-smoke",version:"1"}}},
    {jsonrpc:"2.0",method:"notifications/initialized"},
    {jsonrpc:"2.0",id:2,method:"tools/call",params:{name:$tool,arguments:$args}}' |
    "$ISOLAB_MCP_BIN" > "$out/$file.jsonl"
}
ok() { jq -se 'map(select(.id == 2)) | length == 1 and .[0].result.isError == false' "$out/$1.jsonl" >/dev/null; }
wait_job() {
  local job=$1 file=$2 end=$((SECONDS+600)) state=''
  while ((SECONDS < end)); do
    rpc isolab_k8s_status "$(jq -nc --arg name "$job" '{job_name:$name}')" "$file"
    ok "$file"
    state=$(jq -r 'select(.id == 2).result.structuredContent.job.state' "$out/$file.jsonl")
    if [[ "$state" == succeeded || "$state" == failed ]]; then printf '%s: %s\n' "$job" "$state"; return; fi
    sleep 10
  done
  echo "Timed out waiting for $job; inspect the retained Job" >&2
  exit 1
}
rpc isolab_k8s_overview '{}' overview
ok overview
args=$(jq -nc --arg name "$name" '{job_name:$name,cpu:2,memory:"1Gi",seconds:600,command:["/bin/sh","-ec","mkdir -p \"$ISOLAB_OUTPUT_DIR\"; ecbench plan --spec /opt/isolab/specs/smoke-prime.json --json > \"$ISOLAB_OUTPUT_DIR/plan.json\"; sha256sum \"$ISOLAB_OUTPUT_DIR/plan.json\"; aws s3 cp \"$ISOLAB_OUTPUT_DIR/\" \"$ISOLAB_RESULTS_URI\" --recursive"]}')
rpc isolab_k8s_run "$args" submitted
ok submitted
printf 'Submitted %s through MCP stdio\n' "$name"
rpc isolab_k8s_run "$args" duplicate
jq -se 'map(select(.id == 2))[0].result | .isError == true and (.content[0].text | contains("AlreadyExists"))' "$out/duplicate.jsonl" >/dev/null
wait_job "$name" completed
jq -e 'select(.id == 2).result.structuredContent.job.state == "succeeded"' "$out/completed.jsonl" >/dev/null
rpc isolab_k8s_logs "$(jq -nc --arg name "$name" '{job_name:$name}')" logs
ok logs
rpc isolab_k8s_results "$(jq -nc --arg name "$name" '{job_name:$name}')" results
ok results
key=$(jq -r 'select(.id == 2).result.structuredContent.objects[] | select(.Key | endswith("/plan.json")) | .Key' "$out/results.jsonl")
[[ -n "$key" && "$key" != *$'\n'* ]]
aws s3 cp "s3://isolab-results-104186811093-us-east-2/$key" "$out/plan.json" --region us-east-2 --only-show-errors
[[ $(sha256sum "$out/plan.json" | cut -d' ' -f1) == e37b221bfe4d4460099c3129dc42bc63cd217eff31cf38cee928c5da6b5a7e84 ]]
rpc isolab_k8s_run "$(jq -nc --arg name "$name-fail" '{job_name:$name,cpu:2,memory:"512Mi",seconds:120,command:["/bin/sh","-c","echo expected-exit-7; exit 7"]}')" failure-submitted
ok failure-submitted
wait_job "$name-fail" failure-status
jq -e 'select(.id == 2).result.structuredContent.job.state == "failed"' "$out/failure-status.jsonl" >/dev/null
jq -e 'select(.id == 2).result.structuredContent.pods | any(.[]; any(.status.containerStatuses[]?; .state.terminated.exitCode == 7))' "$out/failure-status.jsonl" >/dev/null
rpc isolab_k8s_run "$(jq -nc --arg name "$name-cancel" '{job_name:$name,cpu:2,memory:"512Mi",seconds:120,command:["/bin/sleep","90"]}')" cancel-submitted
ok cancel-submitted
rpc isolab_k8s_cancel "$(jq -nc --arg name "$name-cancel" '{job_name:$name}')" cancelled
ok cancelled
printf 'Verified success, duplicate rejection, failure status, logs, S3 result hash and cancellation. Evidence: %s\n' "$out"
