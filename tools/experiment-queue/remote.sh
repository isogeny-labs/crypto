#!/usr/bin/env bash
# Trusted CodeBuild controller, embedded in the StartBuild buildspec by Actions.
# Workload argv and its verifier execute only in the Isolab pod.
set -euo pipefail
export PATH="$CODEBUILD_SRC_DIR/bin:$PATH"
: "${ISOLAB_BUCKET:?}" "${ISOLAB_CLUSTER:?}" "${AWS_REGION:?}" "${ISOLAB_REQUEST_KEY:?}"
[[ $ISOLAB_REQUEST_KEY =~ ^inputs/requests/[a-f0-9]{64}\.json$ ]]
expected=${ISOLAB_REQUEST_KEY##*/}; expected=${expected%.json}
receipts=$(mktemp -d)
aws s3 cp "s3://$ISOLAB_BUCKET/$ISOLAB_REQUEST_KEY" "$receipts/request.json" --only-show-errors
[[ $(sha256sum "$receipts/request.json" | cut -d' ' -f1) == "$expected" ]]
jq -e '.schema=="crypto.isolab-dispatch/v1" and (.candidate.key|test("^[a-f0-9]{64}$"))' "$receipts/request.json" >/dev/null
job=$(jq -r .candidate.job_name "$receipts/request.json")
[[ $job =~ ^q-[a-z0-9_-]+-[a-f0-9]{20}$ ]]
key=$(jq -r .candidate.key "$receipts/request.json")
duration=$(jq -r .candidate.entry.seconds "$receipts/request.json")
[[ $duration =~ ^[1-9][0-9]*$ && $duration -le 3600 ]]
uri="s3://$ISOLAB_BUCKET/runs/$job/launcher/queue/"
state=launcher_failed job_uid='' pod_uid='' raw_uri=''
finish() {
  rc=$?
  trap - EXIT
  set +e
  jq -n --arg key "$key" --arg job "$job" --arg uid "$job_uid" --arg pod "$pod_uid" \
    --arg state "$state" --arg raw "$raw_uri" --arg receipt "$uri" --arg sha "$expected" \
    --arg build "${CODEBUILD_BUILD_ID:-}" --argjson rc "$rc" \
    '{schema:"crypto.isolab-queue-result/v1",key:$key,job_name:$job,job_uid:$uid,pod_uid:$pod,
      state:$state,exit_code:$rc,results_uri:$raw,receipt_uri:$receipt,request_sha256:$sha,build_id:$build,
      wall_valid:false}' > "$receipts/result.json"
  aws s3 cp "$receipts/" "$uri" --recursive --only-show-errors || exit 91
  exit "$rc"
}
trap finish EXIT
export KUBECONFIG="$(mktemp)" ISOLAB_KUBE_CONTEXT=isolab
aws eks update-kubeconfig --name "$ISOLAB_CLUSTER" --region "$AWS_REGION" --alias isolab --kubeconfig "$KUBECONFIG" >/dev/null
"$CODEBUILD_SRC_DIR/remote/check-ready" > "$receipts/readiness.json"
isolab-eks-mcp call isolab_k8s_overview '{}' > "$receipts/overview.json"
kind=$(jq -r .candidate.entry.kind "$receipts/request.json")
if [[ $kind == command ]]; then
  jq -r .worker_base64 "$receipts/request.json" | base64 -d > "$receipts/worker.sh"
  [[ $(sha256sum "$receipts/worker.sh" | cut -d' ' -f1) == "$(jq -r .worker_sha256 "$receipts/request.json")" ]]
  args=$(jq -nc --slurpfile r "$receipts/request.json" --rawfile worker "$receipts/worker.sh" \
    '$r[0].candidate as $c | {job_name:$c.job_name,image:$c.entry.image.uri,cpu:$c.entry.cpu,
      memory:$c.entry.memory,seconds:$c.entry.seconds,retries:0,
      command:["bash","-c",$worker,"isolab-queued-command",($c.entry|tojson)]}')
  tool=isolab_k8s_run
else
  [[ $kind == benchmark ]]
  args=$(jq -nc --slurpfile r "$receipts/request.json" '$r[0].candidate as $c |
    {job_name:$c.job_name,spec:$c.spec,seconds:$c.entry.seconds,memory:$c.entry.memory,
      build:{image_uri:$c.entry.image.uri,source_revision:$c.entry.source_commit,
        ecbench_sha256:$c.entry.image.ecbench_sha256}}')
  tool=isolab_k8s_benchmark
fi
state=submission_unknown
if ! isolab-eks-mcp call "$tool" "$args" > "$receipts/submission.json" 2> "$receipts/submission.stderr"; then
  isolab-eks-mcp call isolab_k8s_status "$(jq -nc --arg job "$job" '{job_name:$job}')" > "$receipts/status-after-error.json" 2>&1 || true
  exit 20
fi
job_uid=$(jq -er .job.uid "$receipts/submission.json")
status_args=$(jq -nc --arg job "$job" '{job_name:$job}')
deadline=$((SECONDS + duration + 180))
while (( SECONDS < deadline )); do
  isolab-eks-mcp call isolab_k8s_status "$status_args" > "$receipts/status.json"
  [[ $(jq -r .job.uid "$receipts/status.json") == "$job_uid" ]] || { state=identity_mismatch; exit 21; }
  state=$(jq -r .job.state "$receipts/status.json")
  [[ $state == succeeded || $state == failed ]] && break
  sleep 30
done
isolab-eks-mcp call isolab_k8s_logs "$status_args" > "$receipts/logs.json" 2> "$receipts/logs.stderr" || true
[[ $state == succeeded || $state == failed ]] || { state=launcher_timed_out; exit 22; }
pod_uid=$(jq -er '[.pods[] | select(any(.status.containerStatuses[]?; .name=="task" and .state.terminated!=null)) | .uid] | if length==1 then .[0] else error("expected exactly one terminated attempt") end' "$receipts/status.json")
[[ $pod_uid =~ ^[a-f0-9-]{36}$ ]]
raw_uri="s3://$ISOLAB_BUCKET/runs/$job/$pod_uid/"
jq -e --arg uid "$pod_uid" --arg image "$(jq -r .candidate.entry.image.uri "$receipts/request.json")" '
  .pods[] | select(.uid==$uid) | .status.containerStatuses[] | select(.name=="task") |
  (.image==$image) and (.imageID|endswith($image|split("@")[-1]))' "$receipts/status.json" >/dev/null || { state=image_mismatch; exit 24; }
# Preserve failed attempts too. A failed pod never becomes scientific success.
aws s3 cp "${raw_uri}SHA256SUMS" "$receipts/SHA256SUMS" --only-show-errors || { state=missing_outputs; exit 25; }
pod_state=$state
state=verification_failed
mkdir "$receipts/artifacts"
if [[ $kind == benchmark ]]; then
  exporter_uri=$(jq -r .exporter_uri "$receipts/request.json")
  [[ $exporter_uri == s3://"$ISOLAB_BUCKET"/inputs/requests/*.queue-bin ]]
  aws s3 cp "$exporter_uri" /tmp/isolab-queue-exporter --only-show-errors
  [[ $(sha256sum /tmp/isolab-queue-exporter | cut -d' ' -f1) == "$(jq -r .exporter_sha256 "$receipts/request.json")" ]]
  chmod 700 /tmp/isolab-queue-exporter
  /tmp/isolab-queue-exporter fetch-verified --uri "$raw_uri" --out "$receipts/artifacts"
  export_observations() {
    /tmp/isolab-queue-exporter export-ecbench --request "$receipts/request.json" \
      --session "$receipts/artifacts/session" --out "$receipts/artifacts/export.json" --admitted="$1"
    (cd "$receipts/artifacts" && sha256sum export.json) > "$receipts/export.sha256"
  }
  if [[ $pod_state == failed ]]; then state=failed; export_observations false; exit 23; fi
  # Existing native build/replay/L2/A-A admission; thresholds are unchanged.
  jq '{job:{spec:.candidate.spec,build:(.candidate.entry.image | {image_uri:.uri,source_revision:.source_commit,ecbench_sha256:.ecbench_sha256})}}' "$receipts/request.json" > "$receipts/build-request.json"
  "$CODEBUILD_SRC_DIR/remote/verify-build" "$receipts/build-request.json" "$receipts/status.json" "$pod_uid" "$receipts/artifacts"
  jq -e '.accepted and .executions>0 and (.below_l2|length)==0 and (.invalid_outcomes|length)==0' "$receipts/artifacts/l2-gate.json" >/dev/null
  jq -e '.ok and .regraded and .records_bound and .verified_records==.records and (.replays|length)>0 and all(.replays[];.reproduced)' "$receipts/artifacts/audit.json" >/dev/null
  jq -e '.wall.status=="admitted" and .wall.aa_pairs>=5 and .wall.aa_ci95!=null' "$receipts/artifacts/control-comparison.json" >/dev/null
  export_observations true
else
  aws s3 cp "${raw_uri}queue-job.json" "$receipts/artifacts/queue-job.json" --only-show-errors
  expected_job=$(awk 'substr($0,67)=="queue-job.json" || substr($0,67)=="./queue-job.json" {print substr($0,1,64)}' "$receipts/SHA256SUMS")
  [[ $expected_job =~ ^[a-f0-9]{64}$ && $(sha256sum "$receipts/artifacts/queue-job.json" | cut -d' ' -f1) == "$expected_job" ]]
  while IFS= read -r file; do
    [[ $file != /* && /$file/ != */../* && /$file/ != */./* ]]
    mkdir -p -- "$(dirname -- "$receipts/artifacts/$file")"
    size=$(aws s3api head-object --bucket "$ISOLAB_BUCKET" --key "runs/$job/$pod_uid/$file" --query ContentLength --output text 2>/dev/null) || { [[ $pod_state == failed ]] && continue; exit 25; }
    [[ $size =~ ^[0-9]+$ && $size -le 16777216 ]] || { state=output_too_large; exit 25; }
    aws s3 cp "${raw_uri}$file" "$receipts/artifacts/$file" --only-show-errors
    expected_file=$(awk -v file="$file" 'substr($0,67)==file || substr($0,67)=="./"file {print substr($0,1,64)}' "$receipts/SHA256SUMS")
    [[ $expected_file =~ ^[a-f0-9]{64}$ && $(sha256sum "$receipts/artifacts/$file" | cut -d' ' -f1) == "$expected_file" ]]
  done < <(jq -r '.candidate.entry.outputs[]' "$receipts/request.json")
  if [[ $pod_state == failed ]]; then state=failed; exit 23; fi
  jq -e --arg source "$(jq -r .candidate.entry.source_commit "$receipts/request.json")" --arg pod "$pod_uid" \
    '.schema=="crypto.isolab-worker-result/v1" and .source_revision==$source and .pod_uid==$pod and
      .command_exit==0 and .verify_exit==0 and .inputs_and_runtime_verified' "$receipts/artifacts/queue-job.json" >/dev/null
fi
state=succeeded
