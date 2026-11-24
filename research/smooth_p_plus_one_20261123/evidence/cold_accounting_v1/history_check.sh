#!/usr/bin/env bash
# Administrative comparison of already recorded fields; performs no new solve.
set -euo pipefail
cold_dir=$(mktemp -d)
trap 'rm -rf "$cold_dir"' EXIT
jq -c 'select(.kind=="attempt" and (.mode=="Direct" or .mode=="Refined")) | {mode,draw,a,b,target,selected:.result.selected,relations:.result.relations}' \
 research/smooth_p_plus_one_20261123/evidence/refinement_v1/f127_recovery.jsonl > "$cold_dir/old"
jq -c 'select(.kind=="account_attempt" and (.mode=="Direct" or .mode=="Refined")) | {mode,draw,a,b,target,selected:.result.selected,relations:.result.relations}' \
 research/smooth_p_plus_one_20261123/evidence/cold_accounting_v1/control.jsonl > "$cold_dir/new"
diff -u "$cold_dir/old" "$cold_dir/new"
