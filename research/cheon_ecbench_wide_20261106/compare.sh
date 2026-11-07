#!/usr/bin/env sh
# P1 of PROTOCOL.md (positive control): every run of the medium study's minus
# and plus sessions (targets 0-7 on the six sizes 36 to 56 bits) against this
# study's run with the same arm, curve and target index. Thin orchestration
# (jq only); the counts are the ecbench records'. Wall times are not compared.
# Run from the repository root with both studies' records.jsonl present.
set -eu
O=research/cheon_ecbench_medium_20261106/sessions
N=research/cheon_ecbench_wide_20261106/sessions
for s in minus plus; do
  jq -s -c --arg s "$s" --slurpfile new "$N/$s-20261106/records.jsonl" '
    def key: "\(.arm)|\(.workload.curve.slug)|\(.workload.target_index)";
    def sig: {status: .outcome.status, recovered: (.outcome.recovered|tostring),
              planted: .outcome.matches_planted, gae: .cost.total_gae, s: .cost.s,
              oq: .counters.oracle_queries, osg: .counters.oracle_server_group_ops,
              phases: [.phases[] | {name, adds, doubles, scalar_mults, gae}]};
    ($new | map({key: key, value: sig}) | from_entries) as $n
    | map({k: key, same: (sig == $n[key]), present: ($n[key] != null)})
    | {session: $s, medium_runs: length, identical: (map(select(.same)) | length),
       absent_from_wide: (map(select(.present | not)) | map(.k)),
       differ: (map(select(.present and (.same | not))) | map(.k))}' \
    "$O/$s-20261106/records.jsonl"
done
