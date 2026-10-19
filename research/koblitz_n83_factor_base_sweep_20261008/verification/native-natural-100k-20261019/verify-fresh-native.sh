#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-100k-20261019
raw="$work/publication/fresh-extract/raw-bundle"
proof=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-100k-20261019
mkdir -p "$proof/fresh-replay"
for id in k2048-v6c k4096-v6c; do
 "$raw/bin/runner-analysis-v5" audit "$raw/runs/$id" > "$proof/fresh-replay/$id.json"
 jq -S . "$raw/runs/$id/audit.json" > "$work/publication/original-audit.json"
 jq -S . "$proof/fresh-replay/$id.json" > "$work/publication/fresh-audit.json"
 cmp "$work/publication/original-audit.json" "$work/publication/fresh-audit.json"
done
cmp /home/ec2-user/scratch/n83-xor-opt-20261019/candidate-target/release/examples/koblitz_n83_factor_base_export "$raw/bin/indexed-natural"
for panel in fixtures k2048 k4096; do
 diff -qr "/home/ec2-user/scratch/n83-docker-sweep-20261018/inputs/$panel" "$raw/inputs/$panel"
done
printf '%s\n' 'Both original native audits reproduce from fresh downloaded records. Original/fresh worker and input panels compare byte-for-byte.' > "$proof/fresh-path-byte-checks.txt"
