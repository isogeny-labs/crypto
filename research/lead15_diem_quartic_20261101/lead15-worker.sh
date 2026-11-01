#!/usr/bin/env bash
# Frozen LEAD-15 Iso Lab worker. This is thin orchestration only: the native
# Rust binary performs every scientific computation. The wrapper binds the
# immutable build, enforces the scientific child deadline, and durably uploads
# complete or partial evidence before returning the child's status.
set -euo pipefail

usage() {
  echo "usage: lead15-worker <q> <source-manifest-sha256> <commit> <protocol-sha256> <binary-sha256> <image-digest> <scientific-seconds>" >&2
  exit 2
}

[[ $# == 7 ]] || usage
q=$1
manifest_sha=$2
commit=$3
protocol_sha=$4
binary_sha=$5
image_digest=$6
scientific_seconds=$7

case "$q" in 257|2053|32771|4194301) ;; *) usage ;; esac
[[ "$manifest_sha" =~ ^[0-9a-f]{64}$ ]] || usage
[[ "$commit" =~ ^[0-9a-f]{40}$ ]] || usage
[[ "$protocol_sha" =~ ^[0-9a-f]{64}$ ]] || usage
[[ "$binary_sha" =~ ^[0-9a-f]{64}$ ]] || usage
[[ "$image_digest" =~ ^sha256:[0-9a-f]{64}$ ]] || usage
[[ "$scientific_seconds" == 3600 ]] || usage

: "${ISOLAB_OUTPUT_DIR:?missing ISOLAB_OUTPUT_DIR}"
: "${ISOLAB_RESULTS_URI:?missing ISOLAB_RESULTS_URI}"
: "${ISOLAB_JOB_ID:?missing ISOLAB_JOB_ID}"
: "${POD_UID:?missing POD_UID}"
: "${NODE_NAME:?missing NODE_NAME}"
[[ "$ISOLAB_RESULTS_URI" =~ ^s3://isolab-results-104186811093-us-east-2/runs/${ISOLAB_JOB_ID}/${POD_UID}/$ ]] || {
  echo "unexpected ISOLAB_RESULTS_URI" >&2
  exit 45
}

binary=/opt/lead15/lead15_diem_quartic
build_info=/opt/lead15/build-info.json
mkdir -p "$ISOLAB_OUTPUT_DIR"
records="$ISOLAB_OUTPUT_DIR/records.jsonl"
started_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)
wrapper_sha=$(sha256sum "$0" | awk '{print $1}')

preflight_failure() {
  local reason=$1 upload_exit=0 receipt_upload_exit=0 finished
  finished=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  echo "LEAD-15 image preflight failed: $reason" >&2
  jq -n --arg results "$ISOLAB_RESULTS_URI" --arg job "$ISOLAB_JOB_ID" \
    --arg pod "$POD_UID" --arg node "$NODE_NAME" --arg started "$started_at" --arg finished "$finished" \
    '{schema:"lead15.worker-exit/v1",terminal_class:"infrastructure_failure",
      deadline_reached:false,exit_code:44,child_elapsed_seconds:0,
      payload:{path:"records.jsonl",sha256:"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        bytes:0,final_newline:false},started_at:$started,finished_at:$finished,
      results_uri:$results,job_name:$job,pod_uid:$pod,node_name:$node}' \
    > "$ISOLAB_OUTPUT_DIR/worker-exit.json"
  aws s3 cp "$ISOLAB_OUTPUT_DIR/worker-exit.json" "${ISOLAB_RESULTS_URI}worker-exit.json" --only-show-errors || upload_exit=$?
  jq -n --argjson uploaded "$([[ $upload_exit == 0 ]] && echo true || echo false)" \
    --argjson upload_exit "$upload_exit" --arg results "$ISOLAB_RESULTS_URI" \
    '{schema:"lead15.worker-upload/v1",uploaded:$uploaded,upload_exit:$upload_exit,
      terminal_class:"infrastructure_failure",child_exit_code:44,results_uri:$results,
      payload_sha256:"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      payload_bytes:0}' > "$ISOLAB_OUTPUT_DIR/worker-upload.json"
  aws s3 cp "$ISOLAB_OUTPUT_DIR/worker-upload.json" "${ISOLAB_RESULTS_URI}worker-upload.json" --only-show-errors \
    || receipt_upload_exit=$?
  if [[ $upload_exit != 0 || $receipt_upload_exit != 0 ]]; then
    exit 91
  fi
  exit 44
}

[[ -f "$binary" && -f "$build_info" ]] || preflight_failure "missing binary or build-info"
actual_binary_sha=$(sha256sum "$binary" | awk '{print $1}')
[[ "$actual_binary_sha" == "$binary_sha" ]] || preflight_failure "binary SHA-256 mismatch"
if ! jq -e \
  --arg source "$commit" \
  --arg binary "$binary_sha" \
  --arg worker "$wrapper_sha" \
  '.schema=="lead15.build-info/v1"
   and .source_revision==$source
   and .lead15_binary_sha256==$binary
   and .lead15_worker_sha256==$worker' \
  "$build_info" >/dev/null; then
  preflight_failure "build-info identity mismatch"
fi

cp -- "$build_info" "$ISOLAB_OUTPUT_DIR/build-info.json"
printf '%s  %s\n' "$actual_binary_sha" "$binary" > "$ISOLAB_OUTPUT_DIR/binary.sha256"
jq -n \
  --arg q "$q" \
  --arg manifest "$manifest_sha" \
  --arg commit "$commit" \
  --arg protocol "$protocol_sha" \
  --arg binary "$binary_sha" \
  --arg image "$image_digest" \
  --arg worker "$wrapper_sha" \
  --arg job "$ISOLAB_JOB_ID" \
  --arg pod "$POD_UID" \
  --arg node "$NODE_NAME" \
  --arg started "$started_at" \
  '{schema:"lead15.worker-start/v1",q:($q|tonumber),source_manifest_sha256:$manifest,
    instrument_commit:$commit,protocol_sha256:$protocol,binary_sha256:$binary,
    image_digest:$image,worker_sha256:$worker,
    scientific_deadline_seconds:3600,job_name:$job,pod_uid:$pod,node_name:$node,
    scientific_environment:{RAYON_NUM_THREADS:"1",OMP_NUM_THREADS:"1"},
    started_at:$started}' > "$ISOLAB_OUTPUT_DIR/worker-start.json"

{
  printf '{"schema":"lead15.runtime-provenance/v1","cpu_model":'
  awk -F: '/^(model name|Hardware)[[:space:]]*:/{sub(/^[[:space:]]+/,"",$2); print $2; exit}' /proc/cpuinfo | jq -Rs 'rtrimstr("\n")'
  printf ',"logical_cpus":%s,"memory_bytes":%s,"kernel":' "$(getconf _NPROCESSORS_ONLN)" "$(( $(awk '/^MemTotal:/{print $2}' /proc/meminfo) * 1024 ))"
  uname -srvm | jq -Rs 'rtrimstr("\n")'
  printf ',"job_name":%s,"pod_name":%s,"pod_uid":%s,"node_name":%s}\n' \
    "$(jq -Rn --arg v "$ISOLAB_JOB_ID" '$v')" \
    "$(jq -Rn --arg v "${HOSTNAME:-unknown}" '$v')" \
    "$(jq -Rn --arg v "$POD_UID" '$v')" \
    "$(jq -Rn --arg v "$NODE_NAME" '$v')"
} > "$ISOLAB_OUTPUT_DIR/runtime-provenance.json"

export LEAD15_COMMIT=$commit
export LEAD15_SOURCE_MANIFEST_SHA256=$manifest_sha
export LEAD15_PROTOCOL_SHA256=$protocol_sha
export LEAD15_BINARY_SHA256=$binary_sha
export LEAD15_IMAGE_DIGEST=$image_digest
export LEAD15_NODE_NAME=$NODE_NAME
export LEAD15_JOB_NAME=$ISOLAB_JOB_ID
export LEAD15_POD_NAME=${HOSTNAME:-unknown}
export LEAD15_POD_UID=$POD_UID
export RAYON_NUM_THREADS=1
export OMP_NUM_THREADS=1

child_pid=''
signal_name=''
escalation_pid=''
forward_signal() {
  signal_name=$1
  if [[ -n "$child_pid" ]]; then
    kill -"$1" -- "-$child_pid" 2>/dev/null || kill -"$1" "$child_pid" 2>/dev/null || true
    if [[ -z "$escalation_pid" ]]; then
      ( sleep 25; kill -KILL -- "-$child_pid" 2>/dev/null || true ) &
      escalation_pid=$!
    fi
  fi
}
trap 'forward_signal TERM' TERM
trap 'forward_signal INT' INT

set +e
child_started_epoch=$(date +%s)
setsid timeout --foreground --signal=TERM --kill-after=30 "${scientific_seconds}s" \
  "$binary" run-size "$q" "$records" \
  > "$ISOLAB_OUTPUT_DIR/stdout.log" 2> "$ISOLAB_OUTPUT_DIR/stderr.log" &
child_pid=$!
wait "$child_pid"
child_exit=$?
if [[ -n "$signal_name" ]] && kill -0 "$child_pid" 2>/dev/null; then
  wait "$child_pid"
  child_exit=$?
fi
if [[ -n "$escalation_pid" ]]; then
  kill "$escalation_pid" 2>/dev/null || true
  wait "$escalation_pid" 2>/dev/null || true
fi
child_pid=''
set -e
finished_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)
child_elapsed_seconds=$(( $(date +%s) - child_started_epoch ))

terminal_class=scientific_failure
deadline_reached=false
if [[ -n "$signal_name" ]]; then
  terminal_class=infrastructure_failure
elif [[ $child_exit == 0 ]]; then
  terminal_class=completed
elif [[ $child_exit == 124 || ( $child_exit == 137 && $child_elapsed_seconds -ge $((scientific_seconds - 1)) ) ]]; then
  terminal_class=deadline
  deadline_reached=true
elif [[ $child_exit == 130 || $child_exit == 137 || $child_exit == 143 ]]; then
  terminal_class=infrastructure_failure
fi

payload_bytes=0
payload_sha=$(printf '' | sha256sum | awk '{print $1}')
final_newline=false
if [[ -f "$records" ]]; then
  payload_bytes=$(stat -c %s "$records")
  payload_sha=$(sha256sum "$records" | awk '{print $1}')
  if [[ $payload_bytes -gt 0 && $(tail -c 1 "$records" | od -An -tuC | tr -d '[:space:]') == 10 ]]; then
    final_newline=true
  fi
fi
jq -n \
  --arg terminal "$terminal_class" \
  --argjson deadline "$deadline_reached" \
  --argjson exit "$child_exit" \
  --argjson elapsed "$child_elapsed_seconds" \
  --arg payload_sha "$payload_sha" \
  --argjson payload_bytes "$payload_bytes" \
  --argjson final_newline "$final_newline" \
  --arg started "$started_at" \
  --arg finished "$finished_at" \
  --arg results "$ISOLAB_RESULTS_URI" \
  --arg job "$ISOLAB_JOB_ID" \
  --arg pod "$POD_UID" \
  --arg node "$NODE_NAME" \
  '{schema:"lead15.worker-exit/v1",terminal_class:$terminal,
    deadline_reached:$deadline,exit_code:$exit,child_elapsed_seconds:$elapsed,
    payload:{path:"records.jsonl",
    sha256:$payload_sha,bytes:$payload_bytes,final_newline:$final_newline},
    started_at:$started,finished_at:$finished,results_uri:$results,
    job_name:$job,pod_uid:$pod,node_name:$node}' > "$ISOLAB_OUTPUT_DIR/worker-exit.json"

(
  cd "$ISOLAB_OUTPUT_DIR"
  find . -type f ! -name SHA256SUMS -print0 | sort -z |
    xargs -0 sha256sum
) > "$ISOLAB_OUTPUT_DIR/SHA256SUMS"

upload_exit=0
aws s3 cp "$ISOLAB_OUTPUT_DIR/" "$ISOLAB_RESULTS_URI" --recursive --only-show-errors || upload_exit=$?
jq -n \
  --argjson uploaded "$([[ $upload_exit == 0 ]] && echo true || echo false)" \
  --argjson upload_exit "$upload_exit" \
  --arg terminal "$terminal_class" \
  --argjson child_exit "$child_exit" \
  --arg results "$ISOLAB_RESULTS_URI" \
  --arg payload_sha "$payload_sha" \
  --argjson payload_bytes "$payload_bytes" \
  '{schema:"lead15.worker-upload/v1",uploaded:$uploaded,upload_exit:$upload_exit,
    terminal_class:$terminal,child_exit_code:$child_exit,results_uri:$results,
    payload_sha256:$payload_sha,payload_bytes:$payload_bytes}' \
  > "$ISOLAB_OUTPUT_DIR/worker-upload.json"
receipt_upload_exit=0
aws s3 cp "$ISOLAB_OUTPUT_DIR/worker-upload.json" "${ISOLAB_RESULTS_URI}worker-upload.json" --only-show-errors || receipt_upload_exit=$?
if [[ $upload_exit == 0 && $receipt_upload_exit != 0 ]]; then
  upload_exit=$receipt_upload_exit
fi

if [[ $upload_exit != 0 ]]; then
  echo "LEAD-15 artifact upload failed with exit $upload_exit; scientific child exit was $child_exit" >&2
  exit 91
fi
exit "$child_exit"
