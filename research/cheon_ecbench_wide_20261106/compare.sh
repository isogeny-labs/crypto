#!/usr/bin/env bash
# PROTOCOL.md P1 (positive control) and the amendment-1/2 cross-check. Thin
# orchestration (jq only); the counts are the ecbench records'. Wall times are
# not compared. Run from the repository root with all records.jsonl present.
#  1. every run of the medium study's minus and plus sessions (targets 0-7 on
#     36..56 bits, 4-bit steps) against this study's chunk run with the same
#     arm, curve and target index;
#  2. every record of the four interrupted whole-spec sessions against the
#     chunk run with the same key (records past 56 bits have no chunk run).
set -euo pipefail
O=research/cheon_ecbench_medium_20261106/sessions
N=research/cheon_ecbench_wide_20261106/sessions
BITS="36 38 40 42 44 46 48 50 52 54 56"
cmp() { # LABEL OLD_RECORDS NEW_RECORDS...
  local label=$1 old=$2; shift 2
  jq -s -c --arg l "$label" --slurpfile new <(cat "$@") '
    def key: "\(.arm)|\(.workload.curve.slug)|\(.workload.target_index)";
    def sig: {status: .outcome.status, recovered: (.outcome.recovered|tostring),
              planted: .outcome.matches_planted, gae: .cost.total_gae, s: .cost.s,
              oq: .counters.oracle_queries, osg: .counters.oracle_server_group_ops,
              phases: [.phases[] | {name, adds, doubles, scalar_mults, gae}]};
    ($new | map({key: key, value: sig}) | from_entries) as $n
    | map({k: key, same: (sig == $n[key]), present: ($n[key] != null)})
    | {check: $l, records: length, compared: (map(select(.present)) | length),
       identical: (map(select(.same)) | length),
       no_chunk_run: (map(select(.present | not)) | length),
       differ: (map(select(.present and (.same | not))) | map(.k))}' "$old"
}
for f in minus plus; do
  chunks=(); for b in $BITS; do chunks+=("$N/$f-b$b/records.jsonl"); done
  cmp "P1 medium $f" "$O/$f-20261106/records.jsonl" "${chunks[@]}"
  for s in 20261106 20261107; do
    cmp "interrupted $f-$s" "$N/$f-$s/records.jsonl" "${chunks[@]}"
  done
done
