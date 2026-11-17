#!/bin/bash
# Thin native CLI orchestration and serialization/inventory checks only.
set -euo pipefail
binary=$(realpath "$1")
output=$2
source_root=/home/ec2-user/ml-dataset-work-20261028/startup-main-cli-fixture-run
[[ ! -e "$output" ]]
mkdir -p "$output"
printf '%s  %s\n' e3f0d56284a7c32c90ead0f0ad9049801c809b2362f8c8caf07e2baf064e9d3f "$source_root/receipt.json" | sha256sum -c - > "$output/input-receipt-check.txt"
sha256sum "$binary" > "$output/binary.sha256"
(cd "$source_root" && find . -type f -print0 | sort -z | xargs -0 sha256sum) > "$output/input-inventory-before.sha256"
for shard in 62 63; do
  source_shard=$source_root/shard-$shard
  destination=$output/export-$shard
  "$binary" dataset export --out "$source_shard" --destination "$destination" > "$output/export-$shard.stdout.json" 2> "$output/export-$shard.stderr.log"
  "$binary" dataset export --out "$source_shard" --destination "$destination" > "$output/retry-$shard.stdout.json" 2> "$output/retry-$shard.stderr.log"
  cmp "$output/export-$shard.stdout.json" "$output/retry-$shard.stdout.json"
  jq -e '.feature_policy.version=="strict-structural-allowlist-v5" and .feature_policy.model_input_files==["features.jsonl","prefix-features.jsonl"]' "$destination/manifest.json" > /dev/null
  jq -r '.artifacts[] | [.path,.sha256,(.bytes|tostring)] | @tsv' "$destination/manifest.json" |
  while IFS=$'\t' read -r relative expected_sha expected_bytes; do
    [[ "$relative" != /* && "$relative" != *..* ]]
    printf '%s  %s\n' "$expected_sha" "$destination/$relative" | sha256sum -c - >> "$output/artifact-readback.txt"
    [[ $(stat -c %s "$destination/$relative") == "$expected_bytes" ]]
  done
  while IFS= read -r row; do
    expected_id=$(jq -r .feature_id <<< "$row")
    canonical=$(jq -cS .inputs <<< "$row")
    actual_id=$(printf %s "$canonical" | sha256sum | cut -d' ' -f1)
    [[ "$actual_id" == "$expected_id" ]]
    jq -e --arg id "$expected_id" 'select(.feature_id==$id and .record.kind=="observed_next_branch")' "$destination/restricted/prefix-records.jsonl" > /dev/null
  done < "$destination/prefix-features.jsonl"
done
jq -se '[.[]|.prefix_feature_occurrences]|add>0' "$output"/export-{62,63}/manifest.json > /dev/null
(cd "$source_root" && find . -type f -print0 | sort -z | xargs -0 sha256sum) > "$output/input-inventory-after.sha256"
cmp "$output/input-inventory-before.sha256" "$output/input-inventory-after.sha256"
jq -n '{schema:"software-cli-fixture-result/v1",commands_passed:4,source_unchanged:true,artifact_readback_passed:true,prefix_identity_and_label_joins_passed:true,scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$output/result.json"
cat "$output/result.json"
