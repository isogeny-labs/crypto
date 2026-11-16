#!/usr/bin/env bash
# Draft software fixture. No cloud calls, retries, or success resampling.
set -euo pipefail
binary=$(realpath "$1"); expected_sha=$2; source_commit=$3; output=$4
fixture_dir=$(cd -- "$(dirname -- "$0")" && pwd)
protocol="$fixture_dir/wide-prime-protocol.json"
[[ $expected_sha =~ ^[0-9a-f]{64}$ && $source_commit =~ ^[0-9a-f]{40}$ ]]
jq -e --arg s "$source_commit" --arg b "$expected_sha" '.freeze.status=="frozen" and .software_binding.source==$s and .software_binding.binary_sha256==$b and .adapter_contract_confirmed==true and .controls.source_bound_tests_passed==true' "$protocol" >/dev/null
[[ $(sha256sum "$0" | cut -d ' ' -f1) == $(jq -r .runner_sha256 "$protocol") ]]
[[ $(sha256sum "$fixture_dir/check-wide-cell.sh" | cut -d ' ' -f1) == $(jq -r .checker_sha256 "$protocol") ]]
body_sha=$(jq -cSj 'del(.freeze)' "$protocol" | sha256sum | cut -d ' ' -f1)
[[ $body_sha == $(jq -r .freeze.protocol_sha256 "$protocol") ]]
output_parent=$(realpath -e -- "$(dirname -- "$output")")
[[ $output_parent == /var/tmp || $output_parent == /var/tmp/* ]]
[[ $(stat -f -c %T -- "$output_parent") == xfs ]]
output="$output_parent/$(basename -- "$output")"
[[ ! -e $output && ! -L $output ]]
mkdir "$output"
cp "$protocol" "$output/protocol.json"
stat -f -c %T -- "$output" > "$output/output-filesystem.txt"
command_count=0
started=$SECONDS
finish() {
  result=$?; trap - EXIT
  jq -n --argjson exit_code "$result" --argjson commands "$command_count" --arg source "$source_commit" --arg binary "$expected_sha" '{schema:"ml-dataset.wide-prime-cli-software/v1",exit_code:$exit_code,commands:$commands,source_commit:$source,binary_sha256:$binary,scientific_corpus_rows:0,verified_cloud_cells:0}' > "$output/receipt.json"
  exit "$result"
}
trap finish EXIT
run() {
  local name=$1; shift
  local remaining=$((1800-SECONDS+started))
  ((remaining>5))
  local limit=45
  ((remaining>=50)) || limit=$((remaining-5))
  [[ $(sha256sum "$binary" | cut -d ' ' -f1) == "$expected_sha" ]]
  command_count=$((command_count+1))
  printf '%s\0' "$binary" "$@" > "$output/$name.argv0"
  date -u +%FT%TZ > "$output/$name.started"
  local code=0
  env -i PATH="$PATH" HOME="$HOME" LANG=C RAYON_NUM_THREADS=1 timeout --signal=TERM --kill-after=5s "${limit}s" "$binary" "$@" > "$output/$name.stdout" 2> "$output/$name.stderr" || code=$?
  printf '%s\n' "$code" > "$output/$name.exit"
  date -u +%FT%TZ > "$output/$name.finished"
  [[ $(sha256sum "$binary" | cut -d ' ' -f1) == "$expected_sha" ]]
  return "$code"
}
protocol_sha=$(sha256sum "$protocol" | cut -d ' ' -f1)
run plan dataset plan --out "$output/plan.json" --n 47,53,61 --m 2 --families prime --solvers f4 --curve-seeds 1 --generators 1 --keys 1 --seed 42 --factor-base-size 2 --degree-bound 4 --work-limit 50000 --protocol-sha256 "$protocol_sha"
run preflight dataset preflight --plan "$output/plan.json" --protocol "$protocol"
jq -e --arg s "$source_commit" --arg b "$expected_sha" '.source_commit==$s and .binary_sha256==$b and .source_dirty==false' "$output/preflight.stdout" > "$output/preflight-binding.check"
for shard in 0 1 2 3 4 5; do
  run "generate-$shard" dataset generate --plan "$output/plan.json" --protocol "$protocol" --out "$output/shard-$shard" --max-cases 1 --shard-index "$shard" --shard-count 6 --seconds-per-case 20 --memory-mib 512 --trace-mib 16
  run "verify-$shard" dataset verify --out "$output/shard-$shard"
  run "status-$shard" dataset status --out "$output/shard-$shard"
  run "export-$shard" dataset export --out "$output/shard-$shard" --destination "$output/export-$shard"
  run "report-$shard" dataset report --out "$output/shard-$shard" --destination "$output/report-$shard"
  cell=$(printf '%012d' "$shard")
  dir="$output/shard-$shard/cells/$cell"
  sizes=(47 53 61)
  jq -e --argjson n "${sizes[$((shard/2))]}" --argjson twist "$((shard%2))" '.config.family=="prime" and .config.n==$n and .config.m==2 and .config.solver=="f4" and .config.twist==($twist==1)' "$dir/manifest.json" > "$output/axes-$shard.check"
  if ((shard>=4)); then
    jq -e '.field.modulus=="2305843009213693951"' "$dir/public.json" > "$output/mersenne-$shard.check"
  fi
  bash "$fixture_dir/check-wide-cell.sh" "$dir" "$output/export-$shard" "$protocol" > "$output/wide-$shard.check"
done
[[ $command_count == 32 ]]
