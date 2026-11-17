#!/usr/bin/env bash
set -euo pipefail
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
pub="$work/publication"
export AWS_DEFAULT_REGION=us-east-2 CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store CRYPTO_STORE_INDEX="$pub/sidecar-store/index.json"
mkdir "$pub/sidecar-store"
name=evidence/n83/allocation-profile-20261021/verification.tar.xz
/home/ec2-user/scratch/n83-docker-sweep-20261018/integration-target/release/ecbench store push "$pub/verification.tar.xz" --name "$name" --description 'Native N83 ownership raw publication proofs, positive/negative downloaded replay and source history' --provenance 'Raw03754a1f; exact674d/7e8ae96c experiment source; post-raw-publication source bundle' --json > "$pub/sidecar-push.json"
digest=$(jq -r --arg name "$name" '.entries[$name].sha256' "$pub/sidecar-store/index.json")
key="store/sha256/${digest:0:2}/$digest"
aws s3api head-object --bucket setec-astronomy-artifacts --key "$key" > "$pub/sidecar-head.json"
aws s3api get-object --bucket setec-astronomy-artifacts --key "$key" "$pub/sidecar-download.tar.xz" > "$pub/sidecar-get.json"
cmp "$pub/sidecar-download.tar.xz" "$pub/verification.tar.xz"
mkdir "$pub/sidecar-fresh"
tar -xJf "$pub/sidecar-download.tar.xz" -C "$pub/sidecar-fresh"
(cd "$pub/sidecar-fresh/verification"; sha256sum -c SHA256SUMS) > "$pub/sidecar-payload-checks.txt"
git -C /home/ec2-user/crypto-n83-resume bundle verify "$pub/sidecar-fresh/verification/SOURCE-PUBLICATION.bundle" > "$pub/sidecar-source-bundle.txt" 2>&1
git -C /home/ec2-user/crypto-n83-resume bundle verify "$pub/sidecar-fresh/verification/proof/SOURCE-COMMITS.bundle" > "$pub/sidecar-experiment-bundle.txt" 2>&1
(cd "$pub/sidecar-fresh"; tar --sort=name --mtime='2026-10-21 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - verification | xz -6 -T1) > "$pub/sidecar-repacked.tar.xz"
cmp "$pub/sidecar-repacked.tar.xz" "$pub/sidecar-download.tar.xz"
printf '%s\n' PASS_SIDECAR_PUT_HEAD_FRESH_GET_ALL_PAYLOAD_BOTH_SOURCE_BUNDLES_AND_REPACK > "$pub/sidecar-status.txt"
