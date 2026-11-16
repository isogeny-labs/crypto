#!/bin/bash
# Explicit fixture-only continuation04. Prepare/freeze separately; never auto-retry.
set -euo pipefail
[[ $# == 2 && "$2" == 04 ]]
root=/home/ec2-user/crypto-ml-algebraic-targets-20261030
scratch=/home/ec2-user/ml-dataset-work-20261028
cd "$root"
source_commit=$(git rev-parse "$1^{commit}")
attempt=$2
runtime_source=db9bd045e38f2e55039d02372b92b5bee3f29ba0
prior_source=e935f7d9e4afeedbf42e13906433b0c440150795
old_inventory_sha=545a5dc37792d621a0617e6f20177763b1d0dce236fada3550978ca52293fe85
validation=research/ml_dataset_algebraic_targets_20261030/validation
prerequisites=$validation/continuation-04-prerequisites.json
reuse_receipt=$scratch/algebraic-reused-validation-$attempt.receipt.json
pipeline_receipt=$scratch/algebraic-pipeline-$attempt.receipt.json
inventory=$(jq -er '.materialization_inventory.path' "$validation/plan.json")
inventory_sha=$(jq -er '.materialization_inventory.sha256' "$validation/plan.json")

# Frozen receipt bytes, not only their reported success booleans, bind reuse.
git merge-base --is-ancestor "$prior_source" "$source_commit"
git cat-file -e "$source_commit:$validation/run-full-continuation.sh"
git cat-file -e "$source_commit:$prerequisites"
jq -e --arg inventory "$inventory" --arg sha "$inventory_sha" --arg old "$old_inventory_sha" '
  .schema=="software-validation-continuation-prerequisites/v1" and .attempt=="04" and
  .materialization_inventory=={path:$inventory,sha256:$sha} and
  .previous_inventory.sha256==$old and (.receipts|length)==7
' "$prerequisites" >/dev/null
jq -r '.receipts[]|.sha256+"  "+.path' "$prerequisites" | sha256sum -c -
printf '%s  %s\n' "$old_inventory_sha" "$scratch/algebraic-materialization-01.json" | sha256sum -c -
printf '%s  %s\n' "$inventory_sha" "$inventory" | sha256sum -c -
# The restored inventory is exactly the old entries plus the six archived manifests.
jq -e --slurpfile old "$scratch/algebraic-materialization-01.json" '
  .files==151 and (.entries|length)==151 and (.added_paths|length)==6 and
  (.entries|map(select(.path|startswith("tools/factor-base/validation/archive/manifests/")|not)))==$old[0].entries and
  ([.entries[].path]|unique|length)==151 and
  ([.entries[]|select(.path|startswith("tools/factor-base/validation/archive/manifests/"))|.path])==.added_paths
' "$inventory" >/dev/null
for name in dataset clippy build; do
  case "$name" in
    dataset) expected='["cargo","test","--release","--lib","ml_dataset::","--","--test-threads=1"]' ;;
    clippy) expected='["bash","scripts/clippy-maintained.sh"]' ;;
    build) expected='["cargo","build","--release","--bin","crypto"]' ;;
  esac
  jq -e --arg source "$runtime_source" --arg inventory "$old_inventory_sha" --argjson command "$expected" '
    .kind=="software-validation" and .source_commit==$source and .stage=="completed" and
    .exit_code==0 and .command_exit_code==0 and .command_started==true and
    .source_preflight_matches==true and .source_readback_matches==true and
    .materialization_inventory_sha256==$inventory and .command==$command
  ' "$scratch/algebraic-$name-02.receipt.json" >/dev/null
done
jq -e --arg source "$prior_source" --arg inventory "$old_inventory_sha" '
  .source_commit==$source and .stage=="completed" and .exit_code==0 and .command_exit_code==0 and
  .command_started==true and .source_preflight_matches==true and .source_readback_matches==true and
  .materialization_inventory_sha256==$inventory and
  .command==["bash","research/ml_dataset_algebraic_targets_20261030/cli-fixture/run.sh",
    "/home/ec2-user/ml-dataset-work-20261028/algebraic-export-crypto-03",
    "/home/ec2-user/ml-dataset-work-20261028/algebraic-cli-fixture-run-03"]
' "$scratch/algebraic-cli-03.receipt.json" >/dev/null
jq -e --arg source "$prior_source" --arg inventory "$old_inventory_sha" '
  .source_commit==$source and .stage=="command_failed" and .exit_code==101 and .command_exit_code==101 and
  .command_started==true and .source_preflight_matches==true and .source_readback_matches==true and
  .materialization_inventory_sha256==$inventory and
  .command==["cargo","test","--release","--lib","--","--test-threads=1"]
' "$scratch/algebraic-full-03.receipt.json" >/dev/null
jq -e --arg source "$prior_source" --arg runtime "$runtime_source" '
  .schema=="software-validation-pipeline/v1" and .source_commit==$source and
  .binary_runtime_source==$runtime and .attempt=="03" and .last_phase=="algebraic-full-03" and
  .exit_code==101 and (.finished_utc|type=="string")
' "$scratch/algebraic-pipeline-03.receipt.json" >/dev/null

# Preserve inherited guards and the already-passing CLI fixture byte for byte.
git diff --exit-code "$prior_source" "$source_commit" -- \
  "$validation/run-phase.sh" "$validation/verify-source.sh" "$validation/archive.sh" \
  "$validation/archive-cli.sh" research/ml_dataset_algebraic_targets_20261030/cli-fixture
temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT
git diff --name-only -z "$runtime_source" "$source_commit" > "$temporary/delta"
while IFS= read -r -d '' path; do
  case "$path" in
    docs/session-logs/*.md|docs/ml-dataset/*.md) ;;
    research/ml_dataset_algebraic_targets_20261030/cli-fixture/run.sh) ;;
    research/ml_dataset_algebraic_targets_20261030/validation/run-cli-continuation.sh|research/ml_dataset_algebraic_targets_20261030/validation/run-full-continuation.sh) ;;
    research/ml_dataset_algebraic_targets_20261030/*.md|research/ml_dataset_algebraic_targets_20261030/*.json|research/ml_dataset_algebraic_targets_20261030/*.txt|research/ml_dataset_algebraic_targets_20261030/*.log|research/ml_dataset_algebraic_targets_20261030/*.sha256) ;;
    *) printf 'Cannot reuse validation after changed runtime/build input: %s\n' "$path" >&2; exit 1 ;;
  esac
done < "$temporary/delta"
bash "$validation/verify-source.sh" "$source_commit" "$inventory" "$inventory_sha"

for path in "$reuse_receipt" "$pipeline_receipt" \
  "$scratch/algebraic-wide-archive-$attempt.log" "$scratch/algebraic-wide-archive-$attempt.receipt.json" \
  "$scratch/algebraic-full-$attempt.log" "$scratch/algebraic-full-$attempt.receipt.json"; do
  [[ ! -e "$path" && ! -L "$path" ]]
done
mkdir "$scratch/algebraic-continuation-$attempt.claim"
started=$(date -u +%FT%TZ)
phase=continuation_admission
finish() {
  code=$?
  trap - EXIT
  (set -o noclobber
    jq -n --arg source "$source_commit" --arg runtime "$runtime_source" --arg inventory "$inventory_sha" \
      --arg start "$started" --arg finish "$(date -u +%FT%TZ)" --arg phase "$phase" --argjson code "$code" \
      '{schema:"software-validation-pipeline/v1",source_commit:$source,runtime_source:$runtime,
        materialization_inventory_sha256:$inventory,continuation_of_attempt:"03",attempt:"04",
        reused_validation_receipt:"algebraic-reused-validation-04.receipt.json",
        started_utc:$start,finished_utc:$finish,last_phase:$phase,exit_code:$code,
        scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$pipeline_receipt")
  # Archive failures too; preserve the original command status if archiving fails.
  archive_code=0
  bash "$validation/archive.sh" "$pipeline_receipt" || archive_code=$?
  rm -rf "$temporary"
  if [[ "$code" != 0 ]]; then exit "$code"; fi
  exit "$archive_code"
}
trap finish EXIT
jq -Rs 'split("\u0000")|map(select(length>0))' "$temporary/delta" > "$temporary/delta.json"
(set -o noclobber
  jq -n --arg source "$source_commit" --arg runtime "$runtime_source" --arg prior "$prior_source" \
    --arg inventory "$inventory_sha" --slurpfile delta "$temporary/delta.json" --slurpfile prerequisites "$prerequisites" \
    '{schema:"software-validation-reuse/v1",fixture_validation_source:$source,runtime_source:$runtime,
      previous_cli_source:$prior,materialization_inventory_sha256:$inventory,changed_paths:$delta[0],
      prior_receipts:$prerequisites[0].receipts,
      reused_phases:["dataset02","clippy02","release-build02","cli03"],
      remaining_phases:["wide-archive-test","full-library"],
      inherited_guards_unchanged:true,runtime_and_build_inputs_unchanged:true,cli_fixture_unchanged_since03:true,
      historical_failure_preserved:true,scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$reuse_receipt")
bash "$validation/archive.sh" "$inventory" "$reuse_receipt"
validate() {
  phase=$1
  shift
  local result=0 archive_result=0
  bash "$validation/run-phase.sh" "$phase" "$source_commit" "$@" || result=$?
  bash "$validation/archive.sh" "$scratch/$phase.log" "$scratch/$phase.receipt.json" || archive_result=$?
  if [[ "$result" != 0 ]]; then return "$result"; fi
  return "$archive_result"
}
validate "algebraic-wide-archive-$attempt" cargo test --release --lib \
  cryptanalysis::ic_framework::archive::tests::wide_archive_is_rejected_without_truncation -- --exact --test-threads=1
validate "algebraic-full-$attempt" cargo test --release --lib -- --test-threads=1
phase=completed
