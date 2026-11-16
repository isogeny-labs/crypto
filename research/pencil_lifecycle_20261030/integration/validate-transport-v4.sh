#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../../.."
export CARGO_TARGET_DIR=/tmp/pencil-transport-software-target
out=research/pencil_lifecycle_20261030/integration
manifest="$out/transport-source-hashes-v4.json"
checksum="$out/transport-source-hashes-v4.sha256"
set -o noclobber
{
  jq -r 'keys[]' "$out/transport-source-hashes-v3.json"
  printf '%s\n' taskq/deploy/eks-karpenter/resume/controller/src/deadline.rs "$out/validate-transport-v4.sh"
} | sort -u | while IFS= read -r path; do sha256sum "$path"; done > "$checksum"
jq -Rn '[inputs | capture("^(?<hash>[a-f0-9]{64})  (?<path>.*)$") | {key:.path,value:.hash}] | from_entries' < "$checksum" > "$manifest"
check_source() { sha256sum --check --status "$checksum"; }
run_check() {
  local name=$1
  shift
  check_source
  local started ended rc command_json manifest_sha stdout_sha stderr_sha
  started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  command_json=$(jq -cn --args '$ARGS.positional' -- "$@")
  printf 'START %s\n' "$name"
  if "$@" > "$out/$name.stdout" 2> "$out/$name.stderr"; then rc=0; else rc=$?; fi
  ended=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  manifest_sha=$(sha256sum "$manifest"); manifest_sha=${manifest_sha%% *}
  stdout_sha=$(sha256sum "$out/$name.stdout"); stdout_sha=${stdout_sha%% *}
  stderr_sha=$(sha256sum "$out/$name.stderr"); stderr_sha=${stderr_sha%% *}
  jq -n --argjson command "$command_json" --argjson exit_code "$rc" --arg start "$started" --arg end "$ended" --arg manifest "${manifest##*/}" --arg manifest_sha "$manifest_sha" --arg stdout_sha "$stdout_sha" --arg stderr_sha "$stderr_sha" '{command:$command,exit_code:$exit_code,started_utc:$start,ended_utc:$end,source_manifest:$manifest,source_manifest_sha256:$manifest_sha,scientific_cohort_execution:false,purpose:"software regression; known-answer fixtures only",logs:{stdout:$stdout_sha,stderr:$stderr_sha}}' > "$out/$name.receipt.json"
  printf 'END %s %s\n' "$name" "$rc"
  check_source
  if ((rc != 0)); then tail -n 40 "$out/$name.stderr" "$out/$name.stdout"; return "$rc"; fi
}
for component in controller fresh-owner; do
  crate="taskq/deploy/eks-karpenter/resume/$component/Cargo.toml"
  run_check "$component-test-transport-v4" cargo test --manifest-path "$crate" --locked --offline -j 2 -- --test-threads=1 --quiet
  run_check "$component-clippy-transport-v4" cargo clippy --manifest-path "$crate" --locked --offline --all-targets -j 2 -- -D warnings
done
printf 'TRANSPORT V4 SOFTWARE CHECKS PASSED; NO CLOUD OR SCIENTIFIC EXECUTION\n'
