#!/bin/bash
# Thin fixed-command development validation; never executes a dataset workflow.
set -euo pipefail
[[ $# == 5 ]] || { echo 'usage: run-combined-full-tests.sh full SNAPSHOT MANIFEST_SHA ATTEMPT OUTPUT_DIR' >&2; exit 2; }
phase=$1
snapshot=$2
manifest_sha=$3
attempt=$4
output=$5
case "$phase" in
  full) command_args=(cargo test --locked --release --lib -- --test-threads=1) ;;
  *) echo 'unsupported validation phase' >&2; exit 2 ;;
esac
[[ "$attempt" =~ ^combined-${phase}-[0-9][0-9]$ ]]
[[ -d "$output" ]]
log=$output/$attempt.log
receipt=$output/$attempt.receipt.json
[[ ! -e "$log" && ! -L "$log" && ! -e "$receipt" && ! -L "$receipt" ]]
(set -o noclobber; : > "$log")
start=$(date -u +%FT%TZ)
stage=preflight
preflight=false
readback=false
command_started=false
command_exit=null
base=unknown
overlay=unknown
scripts=$(cd -- "$(dirname -- "$0")" && pwd)
runner_sha=$(sha256sum "$0" | cut -d' ' -f1)
guard_sha=$(sha256sum "$scripts/verify-proposal-source.sh" | cut -d' ' -f1)
command_json=$(printf '%s\n' "${command_args[@]}" | jq -Rsc 'split("\n")|map(select(length>0))')
exec 3>&1
exec >> "$log" 2>&1
finish() {
  result=$?
  trap - EXIT
  jq -n --arg start "$start" --arg finish "$(date -u +%FT%TZ)" --arg stage "$stage" \
    --arg phase "$phase" --arg source "$snapshot" --arg manifest "$manifest_sha" \
    --arg base "$base" --arg overlay "$overlay" --arg runner "$runner_sha" \
    --arg guard "$guard_sha" --argjson result "$result" --argjson command "$command_json" \
    --argjson command_exit "$command_exit" --argjson started "$command_started" \
    --argjson preflight "$preflight" --argjson readback "$readback" \
    '{schema:"proposal-software-validation/v1",started_utc:$start,finished_utc:$finish,stage:$stage,phase:$phase,
      snapshot:$source,manifest_sha256:$manifest,base_commit:$base,overlay_sha256:$overlay,
      runner_sha256:$runner,guard_sha256:$guard,exit_code:$result,command_exit_code:$command_exit,
      command_started:$started,source_preflight_matches:$preflight,source_readback_matches:$readback,
      build_git_dirty:true,command:$command,wall_timeout_seconds:1800,cargo_jobs:8,rayon_threads:8,test_threads:1,
      execution_scope:"isolated proposal unit tests; no CLI workflow, clean delivery or corpus claim",
      scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$receipt"
  cat "$receipt" >&3
  exit "$result"
}
trap finish EXIT
bash "$scripts/verify-proposal-source.sh" "$snapshot" "$manifest_sha"
base=$(jq -er '.base_commit' "$snapshot/manifest.json")
overlay=$(jq -er '.overlay_sha256' "$snapshot/manifest.json")
preflight=true
stage=command
command_started=true
set +e
(cd "$snapshot/source"; env CRYPTO_BUILD_GIT_COMMIT="$base" CRYPTO_BUILD_GIT_DIRTY=true \
  CARGO_BUILD_JOBS=8 RAYON_NUM_THREADS=8 RUSTUP_TOOLCHAIN=1.98 \
  CARGO_TARGET_DIR=/home/ec2-user/crypto-ml-dataset-20261028/target \
  timeout --signal=TERM --kill-after=30s 1800s "${command_args[@]}")
command_exit=$?
set -e
stage=source_readback
if bash "$scripts/verify-proposal-source.sh" "$snapshot" "$manifest_sha"; then readback=true; fi
[[ $(sha256sum "$scripts/verify-proposal-source.sh" | cut -d' ' -f1) == "$guard_sha" ]]
[[ $(sha256sum "$0" | cut -d' ' -f1) == "$runner_sha" ]]
if [[ "$command_exit" != 0 ]]; then stage=command_failed; exit "$command_exit"; fi
[[ "$readback" == true ]]
stage=completed
