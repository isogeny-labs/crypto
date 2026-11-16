#!/bin/bash
# Thin storage/CLI orchestration. All cryptographic replay runs in the frozen Rust binary.
set -euo pipefail
output=$1
[[ "$output" == /* && ! -e "$output" ]]
mkdir -p "$output"
protocol=$(dirname "${BASH_SOURCE[0]}")/protocol.json
cp "$protocol" "$output/protocol.json"
started=$(date -u +%FT%TZ)
phase=download
commands_passed=0
negative_rejected=false
finish() {
  code=$?
  trap - EXIT
  jq -n --arg start "$started" --arg finish "$(date -u +%FT%TZ)" --arg phase "$phase" --argjson code "$code" --argjson commands "$commands_passed" --argjson negative "$negative_rejected" '{schema:"software-restore-fixture-result/v1",started_utc:$start,finished_utc:$finish,last_phase:$phase,exit_code:$code,commands_passed:$commands,negative_control_rejected:$negative,scientific_samples_added:0,verified_cloud_dataset_cells:0,independent_reviewer:false,binary_source_commit:"06773dcf28f1d77fd43341b7959b7571fa1abbd0"}' > "$output/result.json"
  exit "$code"
}
trap finish EXIT
fetch() {
  sha=$1
  name=$2
  aws s3api get-object --bucket setec-astronomy-artifacts --key "store/sha256/${sha:0:2}/$sha" "$output/$name" > "$output/$name.get.json"
  printf '%s  %s\n' "$sha" "$output/$name" | sha256sum -c - >> "$output/download-hashes.txt"
}
fetch "$(jq -r .binary_sha256 "$protocol")" crypto
fetch "$(jq -r .input_archive_sha256 "$protocol")" inputs.tar.xz
fetch "$(jq -r .expected_export_archive_sha256 "$protocol")" expected.tar.xz
chmod u+x "$output/crypto"
phase=extract_verified_archives
mkdir "$output/inputs" "$output/expected"
for item in inputs expected; do
  tar -tJf "$output/$item.tar.xz" > "$output/$item.members.txt"
  awk '/^\// || /(^|\/)\.\.(\/|$)/ {bad=1} END {exit bad}' "$output/$item.members.txt"
  tar --extract --xz --file "$output/$item.tar.xz" --directory "$output/$item" --no-same-owner --no-same-permissions
  [[ -z $(find "$output/$item" -type l -print -quit) ]]
done
input=$output/inputs/runs
expected=$output/expected
(cd "$input" && find . -type f -print0 | sort -z | xargs -0 sha256sum) > "$output/input-before.sha256"
for shard in 62 63; do
  phase=verify-$shard
  "$output/crypto" dataset verify --out "$input/shard-$shard" > "$output/verify-$shard.stdout.json" 2> "$output/verify-$shard.stderr.log"
  commands_passed=$((commands_passed+1))
  phase=export-$shard
  "$output/crypto" dataset export --out "$input/shard-$shard" --destination "$output/export-$shard" > "$output/export-$shard.stdout.json" 2> "$output/export-$shard.stderr.log"
  commands_passed=$((commands_passed+1))
  "$output/crypto" dataset export --out "$input/shard-$shard" --destination "$output/export-$shard" > "$output/retry-$shard.stdout.json" 2> "$output/retry-$shard.stderr.log"
  commands_passed=$((commands_passed+1))
  cmp "$output/export-$shard.stdout.json" "$output/retry-$shard.stdout.json"
  for model_input in features.jsonl prefix-features.jsonl; do
    cmp "$output/export-$shard/$model_input" "$expected/export-$shard/$model_input"
  done
  jq -e '.feature_policy.version=="strict-structural-allowlist-v5" and .feature_policy.model_input_files==["features.jsonl","prefix-features.jsonl"]' "$output/export-$shard/manifest.json" > /dev/null
  jq -r '.artifacts[]|[.path,.sha256,(.bytes|tostring)]|@tsv' "$output/export-$shard/manifest.json" |
  while IFS=$'\t' read -r relative digest bytes; do
    [[ "$relative" != /* && "$relative" != *..* ]]
    printf '%s  %s\n' "$digest" "$output/export-$shard/$relative" | sha256sum -c - >> "$output/export-artifact-hashes.txt"
    [[ $(stat -c %s "$output/export-$shard/$relative") == "$bytes" ]]
  done
done
phase=negative_control
if "$output/crypto" dataset verify --out "$input/tampered-copy" > "$output/negative.stdout.json" 2> "$output/negative.stderr.log"; then
  echo 'retained semantic corruption was unexpectedly accepted' >&2
  exit 1
fi
rg -q 'prime F4 chain input differs at domains' "$output/negative.stderr.log"
negative_rejected=true
commands_passed=$((commands_passed+1))
phase=input_immutability
(cd "$input" && find . -type f -print0 | sort -z | xargs -0 sha256sum) > "$output/input-after.sha256"
cmp "$output/input-before.sha256" "$output/input-after.sha256"
phase=completed
