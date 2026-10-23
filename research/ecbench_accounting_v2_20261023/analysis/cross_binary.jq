# Q4: the same spec on the branch binary ($a records) and on the main binary ($b records).
# jq -n --slurpfile a BRANCH/records.jsonl --slurpfile b MAIN/records.jsonl [--arg floor 1] -f cross_binary.jq
def key: "\(.arm)/\(.workload.workload_id)/\(.round)/\(.warmup)";
def counts: { status: .outcome.status, recovered: .outcome.recovered, algorithm_seed: .algorithm_seed,
              total_gae: .cost.total_gae, unpriced: .cost.unpriced, lower_bound: .cost.lower_bound,
              phases: [.phases[] | {name, adds, doubles, scalar_mults, gae, native}],
              counters: .counters, run_id: .run_id };
def floors: { floor_s: .boundaries.floor_s, ratio_to_floor: .boundaries.ratio_to_floor,
              automorphisms_available: .boundaries.automorphisms_available,
              automorphisms_used: .boundaries.automorphisms_used };
($a | map({key: key, value: .}) | from_entries) as $A
| ($b | map({key: key, value: .}) | from_entries) as $B
| [ $A | to_entries[] | select($B[.key] != null) | .key as $k | .value as $x | $B[$k] as $y
    | { key: $k, arm: $x.arm, branch_rule: $x.boundaries.floor_rule, main_rule: $y.boundaries.floor_rule,
        counts_equal: (($x | counts) == ($y | counts)),
        differing_count_fields: ([($x | counts) as $cx | ($y | counts) as $cy | $cx | keys[] | select($cx[.] != $cy[.])]),
        floors_equal: (($x | floors) == ($y | floors)),
        branch_floor: ($x | floors), main_floor: ($y | floors),
        s_branch: $x.cost.s, s_main: $y.cost.s } ] as $rows
| { branch_records: ($A | length), main_records: ($B | length), matched: ($rows | length),
    unmatched: ((($A | length) - ($rows | length)) + (($B | length) - ($rows | length))),
    counts_equal: ($rows | map(select(.counts_equal)) | length),
    counts_differ: ($rows | map(select(.counts_equal | not)) | length),
    floors_equal: ($rows | map(select(.floors_equal)) | length),
    by_arm: ($rows | group_by(.arm) | map({arm: .[0].arm, n: length, counts_equal: (map(select(.counts_equal)) | length),
        floors_equal: (map(select(.floors_equal)) | length), branch_floor_s: .[0].branch_floor.floor_s, main_floor_s: .[0].main_floor.floor_s,
        branch_rule: .[0].branch_rule, main_rule: .[0].main_rule,
        branch_A: .[0].branch_floor.automorphisms_available, main_A: .[0].main_floor.automorphisms_available})),
    first_count_differences: ($rows | map(select(.counts_equal | not)) | .[0:5]) }
