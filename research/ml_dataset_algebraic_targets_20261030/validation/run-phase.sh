#!/bin/bash
set -euo pipefail
phase=$1
source_commit=$2
shift 2
command=("$@")
root=/home/ec2-user/crypto-ml-algebraic-targets-20261030
scratch=/home/ec2-user/ml-dataset-work-20261028
cd "$root"
log=$scratch/$phase.log
receipt=$scratch/$phase.receipt.json
# A reused attempt name is refused without changing its retained artifacts.
[[ ! -e "$log" && ! -e "$receipt" ]]
(set -o noclobber; : > "$log")
start=$(date -u +%FT%TZ)
stage=preflight
preflight_match=false
match=false
command_started=false
command_code=null
inventory_sha=unknown
exec 3>&1
exec >> "$log" 2>&1
finish() {
  result=$?
  trap - EXIT
  jq -n --arg source "$source_commit" --arg start "$start" --arg finish "$(date -u +%FT%TZ)" \
    --arg stage "$stage" --arg inventory_sha "$inventory_sha" \
    --argjson code "$result" --argjson command_code "$command_code" \
    --argjson started "$command_started" --argjson preflight "$preflight_match" --argjson match "$match" \
    --args '{kind:"software-validation",source_commit:$source,started_utc:$start,finished_utc:$finish,
      stage:$stage,exit_code:$code,command_started:$started,command_exit_code:$command_code,
      source_preflight_matches:$preflight,source_readback_matches:$match,
      materialization_inventory_sha256:$inventory_sha,command:$ARGS.positional,
      scientific_samples_added:0,verified_cloud_dataset_cells:0}' -- "${command[@]}" > "$receipt"
  cat "$receipt" >&3
  exit "$result"
}
trap finish EXIT
source_commit=$(git rev-parse "$source_commit^{commit}")
validation=research/ml_dataset_algebraic_targets_20261030/validation
inventory=$(jq -er '.materialization_inventory.path' "$validation/plan.json")
inventory_sha=$(jq -er '.materialization_inventory.sha256' "$validation/plan.json")
bash "$validation/verify-source.sh" "$source_commit" "$inventory" "$inventory_sha"
preflight_match=true
stage=command
command_started=true
set +e
env CRYPTO_BUILD_GIT_COMMIT="$source_commit" CRYPTO_BUILD_GIT_DIRTY=false \
  CARGO_BUILD_JOBS=8 RAYON_NUM_THREADS=8 RUSTUP_TOOLCHAIN=1.98 \
  CARGO_TARGET_DIR=/home/ec2-user/crypto-ml-dataset-20261028/target "${command[@]}"
command_code=$?
set -e
stage=source_readback
if bash "$validation/verify-source.sh" "$source_commit" "$inventory" "$inventory_sha"; then
  match=true
fi
if [[ "$command_code" != 0 ]]; then
  stage=command_failed
  exit "$command_code"
fi
[[ "$match" == true ]]
stage=completed
