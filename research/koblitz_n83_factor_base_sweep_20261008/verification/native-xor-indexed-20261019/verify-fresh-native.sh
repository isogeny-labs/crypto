#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
opt=/home/ec2-user/scratch/n83-xor-opt-20261019
raw="$opt/publication/fresh-extract/raw-bundle"
out=research/koblitz_n83_factor_base_sweep_20261008/verification/native-xor-indexed-20261019/fresh-replay
mkdir -p "$out"
jq -r '.[]|[.id,.dir]|@tsv' "$opt/manifest-final.json" | while IFS=$'\t' read -r id original; do
 case "$id" in
 profile-censored) dir="$raw/runs/xor-profile/witness";;
 profile-one-conflict) dir="$raw/runs/xor-early/witness";;
 *) dir="$raw/runs/xor-indexed/$id";;
 esac
 if test -f "$dir/audit.json"; then
  "$raw/bin/runner-analysis-v5" audit "$dir" > "$out/$id.json"
  jq -S . "$dir/audit.json" > "$opt/publication/expected-audit.json"
  jq -S . "$out/$id.json" > "$opt/publication/fresh-audit.json"
  cmp "$opt/publication/expected-audit.json" "$opt/publication/fresh-audit.json"
 fi
done
# This is a new typed interpretation, not a rewrite of the original failed audit.
"$raw/bin/runner-analysis-v5" audit "$raw/runs/xor-indexed/aa-e1" > "$out/aa-e1-corrected-unknown.json"
