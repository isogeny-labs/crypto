#!/bin/sh
# The counts-identical control: every count-bearing field of every record, two sessions.
# Usage: counts_diff.sh SESSION_A SESSION_B  -> JSON; exit 1 when anything differs or is unmatched.
set -eu
proj() { jq -c 'select(.schema == "ecbench.record/v1") | {key: ([.arm, .round, .warmup, .workload.target_index, .workload.workload_id] | tojson),
  counts: {status: .outcome.status, recovered: .outcome.recovered, matches_target: .outcome.matches_target,
    algorithm_seed, run_id, workload_sha256: .workload.workload_sha256, method_sha256: .method.method_sha256,
    total_gae: .cost.total_gae, s: .cost.s, lower_bound: .cost.lower_bound, unpriced: .cost.unpriced,
    phases: [.phases[] | {name, adds, doubles, scalar_mults, gae, native}], counters}}' "$1/records.jsonl"; }
ta=$(mktemp); tb=$(mktemp)
proj "$1" | jq -s 'map({(.key): .counts}) | add' > "$ta"
proj "$2" | jq -s 'map({(.key): .counts}) | add' > "$tb"
set +e
jq -n --slurpfile a "$ta" --slurpfile b "$tb" --arg na "$(basename "$1")" --arg nb "$(basename "$2")" '
  $a[0] as $a | $b[0] as $b
  | ($a | keys) as $ka | ($b | keys) as $kb
  | ($ka - $kb) as $only_a | ($kb - $ka) as $only_b
  | ([$ka[] | select($b[.] != null)]) as $both
  | ($both | map(select(($a[.] | tojson) != ($b[.] | tojson)))) as $diff
  | {a: $na, b: $nb, records_a: ($ka | length), records_b: ($kb | length), matched: ($both | length),
     only_in_a: $only_a, only_in_b: $only_b, differing: ($diff | length),
     differing_keys: $diff[0:10],
     identical: (($only_a | length) == 0 and ($only_b | length) == 0 and ($diff | length) == 0)}' > "$ta.out"
cat "$ta.out"
jq -e '.identical' "$ta.out" >/dev/null; rc=$?
rm -f "$ta" "$tb" "$ta.out"
exit $rc
