#!/usr/bin/env bash
set -euo pipefail
work=/home/ec2-user/scratch/n83-natural-1m-20261020
pub="$work/publication"
name=evidence/n83/natural-1m-20261020/runs.tar.xz
export AWS_DEFAULT_REGION=us-east-2 CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store CRYPTO_STORE_INDEX="$pub/store/index.json"
mkdir "$pub/store"
/home/ec2-user/scratch/n83-docker-sweep-20261018/integration-target/release/ecbench store push "$pub/runs.tar.xz" --name "$name" --description 'Seven native N83 1M attempts, all available raw evidence, final K2048/K4096 replay and memory profile PASS, original failures retained' --provenance 'Worker unchanged; final controller66ab009b; protocols SCOPE-V20; seal-final.sh' --json > "$pub/push.json"
digest=$(jq -r --arg name "$name" '.entries[$name].sha256' "$pub/store/index.json")
key="store/sha256/${digest:0:2}/$digest"
aws s3api head-object --bucket setec-astronomy-artifacts --key "$key" > "$pub/head.json"
aws s3api get-object --bucket setec-astronomy-artifacts --key "$key" "$pub/download.tar.xz" > "$pub/get.json"
cmp "$pub/download.tar.xz" "$pub/runs.tar.xz"
sha256sum "$pub/download.tar.xz" > "$pub/download.sha256"
sudo -n mkdir "$pub/fresh-extract"
sudo -n tar -xJf "$pub/download.tar.xz" -C "$pub/fresh-extract"
raw="$pub/fresh-extract/runs"
sudo -n bash -c 'cd /home/ec2-user/scratch/n83-natural-1m-20261020/publication/fresh-extract/runs && sha256sum -c SHA256SUMS' > "$pub/payload-checks.txt"
mkdir "$pub/replay"
for id in k2048-v20 k4096-v20; do
 sudo -n "$raw/bin/runner-v20" audit "$raw/runs/$id" > "$pub/replay/$id.audit.json"
 sudo -n jq -S . "$raw/runs/$id/audit.json" > "$pub/replay/$id.original.json"
 jq -S . "$pub/replay/$id.audit.json" > "$pub/replay/$id.fresh.json"
 cmp "$pub/replay/$id.original.json" "$pub/replay/$id.fresh.json"
 sudo -n "$raw/bin/runner-v20" summarize-memory "$raw/runs/$id" "$pub/replay/$id.memory.json"
 sudo -n jq -S 'del(.dir)' "$raw/validation/$id.memory-summary.json" > "$pub/replay/$id.memory-original.json"
 sudo -n jq -S 'del(.dir)' "$pub/replay/$id.memory.json" > "$pub/replay/$id.memory-fresh.json"
 cmp "$pub/replay/$id.memory-original.json" "$pub/replay/$id.memory-fresh.json"
 sudo -n cmp "$raw/validation/live-capture/$id.stdout" "$raw/runs/$id/stdout.log"
 sudo -n cmp "$raw/validation/live-capture/$id.stderr" "$raw/runs/$id/stderr.log"
done
sudo -n "$raw/bin/runner-v15" audit "$raw/runs/k4096-v16" > "$pub/replay/historical-native-audit.json"
sudo -n jq -S . "$raw/runs/k4096-v16/audit.json" > "$pub/replay/historical-original.json"
jq -S . "$pub/replay/historical-native-audit.json" > "$pub/replay/historical-fresh.json"
cmp "$pub/replay/historical-original.json" "$pub/replay/historical-fresh.json"
for id in k4096-v16 k2048-v19; do
 set +e
 sudo -n "$raw/bin/runner-v18" summarize-memory "$raw/runs/$id" "$pub/replay/$id.rejected.json" > "$pub/replay/$id.stdout" 2> "$pub/replay/$id.stderr"
 rc=$?
 set -e
 test "$rc" -ne 0
 printf '%s\n' "$rc" > "$pub/replay/$id.exit"
 sudo -n jq -e '.status=="REJECTED_MEMORY_PROFILE" and .admitted==false' "$pub/replay/$id.rejected.json" > "$pub/replay/$id.check.txt"
done
sudo -n "$raw/bin/runner-v18" summarize-memory "$raw/runs/k2048-v16" "$pub/replay/interrupted-prefix.json"
sudo -n jq -e '.status=="PREFIX_ONLY_MISSING_TERMINAL_CUSTODY_NOT_ADMITTED" and .n_samples==5' "$pub/replay/interrupted-prefix.json" > "$pub/replay/prefix-check.txt"
sudo -n "$raw/bin/runner-v19" check-sat-trace "$raw/runs/k2048-v19/failure-stderr.log" > "$pub/replay/k2048-v19.sat-prefix.json"
sudo -n cmp "$pub/replay/k2048-v19.sat-prefix.json" "$raw/validation/k2048-v19.sat-prefix.json"
sudo -n cmp "$raw/bin/indexed-natural" /home/ec2-user/scratch/n83-xor-opt-20261019/candidate-target/release/examples/koblitz_n83_factor_base_export
for id in k2048 k4096; do
 sudo -n diff -qr "$raw/inputs/$id" "/home/ec2-user/scratch/n83-docker-sweep-20261018/inputs/$id" > "$pub/replay/$id.input-diff.txt"
done
sudo -n bash -c 'cd /home/ec2-user/scratch/n83-natural-1m-20261020/publication/fresh-extract && tar --sort=name --mtime="2026-10-21 00:00:00Z" --owner=0 --group=0 --numeric-owner --mode="u+rwX,go+rX,go-w" --format=gnu -cf - runs | xz -6 -T1' > "$pub/repacked.tar.xz"
cmp "$pub/repacked.tar.xz" "$pub/download.tar.xz"
printf '%s\n' PASS_S3_BYTES_ALL_PAYLOAD_CURRENT_NATIVE_AUDITS_PROFILES_HISTORICAL_REPLAY_REJECTIONS_PREFIX_INPUTS_ELF_AND_REPACK > "$pub/status.txt"
