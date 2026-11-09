#!/bin/bash
# Explicit continuation after attempt02's fixture assertion failure. No automatic retry.
set -euo pipefail
[[ $# == 2 && "$2" == 03 ]]
root=/home/ec2-user/crypto-ml-algebraic-targets-20261030
scratch=/home/ec2-user/ml-dataset-work-20261028
cd "$root"
source_commit=$(git rev-parse "$1^{commit}")
attempt=$2
prior_source=db9bd045e38f2e55039d02372b92b5bee3f29ba0
prior_binary_sha=e006561583ee3e8895c27ea6fbe92546a198ec341cea0bbf30cfe9c236626f39
validation=research/ml_dataset_algebraic_targets_20261030/validation
prior_binary=$scratch/algebraic-export-crypto-02
binary=$scratch/algebraic-export-crypto-$attempt
prior_hash_file=$scratch/algebraic-cli-fixture-run-02/binary.sha256
reuse_receipt=$scratch/algebraic-reused-validation-$attempt.receipt.json
pipeline_receipt=$scratch/algebraic-pipeline-$attempt.receipt.json

# Require the known terminal outcomes, not a stale lock or a missing tool handle.
jq -e '.exit_code == 0 and .last_phase == "completed" and .source_commit == "d9a69910198d14a829b689253a62a7663b41d753"' "$scratch/prefix-cap-fix-pipeline.receipt.json" >/dev/null
jq -e --arg source "$prior_source" '.schema=="software-validation-pipeline/v1" and .source_commit==$source and .last_phase=="algebraic-cli-02" and .exit_code==1 and (.finished_utc|type=="string")' "$scratch/algebraic-pipeline-02.receipt.json" >/dev/null
jq -e --arg source "$prior_source" '.source_commit==$source and .exit_code==1 and .command_exit_code==1 and .stage=="command_failed" and .command_started==true and .source_preflight_matches==true and .source_readback_matches==true' "$scratch/algebraic-cli-02.receipt.json" >/dev/null
inventory=$(jq -er '.materialization_inventory.path' "$validation/plan.json")
inventory_sha=$(jq -er '.materialization_inventory.sha256' "$validation/plan.json")
for name in dataset clippy build; do
  case "$name" in
    dataset) expected='["cargo","test","--release","--lib","ml_dataset::","--","--test-threads=1"]' ;;
    clippy) expected='["bash","scripts/clippy-maintained.sh"]' ;;
    build) expected='["cargo","build","--release","--bin","crypto"]' ;;
  esac
  jq -e --arg source "$prior_source" --arg inventory "$inventory_sha" --argjson command "$expected" '
    .kind=="software-validation" and .source_commit==$source and .stage=="completed" and
    .exit_code==0 and .command_exit_code==0 and .command_started==true and
    .source_preflight_matches==true and .source_readback_matches==true and
    .materialization_inventory_sha256==$inventory and .command==$command
  ' "$scratch/algebraic-$name-02.receipt.json" >/dev/null
done

# The source delta can change the fixture and evidence, never executable Rust,
# dependency/configuration/build inputs, or the inherited validation guards.
git merge-base --is-ancestor "$prior_source" "$source_commit"
# Untracked shell files are outside verify-source's Rust/Cargo discovery checks.
# Require this new runner itself to exist in the frozen execution commit.
git cat-file -e "$source_commit:$validation/run-cli-continuation.sh"
git diff --exit-code "$prior_source" "$source_commit" -- \
  "$validation/run-phase.sh" "$validation/verify-source.sh" \
  "$validation/archive.sh" "$validation/archive-cli.sh"
temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT
git diff --name-only -z "$prior_source" "$source_commit" > "$temporary/delta"
while IFS= read -r -d '' path; do
  case "$path" in
    docs/session-logs/*.md|docs/ml-dataset/*.md) ;;
    research/ml_dataset_algebraic_targets_20261030/cli-fixture/run.sh|research/ml_dataset_algebraic_targets_20261030/validation/run-cli-continuation.sh) ;;
    research/ml_dataset_algebraic_targets_20261030/*.md|research/ml_dataset_algebraic_targets_20261030/*.json|research/ml_dataset_algebraic_targets_20261030/*.txt|research/ml_dataset_algebraic_targets_20261030/*.log|research/ml_dataset_algebraic_targets_20261030/*.sha256) ;;
    *) printf 'Cannot reuse prior validation after changed build/runtime input: %s\n' "$path" >&2; exit 1 ;;
  esac
done < "$temporary/delta"
bash "$validation/verify-source.sh" "$source_commit" "$inventory" "$inventory_sha"
[[ -f "$prior_binary" && ! -L "$prior_binary" && -x "$prior_binary" ]]
[[ -f "$prior_hash_file" && ! -L "$prior_hash_file" ]]
[[ $(wc -l < "$prior_hash_file") == 1 ]]
[[ $(cat "$prior_hash_file") == "$prior_binary_sha  $prior_binary" ]]
sha256sum -c "$prior_hash_file"

# Distinct attempt names and an atomic claim prevent overwriting or duplicate work.
for path in "$pipeline_receipt" "$reuse_receipt" "$binary" \
  "$scratch/algebraic-cli-$attempt.log" "$scratch/algebraic-cli-$attempt.receipt.json" \
  "$scratch/algebraic-full-$attempt.log" "$scratch/algebraic-full-$attempt.receipt.json" \
  "$scratch/algebraic-cli-fixture-run-$attempt" "$scratch/algebraic-cli-fixture-run-$attempt.tar.xz"; do
  [[ ! -e "$path" && ! -L "$path" ]]
done
mkdir "$scratch/algebraic-continuation-$attempt.claim"
started=$(date -u +%FT%TZ)
phase=continuation_admission
finish() {
  code=$?
  trap - EXIT
  (set -o noclobber
    jq -n --arg source "$source_commit" --arg runtime "$prior_source" --arg binary_sha "$prior_binary_sha" \
      --arg start "$started" --arg finish "$(date -u +%FT%TZ)" --arg phase "$phase" --argjson code "$code" \
      '{schema:"software-validation-pipeline/v1",source_commit:$source,binary_runtime_source:$runtime,
        reused_binary_sha256:$binary_sha,continuation_of_attempt:"02",attempt:"03",
        reused_validation_receipt:"algebraic-reused-validation-03.receipt.json",
        started_utc:$start,finished_utc:$finish,last_phase:$phase,exit_code:$code,
        scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$pipeline_receipt")
  rm -rf "$temporary"
  exit "$code"
}
trap finish EXIT

# Copy exact retained bytes, without replacing either the original or a raced file.
(set -o noclobber; cat "$prior_binary" > "$binary")
chmod 0555 "$binary"
printf '%s  %s\n' "$prior_binary_sha" "$binary" | sha256sum -c -
sha256sum -c "$prior_hash_file"
prerequisites=()
for name in dataset clippy build cli; do
  file=$scratch/algebraic-$name-02.receipt.json
  prerequisites+=("$(jq -cn --arg path "$file" --arg sha "$(sha256sum "$file" | cut -d' ' -f1)" '{path:$path,sha256:$sha}')")
done
file=$scratch/algebraic-pipeline-02.receipt.json
prerequisites+=("$(jq -cn --arg path "$file" --arg sha "$(sha256sum "$file" | cut -d' ' -f1)" '{path:$path,sha256:$sha}')")
jq -Rs 'split("\u0000")|map(select(length>0))' "$temporary/delta" > "$temporary/delta.json"
(set -o noclobber
  jq -n --arg source "$source_commit" --arg runtime "$prior_source" --arg binary "$binary" \
    --arg sha "$prior_binary_sha" --arg inventory "$inventory_sha" --slurpfile delta "$temporary/delta.json" \
    --args '{schema:"software-validation-reuse/v1",fixture_validation_source:$source,
      binary_runtime_source:$runtime,binary:$binary,binary_sha256:$sha,
      materialization_inventory_sha256:$inventory,changed_paths:$delta[0],
      prior_receipts:($ARGS.positional|map(fromjson)),
      reused_phases:["dataset","clippy","release-build"],remaining_phases:["cli","full-library"],
      inherited_guards_unchanged:true,runtime_and_build_inputs_unchanged:true,
      historical_failure_preserved:true,scientific_samples_added:0,verified_cloud_dataset_cells:0}' \
    -- "${prerequisites[@]}" > "$reuse_receipt")
bash "$validation/archive.sh" "$reuse_receipt"

validate() {
  phase=$1
  shift
  local result=0 archive_result=0 cli_archive_result=0
  bash "$validation/run-phase.sh" "$phase" "$source_commit" "$@" || result=$?
  bash "$validation/archive.sh" "$scratch/$phase.log" "$scratch/$phase.receipt.json" || archive_result=$?
  if [[ "$phase" == "algebraic-cli-$attempt" ]]; then
    bash "$validation/archive-cli.sh" "$attempt" || cli_archive_result=$?
  fi
  if [[ "$result" != 0 ]]; then return "$result"; fi
  if [[ "$archive_result" != 0 ]]; then return "$archive_result"; fi
  return "$cli_archive_result"
}
validate "algebraic-cli-$attempt" bash research/ml_dataset_algebraic_targets_20261030/cli-fixture/run.sh "$binary" "$scratch/algebraic-cli-fixture-run-$attempt"
printf '%s  %s\n' "$prior_binary_sha" "$binary" | sha256sum -c -
sha256sum -c "$prior_hash_file"
validate "algebraic-full-$attempt" cargo test --release --lib -- --test-threads=1
phase=completed
