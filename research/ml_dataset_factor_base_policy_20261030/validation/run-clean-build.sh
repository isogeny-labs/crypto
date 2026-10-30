#!/bin/bash
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: run-clean-build.sh PACKAGE RECEIPT_SHA OUTPUT' >&2; exit 2; }
package=$(realpath "$1")
receipt_sha=$2
output=$3
[[ ! -e "$output" && ! -L "$output" ]]
mkdir "$output"
output=$(realpath "$output")
runner=$(realpath "$0")
runner_sha=$(sha256sum "$runner" | cut -d' ' -f1)
start=$(date -u +%FT%TZ)
stage=admission
commit=unknown
preflight=false
readback=false
command_exit=null
target=/home/ec2-user/crypto-ml-dataset-20261028/target
command_args=(cargo build --locked --release --bin crypto --example dataset_startup_cli_fixture)
exec 3>&1
exec > "$output/build.log" 2>&1
finish() {
  result=$?
  trap - EXIT
  jq -n --arg start "$start" --arg finish "$(date -u +%FT%TZ)" \
    --arg stage "$stage" --arg commit "$commit" --arg package "$package" \
    --arg package_sha "$receipt_sha" --arg runner_sha "$runner_sha" \
    --arg log_sha "$(sha256sum "$output/build.log" | cut -d' ' -f1)" \
    --argjson command "$(printf '%s\n' "${command_args[@]}" | jq -Rsc 'split("\n")|map(select(length>0))')" \
    --argjson result "$result" --argjson command_exit "$command_exit" \
    --argjson preflight "$preflight" --argjson readback "$readback" \
    '{schema:"ml-dataset.clean-software-build/v1",started_utc:$start,finished_utc:$finish,
      stage:$stage,source_commit:$commit,source_package:$package,package_receipt_sha256:$package_sha,
      runner_sha256:$runner_sha,log_sha256:$log_sha,command:$command,exit_code:$result,
      command_exit_code:$command_exit,source_preflight_matches:$preflight,source_readback_matches:$readback,
      build_git_dirty:false,wall_timeout_seconds:1800,cargo_jobs:8,rayon_threads:8,
      source_identity:"standalone validation source closure; not upstream delivery",
      scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$output/receipt.json"
  cat "$output/receipt.json" >&3
  exit "$result"
}
trap finish EXIT
printf '%s  %s\n' "$receipt_sha" "$package/receipt.json" | sha256sum -c -
jq -e '.schema=="ml-dataset.clean-source-package/v1" and .exit_code==0 and .source_preflight_matches and .source_readback_matches' "$package/receipt.json"
commit=$(jq -er .source_commit "$package/receipt.json")
[[ "$commit" =~ ^[a-f0-9]{40}$ ]]
verify_source() {
  printf '%s  %s\n' "$(jq -er .input_manifest_sha256 "$package/receipt.json")" "$package/manifest.json" | sha256sum -c -
  for mapping in 'input_hash_manifest_sha256 source.sha256' 'mode_manifest_sha256 source-modes.tsv' 'path_manifest_sha256 final-paths.bin'; do
    read -r key name <<< "$mapping"
    printf '%s  %s\n' "$(jq -er --arg key "$key" '.[$key]' "$package/manifest.json")" "$package/$name" | sha256sum -c -
  done
  (cd "$package"; sha256sum -c package.sha256)
  [[ $(git -C "$package/source" rev-parse HEAD) == "$commit" ]]
  [[ -z $(git -C "$package/source" status --porcelain --untracked-files=all) ]]
  git -C "$package/source" ls-files -z | sort -z | cmp "$package/final-paths.bin" -
  (cd "$package/source"; rg --files --hidden --no-ignore --glob '!.git/**' -0 | sort -z) | cmp "$package/final-paths.bin" -
  (cd "$package/source"; sha256sum -c "$package/source.sha256")
  while IFS=$'\t' read -r path mode; do
    [[ -f "$package/source/$path" && ! -L "$package/source/$path" ]] || return 1
    [[ $(stat -c %a -- "$package/source/$path") == "$mode" ]] || return 1
  done < "$package/source-modes.tsv"
  [[ -z $(find -P "$package/source" -path "$package/source/.git" -prune -o ! -type d ! -type f -print -quit) ]]
}
verify_source
preflight=true
stage=command
set +e
(cd "$package/source"; env CRYPTO_BUILD_GIT_COMMIT="$commit" CRYPTO_BUILD_GIT_DIRTY=false \
  CARGO_BUILD_JOBS=8 RAYON_NUM_THREADS=8 RUSTUP_TOOLCHAIN=1.98 CARGO_TARGET_DIR="$target" \
  timeout --signal=TERM --kill-after=30s 1800s "${command_args[@]}")
command_exit=$?
set -e
stage=source_readback
set +e
(set -e; verify_source)
guard_exit=$?
set -e
[[ "$guard_exit" != 0 ]] || readback=true
[[ $(sha256sum "$runner" | cut -d' ' -f1) == "$runner_sha" ]]
[[ $(sha256sum "$package/receipt.json" | cut -d' ' -f1) == "$receipt_sha" ]]
if [[ "$command_exit" != 0 ]]; then stage=command_failed; exit "$command_exit"; fi
[[ "$readback" == true ]]
stage=retain_binaries
cp "$target/release/crypto" "$output/crypto"
cp "$target/release/examples/dataset_startup_cli_fixture" "$output/dataset_startup_cli_fixture"
(cd "$output"; sha256sum crypto dataset_startup_cli_fixture > binaries.sha256)
jq -n --arg commit "$commit" \
  --arg crypto_sha "$(sha256sum "$output/crypto" | cut -d' ' -f1)" \
  --arg fixture_sha "$(sha256sum "$output/dataset_startup_cli_fixture" | cut -d' ' -f1)" \
  --argjson crypto_bytes "$(stat -c %s "$output/crypto")" \
  --argjson fixture_bytes "$(stat -c %s "$output/dataset_startup_cli_fixture")" \
  '{source_commit:$commit,crypto:{sha256:$crypto_sha,bytes:$crypto_bytes},dataset_startup_cli_fixture:{sha256:$fixture_sha,bytes:$fixture_bytes}}' > "$output/binaries.json"
stage=completed
