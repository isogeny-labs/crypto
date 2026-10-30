#!/bin/bash
set -euo pipefail
root=/home/ec2-user/crypto-ml-algebraic-targets-20261030
scratch=/home/ec2-user/ml-dataset-work-20261028
export AWS_CONFIG_FILE=/tmp/ml-dataset-aws-config-20261028 AWS_PROFILE=ml-dataset-checkpoint AWS_DEFAULT_REGION=us-east-2
export CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store
export CRYPTO_STORE_INDEX=$root/research/ml_dataset_algebraic_targets_20261030/store-index.json
for file in "$@"; do
 name=$(basename "$file")
 "$scratch/graph-export-crypto" store --json push "$file" --name "evidence/ml-dataset-algebraic-targets-20261030/$name" --description 'Retained consequence exporter software validation artifact; not generated research corpus' --provenance 'Native polynomial-consequence validation attempt; source-check and command outcomes are recorded in its phase receipt' > "$scratch/$name.store-receipt.json"
 sha=$(sha256sum "$file" | cut -d' ' -f1)
 download=$scratch/algebraic-s3-readback-$sha
 aws s3api get-object --bucket setec-astronomy-artifacts --key "store/sha256/${sha:0:2}/$sha" "$download" > "$scratch/$name.get-receipt.json"
 cmp "$file" "$download"
 jq -n --arg file "$name" --arg sha "$sha" --arg uri "s3://setec-astronomy-artifacts/store/sha256/${sha:0:2}/$sha" --argjson bytes "$(stat -c %s "$download")" '{file:$file,sha256:$sha,uri:$uri,bytes:$bytes,fresh_download_byte_comparison:true}' > "$scratch/$name.readback.json"
done
