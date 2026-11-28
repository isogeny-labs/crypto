#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-200k-20261019
raw="$work/publication/fresh-extract/raw-bundle"
proof=research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-200k-20261019
mkdir -p "$proof/fresh-replay"
for id in k2048-v8b k4096-v8; do
 "$raw/bin/runner-v8" audit "$raw/runs/$id" > "$proof/fresh-replay/$id.json"
 jq -S . "$raw/runs/$id/audit.json" > "$work/publication/original-audit.json"
 jq -S . "$proof/fresh-replay/$id.json" > "$work/publication/fresh-audit.json"
 cmp "$work/publication/original-audit.json" "$work/publication/fresh-audit.json"
done
cmp /home/ec2-user/scratch/n83-xor-opt-20261019/candidate-target/release/examples/koblitz_n83_factor_base_export "$raw/bin/indexed-natural"
cmp "$work/bin/runner-v7" "$raw/bin/runner-v7"
cmp "$work/bin/runner-v8" "$raw/bin/runner-v8"
for panel in fixtures k2048 k4096; do
 diff -qr "/home/ec2-user/scratch/n83-docker-sweep-20261018/inputs/$panel" "$raw/inputs/$panel"
done
printf '%s\n' 'Both original native audits reproduce from freshly downloaded records. Original/fresh worker, guards and input panels compare byte-for-byte.' > "$proof/fresh-path-byte-checks.txt"
