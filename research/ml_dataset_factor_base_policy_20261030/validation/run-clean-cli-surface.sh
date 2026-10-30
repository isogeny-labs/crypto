#!/bin/bash
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: run-clean-cli-surface.sh BUILD BUILD_RECEIPT_SHA OUTPUT' >&2; exit 2; }
build=$(realpath "$1")
receipt_sha=$2
output=$3
[[ ! -e "$output" && ! -L "$output" ]]
mkdir "$output"
output=$(realpath "$output")
start=$(date -u +%FT%TZ)
stage=admission
count=0
source=unknown
runner_sha=$(sha256sum "$0" | cut -d' ' -f1)
finish() {
  result=$?
  trap - EXIT
  jq -n --arg start "$start" --arg finish "$(date -u +%FT%TZ)" --arg stage "$stage" \
    --arg source "$source" --arg build_receipt "$receipt_sha" --arg runner "$runner_sha" \
    --argjson count "$count" --argjson code "$result" \
    '{schema:"ml-dataset.cli-surface-validation/v1",started_utc:$start,finished_utc:$finish,
      stage:$stage,source_commit:$source,build_receipt_sha256:$build_receipt,runner_sha256:$runner,
      observed_commands:$count,exit_code:$code,scope:"CLI surface only",solver_calls:0,
      scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$output/receipt.json"
  cat "$output/receipt.json"
  exit "$result"
}
trap finish EXIT
printf '%s  %s\n' "$receipt_sha" "$build/receipt.json" | sha256sum -c -
jq -e '.exit_code==0 and .command_exit_code==0 and .source_preflight_matches and .source_readback_matches' "$build/receipt.json" >/dev/null
source=$(jq -er .source_commit "$build/receipt.json")
(cd "$build"; sha256sum -c binaries.sha256)
run() {
  local id=$1
  shift
  stage=$id
  local code
  set +e
  timeout --signal=TERM --kill-after=5s 30s "$@" > "$output/$id.stdout" 2> "$output/$id.stderr"
  code=$?
  set -e
  count=$((count+1))
  jq -n --arg id "$id" --argjson code "$code" \
    --argjson argv "$(printf '%s\n' "$@" | jq -Rsc 'split("\n")|map(select(length>0))')" \
    --arg stdout "$(sha256sum "$output/$id.stdout" | cut -d' ' -f1)" \
    --arg stderr "$(sha256sum "$output/$id.stderr" | cut -d' ' -f1)" \
    '{id:$id,argv:$argv,exit_code:$code,stdout_sha256:$stdout,stderr_sha256:$stderr,timeout_seconds:30}' > "$output/$id.json"
  [[ "$code" == 0 ]] || exit "$code"
  [[ -s "$output/$id.stdout" && ! -s "$output/$id.stderr" ]]
}
run dataset-help "$build/crypto" dataset --help
for name in start budget plan generate preflight verify status report publish capabilities export restore reconcile; do
  rg -q "^  $name[[:space:]]" "$output/dataset-help.stdout"
done
run startup-help "$build/crypto" dataset start --help
rg -q -- '--resume' "$output/startup-help.stdout"
run capabilities "$build/crypto" dataset capabilities
jq -e '.schema=="ml-dataset.capabilities/v1" and
  (.solvers|map(.id)|sort)==(["enumerate","bsgs","sat","weil","f4","f5","f6","root","pkm","double-large-prime"]|sort) and
  .factor_base_policy_plans.schemas==["crypto.ml-dataset.plan/v1","crypto.ml-dataset.plan/v2"]' "$output/capabilities.stdout" >/dev/null
run controller-help "$build/dataset_startup_cli_fixture" --help
rg -q -- '--policy-acceptance' "$output/controller-help.stdout"
stage=binary_readback
(cd "$build"; sha256sum -c binaries.sha256)
[[ $(sha256sum "$build/receipt.json" | cut -d' ' -f1) == "$receipt_sha" ]]
[[ $(sha256sum "$0" | cut -d' ' -f1) == "$runner_sha" ]]
stage=completed
