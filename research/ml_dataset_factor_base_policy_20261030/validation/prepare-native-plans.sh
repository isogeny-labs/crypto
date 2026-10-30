#!/bin/bash
# Thin fixed native CLI invocations; all plan and scope hashing is native.
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: prepare-native-plans.sh BUILD BUILD_RECEIPT_SHA OUTPUT' >&2; exit 2; }
build=$(realpath "$1")
build_sha=$2
output=$3
[[ ! -e "$output" && ! -L "$output" ]]
mkdir "$output"
output=$(realpath "$output")
mkdir "$output/inputs" "$output/plans" "$output/commands"
scripts=$(cd -- "$(dirname -- "$0")" && pwd)
runner_sha=$(sha256sum "$0" | cut -d' ' -f1)
guard_sha=$(sha256sum "$scripts/verify-clean-source.sh" | cut -d' ' -f1)
start=$(date -u +%FT%TZ)
stage=admission
source=unknown
binary=unknown
count=0
preflight=false
readback=false
exec 3>&1
exec > "$output/preparation.log" 2>&1
finish() {
  result=$?
  trap - EXIT
  jq -n --arg start "$start" --arg finish "$(date -u +%FT%TZ)" --arg stage "$stage" \
    --arg source "$source" --arg binary "$binary" --arg build_sha "$build_sha" \
    --arg runner "$runner_sha" --arg guard "$guard_sha" --argjson count "$count" \
    --argjson preflight "$preflight" --argjson readback "$readback" --argjson code "$result" \
    '{schema:"ml-dataset.native-plan-preparation/v1",started_utc:$start,finished_utc:$finish,
      stage:$stage,source_commit:$source,binary_sha256:$binary,build_receipt_sha256:$build_sha,
      runner_sha256:$runner,guard_sha256:$guard,observed_commands:$count,exit_code:$code,
      source_preflight_matches:$preflight,source_readback_matches:$readback,
      plan_hash_source:"native crypto dataset plan receipts",protocols_frozen:false,
      generated_cells:0,scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$output/receipt.json"
  cat "$output/receipt.json" >&3
  exit "$result"
}
trap finish EXIT
printf '%s  %s\n' "$build_sha" "$build/receipt.json" | sha256sum -c -
jq -e '.exit_code==0 and .source_preflight_matches and .source_readback_matches' "$build/receipt.json" >/dev/null
source=$(jq -er .source_commit "$build/receipt.json")
package=$(jq -er .source_package "$build/receipt.json")
package_sha=$(jq -er .package_receipt_sha256 "$build/receipt.json")
binary=$(sha256sum "$build/crypto" | cut -d' ' -f1)
(cd "$build"; sha256sum -c binaries.sha256)
bash "$scripts/verify-clean-source.sh" "$package" "$package_sha"
preflight=true
metadata=$package/source/research/ml_dataset_20261028/startup-integration-cli-fixture-native
for name in cohorts.json native-cells.json limits.json planner-coverage.json normal-policy.json empty-policy.json paired-policy.json; do
  cp "$metadata/$name" "$output/inputs/$name"
  cmp "$metadata/$name" "$output/inputs/$name"
done
(cd "$output/inputs"; sha256sum ./*.json > "$output/inputs.sha256")
ids=(v1-bsgs v2-bsgs v2-empty-bsgs v2-empty-pdp v2-pkm-cap v2-lp-smallwork planner-coverage)
for i in 0 1 2 3 4 5 6; do
  id=${ids[$i]}
  stage=$id
  if [[ "$i" == 6 ]]; then
    config=$(jq -c '. + {seed:42,factor_base_size:2,degree_bound:4,work_limit:1,planned_cell_count:.planned_cells}' "$output/inputs/planner-coverage.json")
  else
    config=$(jq -c --argjson i "$i" '.[$i]' "$output/inputs/cohorts.json")
    [[ $(jq -r .id <<< "$config") == "$id" ]]
  fi
  args=("$build/crypto" dataset plan --out "$output/plans/$id.json")
  for key in n m families solvers; do
    args+=("--$key" "$(jq -r --arg key "$key" '.[$key]|map(tostring)|join(",")' <<< "$config")")
  done
  for key in curve_seeds generators keys seed factor_base_size degree_bound work_limit; do
    args+=("--${key//_/-}" "$(jq -er --arg key "$key" '.[$key]' <<< "$config")")
  done
  case "$id" in
    v1-bsgs) ;;
    v2-bsgs|planner-coverage) args+=(--factor-base-policy-file "$output/inputs/paired-policy.json") ;;
    v2-empty-bsgs|v2-empty-pdp) args+=(--factor-base-policy-file "$output/inputs/empty-policy.json") ;;
    *) args+=(--factor-base-policy-file "$output/inputs/normal-policy.json") ;;
  esac
  (cd "$build"; sha256sum -c binaries.sha256)
  set +e
  timeout --signal=TERM --kill-after=5s 15s "${args[@]}" > "$output/commands/$id.stdout" 2> "$output/commands/$id.stderr"
  code=$?
  set -e
  count=$((count+1))
  jq -n --arg id "$id" --argjson code "$code" \
    --argjson argv "$(printf '%s\n' "${args[@]}" | jq -Rsc 'split("\n")|map(select(length>0))')" \
    --arg stdout "$(sha256sum "$output/commands/$id.stdout" | cut -d' ' -f1)" \
    --arg stderr "$(sha256sum "$output/commands/$id.stderr" | cut -d' ' -f1)" \
    '{id:$id,argv:$argv,exit_code:$code,stdout_sha256:$stdout,stderr_sha256:$stderr,timeout_seconds:15}' > "$output/commands/$id.json"
  [[ "$code" == 0 ]] || exit "$code"
  [[ ! -s "$output/commands/$id.stderr" ]]
  sha=$(sha256sum "$output/plans/$id.json" | cut -d' ' -f1)
  jq -e --arg sha "$sha" --argjson expected "$(jq -r .planned_cell_count <<< "$config")" \
    '.sha256==$sha and .planned_cells==$expected and .generated_cells==0 and .twists_included==true and (.plan_sha256|test("^[0-9a-f]{64}$"))' "$output/commands/$id.stdout" >/dev/null
  jq -e 'if .schema=="crypto.ml-dataset.plan/v1" then .protocol_sha256==null else .axes.protocol_sha256==null end' "$output/plans/$id.json" >/dev/null
done
stage=source_readback
bash "$scripts/verify-clean-source.sh" "$package" "$package_sha"
(cd "$build"; sha256sum -c binaries.sha256)
(cd "$output/inputs"; sha256sum -c "$output/inputs.sha256")
[[ $(sha256sum "$0" | cut -d' ' -f1) == "$runner_sha" ]]
[[ $(sha256sum "$scripts/verify-clean-source.sh" | cut -d' ' -f1) == "$guard_sha" ]]
[[ $(sha256sum "$build/receipt.json" | cut -d' ' -f1) == "$build_sha" ]]
readback=true
stage=completed
