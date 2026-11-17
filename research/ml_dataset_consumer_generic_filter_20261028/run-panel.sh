#!/usr/bin/env bash
set -euo pipefail
panel=${1:?primary or supplement}
case "$panel" in primary|supplement) ;; *) exit 2;; esac
repo_root=$(git rev-parse --show-toplevel)
design="$repo_root/research/ml_dataset_consumer_generic_filter_20261028"
output=/home/ec2-user/ml-dataset-resume-20261028/consumer-generic-filter-validation
producer=/home/ec2-user/ml-dataset-resume-20261028/generic-filter-trace-validation/crypto-source-bound
expected=968a316daab4084ef61fd46c779832b9ebd9e482e58492674bc82174c4b1cfb7
actual=$(sha256sum "$producer" | cut -d' ' -f1)
[[ "$actual" = "$expected" ]]
sha256sum --check "$design/FROZEN_INPUTS.sha256" > "$output/frozen-inputs-check-$panel.log"
if [[ "$panel" = supplement ]]; then [[ -s "$output/supplement-coverage-gate.json" ]]; fi
mkdir -p "$output/panel"
for spec in "$design"/frozen/"$panel"-*.spec.json; do
 name=$(jq -r .name "$spec")
 cell_dir="$output/panel/$name"
 [[ ! -e "$cell_dir" ]]
 mkdir "$cell_dir"
 sha256sum "$producer" "$spec" "$design/frozen/$name.json" "$design/frozen/$name.plan.json" "$design/frozen/protocol.json" > "$cell_dir/inputs.sha256"
 "$producer" dataset preflight --plan "$design/frozen/$name.plan.json" --protocol "$design/frozen/protocol.json" > "$cell_dir/preflight.json" 2> "$cell_dir/preflight.stderr"
 shard=$(jq -r .shard_index "$spec")
 started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
 set +e
 "$producer" dataset generate --plan "$design/frozen/$name.plan.json" --protocol "$design/frozen/protocol.json" --out "$cell_dir/run" --max-cases 1 --shard-count 96 --shard-index "$shard" --seconds-per-case 30 --memory-mib 1024 --trace-mib 128 > "$cell_dir/generate.stdout" 2> "$cell_dir/generate.stderr"
 code=$?
 set -e
 ended=$(date -u +%Y-%m-%dT%H:%M:%SZ)
 jq -n --arg name "$name" --arg started "$started" --arg ended "$ended" --argjson code "$code" --arg producer "$expected" '{name:$name,started_at:$started,ended_at:$ended,exit_code:$code,producer_sha256:$producer,scientific_observation:false,retried:false}' > "$cell_dir/receipt.json"
 manifest=$(printf '%s/run/cells/%012d/manifest.json' "$cell_dir" "$shard")
 if [[ -f "$manifest" ]]; then
   jq -S '.config' "$manifest" > "$cell_dir/actual-config.json"
   jq -S '.' "$design/frozen/$name.json" > "$cell_dir/expected-config.json"
   cmp "$cell_dir/actual-config.json" "$cell_dir/expected-config.json"
 fi
 jq -c . "$cell_dir/receipt.json"
done
