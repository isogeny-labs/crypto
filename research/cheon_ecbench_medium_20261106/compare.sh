#!/usr/bin/env sh
# P1 of PROTOCOL.md: every run of the medium-tier sessions against the original
# session's run with the same arm, curve and target index. Thin orchestration
# (jq only); the counts are the ecbench records'. Wall times are not compared.
# Run from the repository root with both sessions' records.jsonl present.
set -eu
O=research/cheon_ecbench_20261022/sessions
N=research/cheon_ecbench_medium_20261106/sessions
for s in minus minus-naive plus; do
  jq -s -c --arg s "$s" --slurpfile old "$O/$s-20261022/records.jsonl" '
    def key: "\(.arm)|\(.workload.curve.slug)|\(.workload.target_index)";
    def sig: {status: .outcome.status, recovered: (.outcome.recovered|tostring),
              planted: .outcome.matches_planted, gae: .cost.total_gae, s: .cost.s,
              oq: .counters.oracle_queries, osg: .counters.oracle_server_group_ops,
              phases: [.phases[] | {name, adds, doubles, scalar_mults, gae}]};
    ($old | map({key: key, value: sig}) | from_entries) as $o
    | map({k: key, same: (sig == $o[key]), present: ($o[key] != null)})
    | {session: $s, runs: length, identical: (map(select(.same)) | length),
       absent_from_original: (map(select(.present | not)) | length),
       differ: (map(select(.present and (.same | not))) | map(.k))}' \
    "$N/$s-20261106/records.jsonl"
done
