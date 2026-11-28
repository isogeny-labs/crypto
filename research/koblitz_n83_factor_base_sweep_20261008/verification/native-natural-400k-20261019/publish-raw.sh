#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-400k-20261019
proof=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-400k-20261019
store=/home/ec2-user/scratch/n83-docker-sweep-20261018/integration-target/release/ecbench
export AWS_DEFAULT_REGION=us-east-2
export CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store
export CRYPTO_STORE_INDEX="$proof/store/index.json"
mkdir -p "$proof/store"
"$store" store push "$work/publication/runs.tar.xz" --name evidence/n83/natural-400k-20261019/runs.tar.xz --description 'Complete raw native N83 K2048/K4096 400000-conflict continuation and every actual attempt including the assessed K4096 inspection failure' --provenance 'Frozen guard d216680177 and separately assessed inspection correction cad91042bf; unchanged retained worker; native exact cards and deterministic collection' --json > "$proof/push.json"
digest=$(jq -r '.entries["evidence/n83/natural-400k-20261019/runs.tar.xz"].sha256' "$CRYPTO_STORE_INDEX")
key="store/sha256/${digest:0:2}/$digest"
aws s3api head-object --bucket setec-astronomy-artifacts --key "$key" > "$proof/s3-head.json"
aws s3api get-object --bucket setec-astronomy-artifacts --key "$key" "$work/publication/download.tar.xz" > "$proof/s3-get.json"
sha256sum "$work/publication/download.tar.xz" > "$proof/download.sha256"
cmp "$work/publication/runs.tar.xz" "$work/publication/download.tar.xz"
mkdir "$work/publication/fresh-extract"
tar -xJf "$work/publication/download.tar.xz" -C "$work/publication/fresh-extract"
(cd "$work/publication/fresh-extract/raw-bundle"; sha256sum -c SHA256SUMS) > "$proof/download-file-checks.log"
sha256sum "$work/publication/fresh-extract/raw-bundle/SHA256SUMS" > "$proof/download-manifest.sha256"
cmp "$work/publication/raw-bundle/SHA256SUMS" "$work/publication/fresh-extract/raw-bundle/SHA256SUMS"
cp "$work/publication/raw-bundle/SHA256SUMS" "$proof/payloads.sha256"
bash "$work/verify-fresh-native.sh" > "$proof/fresh-replay.log" 2>&1
(cd "$work/publication/fresh-extract"; tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 --numeric-owner -cf - raw-bundle | xz -6 -T1 > "$work/publication/repacked.tar.xz")
sha256sum "$work/publication/repacked.tar.xz" > "$proof/repacked.sha256"
cmp "$work/publication/download.tar.xz" "$work/publication/repacked.tar.xz"
printf '%s\n' 'PASS: full archive bytes, payload manifest, every payload SHA256, downloaded-record native replay, ELF/panel bytes and deterministic fresh-extraction repack.' > "$proof/verification-status.txt"
