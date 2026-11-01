#!/usr/bin/env bash
# Native CLI software fixture only; no publication, cloud credentials or retries.
set -euo pipefail
binary=$(realpath "$1")
expected_sha=$2
source_commit=$3
output=$4
fixture_dir=$(cd -- "$(dirname -- "$0")" && pwd)
[[ $expected_sha =~ ^[0-9a-f]{64}$ && $source_commit =~ ^[0-9a-f]{40}$ ]]
[[ ! -e $output ]]
[[ $(stat -f -c %T "$(dirname "$output")") == xfs ]]
mkdir -p "$output"
output=$(realpath "$output")
cp "$fixture_dir/cofactor-prime-protocol.json" "$output/protocol.json"
printf '%s\n' "$source_commit" > "$output/expected-source.txt"
printf '%s\n' "$expected_sha" > "$output/expected-binary-sha256.txt"
command_count=0
finish() {
  result=$?
  trap - EXIT
  jq -n --argjson exit_code "$result" --argjson commands "$command_count" \
    --arg source "$source_commit" --arg binary "$expected_sha" \
    '{schema:"ml-dataset.root-integration-cli-software/v1",exit_code:$exit_code,commands:$commands,source_commit:$source,binary_sha256:$binary,scientific_corpus_rows:0,verified_cloud_cells:0}' > "$output/receipt.json"
  exit "$result"
}
trap finish EXIT
run() {
  local name=$1
  shift
  [[ $(sha256sum "$binary" | cut -d ' ' -f1) == "$expected_sha" ]]
  command_count=$((command_count+1))
  printf '%s\0' "$binary" "$@" > "$output/$name.argv0"
  date -u +%FT%TZ > "$output/$name.started"
  local code=0
  env -i PATH="$PATH" HOME="$HOME" LANG=C RAYON_NUM_THREADS=1 \
    timeout --signal=TERM --kill-after=5s 45s "$binary" "$@" \
    > "$output/$name.stdout" 2> "$output/$name.stderr" || code=$?
  printf '%s\n' "$code" > "$output/$name.exit"
  date -u +%FT%TZ > "$output/$name.finished"
  [[ $(sha256sum "$binary" | cut -d ' ' -f1) == "$expected_sha" ]]
  return "$code"
}
protocol_sha=$(sha256sum "$output/protocol.json" | cut -d ' ' -f1)
run plan dataset plan --out "$output/plan.json" --n 7 --m 2 --families prime --solvers root --curve-seeds 8 --generators 2 --keys 2 --seed 42 --factor-base-size 2 --degree-bound 4 --work-limit 50000 --protocol-sha256 "$protocol_sha"
run preflight dataset preflight --plan "$output/plan.json" --protocol "$output/protocol.json"
jq -e --arg source "$source_commit" --arg binary "$expected_sha" '.source_commit == $source and .binary_sha256 == $binary and .source_dirty == false' "$output/preflight.stdout" > "$output/preflight-binding-check.txt"
for shard in 62 63; do
  run "generate-$shard" dataset generate --plan "$output/plan.json" --protocol "$output/protocol.json" --out "$output/shard-$shard" --max-cases 1 --shard-index "$shard" --shard-count 64 --seconds-per-case 20 --memory-mib 512 --trace-mib 16
  run "verify-$shard" dataset verify --out "$output/shard-$shard"
  run "status-$shard" dataset status --out "$output/shard-$shard"
  run "export-$shard" dataset export --out "$output/shard-$shard" --destination "$output/export-$shard"
  cell=$(printf '%012d' "$shard")
  jq -e --argjson twist "$((shard%2))" '.config.solver == "root" and .config.m == 2 and .config.twist == ($twist == 1) and ([.artifacts[] | select(.role == "prime-root-certificates/v1")] | length) == 1' \
    "$output/shard-$shard/cells/$cell/manifest.json" > "$output/certificate-$shard.check"
  jq -s -e 'length == 1 and ([.[0].source_artifacts[] | select(.role == "prime-root-certificates/v1")] | length) == 1' \
    "$output/export-$shard/restricted/index.jsonl" > "$output/export-retention-$shard.check"
  # Representative leakage assertions check the merged allowlisted exporter.
  jq -s -e 'all(.[]; ([.. | objects | keys[] | select(. == "root_certificate_scope" or . == "queries" or . == "events" or . == "known_answer_scalar" or . == "found_pdp")] | length) == 0)' \
    "$output/export-$shard/features.jsonl" > "$output/public-root-exclusion-$shard.check"
done
