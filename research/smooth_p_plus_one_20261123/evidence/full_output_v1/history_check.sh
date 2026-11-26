#!/usr/bin/env bash
# Administrative comparison of retained records; performs no solver calls.
set -euo pipefail
study=research/smooth_p_plus_one_20261123/evidence
fields='select(.kind=="account_attempt") | {mode,draw,a,b,target,result,rank_before,rank_after,chart_counts}'
for policy in original lookup; do
  diff -u <(jq -cS "$fields" "$study/base_lookup_v1/$policy.jsonl") \
    <(jq -cS "$fields" "$study/full_output_v1/$policy.jsonl")
done
matched='select(.kind=="account_attempt" and .mode==$mode) | {draw,a,b,target,result,rank_before,rank_after}'
diff -u <(jq -cS --arg mode Direct "$matched" "$study/full_output_v1/full_lookup.jsonl") \
  <(jq -cS --arg mode ProductionCofactorMsolve "$matched" "$study/full_output_v1/full_lookup.jsonl")
