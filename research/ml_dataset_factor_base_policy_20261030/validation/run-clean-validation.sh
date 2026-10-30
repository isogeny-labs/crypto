#!/bin/bash
set -euo pipefail
[[ $# == 4 ]] || { echo 'usage: run-clean-validation.sh BUILD BUILD_RECEIPT_SHA PHASE OUTPUT' >&2; exit 2; }
build=$(realpath "$1")
build_sha=$2
phase=$3
output=$4
case "$phase" in
  fixture) command_args=(cargo test --locked --release --example dataset_startup_cli_fixture -- --test-threads=1); summary='test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;' ;;
  dataset) command_args=(cargo test --locked --release --lib ml_dataset:: -- --test-threads=1); summary='test result: ok. 479 passed; 0 failed; 2 ignored; 0 measured;' ;;
  clippy) command_args=(bash scripts/clippy-maintained.sh); summary='' ;;
  *) echo 'unsupported validation phase' >&2; exit 2 ;;
esac
[[ ! -e "$output" && ! -L "$output" ]]
mkdir "$output"
output=$(realpath "$output")
scripts=$(cd -- "$(dirname -- "$0")" && pwd)
runner_sha=$(sha256sum "$0" | cut -d' ' -f1)
guard_sha=$(sha256sum "$scripts/verify-clean-source.sh" | cut -d' ' -f1)
start=$(date -u +%FT%TZ)
stage=admission
source=unknown
binary=unknown
preflight=false
readback=false
command_exit=null
command_started=false
exec 3>&1
exec > "$output/validation.log" 2>&1
finish() {
  result=$?
  trap - EXIT
  jq -n --arg start "$start" --arg finish "$(date -u +%FT%TZ)" --arg phase "$phase" \
    --arg stage "$stage" --arg source "$source" --arg binary "$binary" --arg build_sha "$build_sha" \
    --arg runner "$runner_sha" --arg guard "$guard_sha" --arg log_sha "$(sha256sum "$output/validation.log" | cut -d' ' -f1)" \
    --argjson command "$(printf '%s\n' "${command_args[@]}" | jq -Rsc 'split("\n")|map(select(length>0))')" \
    --argjson result "$result" --argjson command_exit "$command_exit" --argjson started "$command_started" \
    --argjson preflight "$preflight" --argjson readback "$readback" \
    '{schema:"ml-dataset.clean-software-validation/v1",started_utc:$start,finished_utc:$finish,
      phase:$phase,stage:$stage,source_commit:$source,binary_sha256:$binary,build_receipt_sha256:$build_sha,
      runner_sha256:$runner,guard_sha256:$guard,log_sha256:$log_sha,command:$command,
      exit_code:$result,command_exit_code:$command_exit,command_started:$started,
      source_preflight_matches:$preflight,source_readback_matches:$readback,
      build_git_dirty:false,wall_timeout_seconds:1800,cargo_jobs:8,rayon_threads:8,test_threads:1,
      scope:"clean source package readiness; test harnesses are separate from retained CLI",
      scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$output/receipt.json"
  cat "$output/receipt.json" >&3
  exit "$result"
}
trap finish EXIT
printf '%s  %s\n' "$build_sha" "$build/receipt.json" | sha256sum -c -
jq -e '.exit_code==0 and .command_exit_code==0 and .source_preflight_matches and .source_readback_matches' "$build/receipt.json" >/dev/null
source=$(jq -er .source_commit "$build/receipt.json")
package=$(jq -er .source_package "$build/receipt.json")
package_sha=$(jq -er .package_receipt_sha256 "$build/receipt.json")
[[ "$source" == "$(jq -er .source_commit "$package/receipt.json")" ]]
binary=$(sha256sum "$build/crypto" | cut -d' ' -f1)
(cd "$build"; sha256sum -c binaries.sha256)
bash "$scripts/verify-clean-source.sh" "$package" "$package_sha"
preflight=true
stage=command
command_started=true
set +e
(cd "$package/source"; env CRYPTO_BUILD_GIT_COMMIT="$source" CRYPTO_BUILD_GIT_DIRTY=false \
  CARGO_BUILD_JOBS=8 RAYON_NUM_THREADS=8 RUSTUP_TOOLCHAIN=1.98 \
  CARGO_TARGET_DIR=/home/ec2-user/crypto-ml-dataset-20261028/target \
  timeout --signal=TERM --kill-after=30s 1800s "${command_args[@]}")
command_exit=$?
set -e
stage=source_readback
if bash "$scripts/verify-clean-source.sh" "$package" "$package_sha"; then readback=true; fi
[[ $(sha256sum "$0" | cut -d' ' -f1) == "$runner_sha" ]]
[[ $(sha256sum "$scripts/verify-clean-source.sh" | cut -d' ' -f1) == "$guard_sha" ]]
[[ $(sha256sum "$build/receipt.json" | cut -d' ' -f1) == "$build_sha" ]]
(cd "$build"; sha256sum -c binaries.sha256)
[[ $(sha256sum "$build/crypto" | cut -d' ' -f1) == "$binary" ]]
if [[ "$command_exit" != 0 ]]; then stage=command_failed; exit "$command_exit"; fi
[[ "$readback" == true ]]
stage=coverage_check
if [[ -n "$summary" ]]; then
  [[ $(rg -c '^test result:' "$output/validation.log") == 1 ]]
  rg -F "$summary" "$output/validation.log" > "$output/test-summary.txt"
fi
stage=completed
