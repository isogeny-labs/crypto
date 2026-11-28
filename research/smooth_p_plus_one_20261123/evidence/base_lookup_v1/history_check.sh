#!/usr/bin/env bash
# Compare recorded algorithmic fields only; no new solve or timing calculation.
set -euo pipefail
lookup_tmp=$(mktemp -d)
trap 'rm -rf "$lookup_tmp"' EXIT
lookup_filter='select(.kind=="account_attempt") | {mode,draw,a,b,target,result,rank_before,rank_after,chart_counts}'
jq -c "$lookup_filter" research/smooth_p_plus_one_20261123/evidence/cold_accounting_v1/control.jsonl > "$lookup_tmp/old"
jq -c "$lookup_filter" research/smooth_p_plus_one_20261123/evidence/base_lookup_v1/original.jsonl > "$lookup_tmp/new"
diff -u "$lookup_tmp/old" "$lookup_tmp/new"
