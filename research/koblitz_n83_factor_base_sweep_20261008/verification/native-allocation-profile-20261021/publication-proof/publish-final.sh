#!/usr/bin/env bash
set -euo pipefail
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
pub="$work/publication"
export AWS_DEFAULT_REGION=us-east-2 CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store CRYPTO_STORE_INDEX="$pub/store/index.json"
mkdir "$pub/store"
name=evidence/n83/allocation-profile-20261021/runs.tar.xz
/home/ec2-user/scratch/n83-docker-sweep-20261018/integration-target/release/ecbench store push "$pub/runs.tar.xz" --name "$name" --description 'Both 450K N83 native SAT ownership profiles with all failures, full raw traces/receipts/ELFs/inputs, source and inspected report' --provenance 'Worker674d; queued guard7e8ae96c; original1M arms retained separately; no reclamation or speed claim' --json > "$pub/push.json"
digest=$(jq -r --arg name "$name" '.entries[$name].sha256' "$pub/store/index.json")
key="store/sha256/${digest:0:2}/$digest"
aws s3api head-object --bucket setec-astronomy-artifacts --key "$key" > "$pub/head.json"
aws s3api get-object --bucket setec-astronomy-artifacts --key "$key" "$pub/download.tar.xz" > "$pub/get.json"
cmp "$pub/download.tar.xz" "$pub/runs.tar.xz"
sha256sum "$pub/download.tar.xz" > "$pub/download.sha256"
mkdir "$pub/fresh" "$pub/replay"
tar -xJf "$pub/download.tar.xz" -C "$pub/fresh"
raw="$pub/fresh/runs"
(cd "$raw"; sha256sum -c SHA256SUMS) > "$pub/payload-checks.txt"
git -C /home/ec2-user/crypto-n83-resume bundle verify "$raw/SOURCE.bundle" > "$pub/replay/source-bundle.txt" 2>&1
for arm in k2048-p22 k4096-p22; do
 if test "$arm" = k2048-p22; then runner="$raw/bin/runner-v21"; ref=k2048-v20; else runner="$raw/bin/runner-v22"; ref=k4096-v20; fi
 run="$raw/exec/$arm"
 sudo -n "$runner" audit "$run" > "$pub/replay/$arm.audit.json"
 sudo -n "$runner" summarize-memory "$run" "$pub/replay/$arm.memory.json"
 sudo -n "$runner" summarize-storage "$run" "$raw/reference/$ref" "$pub/replay/$arm.storage.json"
 jq -S . "$pub/replay/$arm.audit.json" > "$pub/replay/$arm.audit-fresh.json"
 jq -S . "$raw/analysis/$arm.audit.json" > "$pub/replay/$arm.audit-original.json"
 cmp "$pub/replay/$arm.audit-fresh.json" "$pub/replay/$arm.audit-original.json"
 for kind in memory storage; do
  sudo -n jq -S 'del(.dir,.reference_dir)' "$pub/replay/$arm.$kind.json" > "$pub/replay/$arm.$kind-fresh.json"
  jq -S 'del(.dir,.reference_dir)' "$raw/analysis/$arm.$kind.json" > "$pub/replay/$arm.$kind-original.json"
  cmp "$pub/replay/$arm.$kind-fresh.json" "$pub/replay/$arm.$kind-original.json"
 done
 cmp "$raw/live-capture/$arm.stdout" "$run/stdout.log"
 cmp "$raw/live-capture/$arm.stderr" "$run/stderr.log"
done
set +e
sudo -n "$raw/bin/runner-v22" summarize-storage "$raw/reference/k2048-v20" "$raw/reference/k2048-v20" "$pub/replay/old-storage.json" > "$pub/replay/missing-storage.stdout" 2> "$pub/replay/missing-storage.stderr"
rc=$?
set -e
printf '%s\n' "$rc" > "$pub/replay/missing-storage.exit"
test "$rc" -ne 0
test ! -e "$pub/replay/old-storage.json"
rg -q 'storage sampling coverage differs' "$pub/replay/missing-storage.stderr"
cp -a "$raw/exec/k4096-p22" "$pub/replay/tampered-queue"
jq '.elapsed_seconds=600' "$pub/replay/tampered-queue/lock-wait.json" > "$pub/replay/tampered-queue/changed.json"
mv "$pub/replay/tampered-queue/changed.json" "$pub/replay/tampered-queue/lock-wait.json"
set +e
sudo -n "$raw/bin/runner-v22" audit "$pub/replay/tampered-queue" > "$pub/replay/tampered-queue.stdout" 2> "$pub/replay/tampered-queue.stderr"
rc=$?
set -e
printf '%s\n' "$rc" > "$pub/replay/tampered-queue.exit"
test "$rc" -ne 0
rg -q 'lock queue custody/policy differs' "$pub/replay/tampered-queue.stderr"
for binary in native-storage-baseline runner-v21 runner-v22; do cmp "$raw/bin/$binary" "$work/bin/$binary"; done
cmp "$raw/bin/retained-worker" /home/ec2-user/scratch/n83-xor-opt-20261019/candidate-target/release/examples/koblitz_n83_factor_base_export
for id in fixtures k2048 k4096; do diff -qr "$raw/inputs/$id" "/home/ec2-user/scratch/n83-docker-sweep-20261018/inputs/$id" > "$pub/replay/$id-input-diff.txt"; done
(cd "$pub/fresh"; tar --sort=name --mtime='2026-10-21 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1) > "$pub/repacked.tar.xz"
cmp "$pub/repacked.tar.xz" "$pub/download.tar.xz"
printf '%s\n' PASS_FRESH_PUT_HEAD_GET_ALL_PAYLOAD_BUNDLE_BOTH_NATIVE_AUDITS_MEMORY_STORAGE_PREFIX_NEGATIVES_INPUTS_ELFS_AND_REPACK > "$pub/status.txt"
