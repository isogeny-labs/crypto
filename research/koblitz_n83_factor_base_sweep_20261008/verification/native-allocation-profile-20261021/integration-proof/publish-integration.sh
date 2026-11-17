#!/usr/bin/env bash
set -euo pipefail
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
pub="$work/integration-publication"
export AWS_DEFAULT_REGION=us-east-2 CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store CRYPTO_STORE_INDEX="$pub/store/index.json"
mkdir "$pub/store"
name=evidence/n83/allocation-profile-20261021/integration-verification.tar.xz
/home/ec2-user/scratch/n83-docker-sweep-20261018/integration-target/release/ecbench store push "$pub/integration-verification.tar.xz" --name "$name" --description 'Combined PR2036 main integration source and full native release validation, preserving documentary N83 ownership evidence' --provenance 'Source d5aac8e9; prerequisite remote05ec85b5; no new SAT worker/cap; raw03754a1f and sidecarfdf07b85 unchanged' --json > "$pub/push.json"
digest=$(jq -r --arg name "$name" '.entries[$name].sha256' "$pub/store/index.json")
key="store/sha256/${digest:0:2}/$digest"
aws s3api head-object --bucket setec-astronomy-artifacts --key "$key" > "$pub/head.json"
aws s3api get-object --bucket setec-astronomy-artifacts --key "$key" "$pub/download.tar.xz" > "$pub/get.json"
cmp "$pub/download.tar.xz" "$pub/integration-verification.tar.xz"
mkdir "$pub/fresh"
tar -xJf "$pub/download.tar.xz" -C "$pub/fresh"
(cd "$pub/fresh/integration"; sha256sum -c SHA256SUMS) > "$pub/payload-checks.txt"
git -C /home/ec2-user/crypto-n83-resume bundle verify "$pub/fresh/integration/SOURCE-INTEGRATED.bundle" > "$pub/source-bundle.txt" 2>&1
cmp "$pub/fresh/integration/integrated-validation/status.txt" "$work/integrated-validation/status.txt"
(cd "$pub/fresh"; tar --sort=name --mtime='2026-10-21 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - integration | xz -6 -T1) > "$pub/repacked.tar.xz"
cmp "$pub/repacked.tar.xz" "$pub/download.tar.xz"
printf '%s\n' PASS_INTEGRATION_PUT_HEAD_FRESH_GET_ALL_PAYLOAD_SOURCE_BUNDLE_VALIDATION_LOGS_AND_REPACK > "$pub/status.txt"
