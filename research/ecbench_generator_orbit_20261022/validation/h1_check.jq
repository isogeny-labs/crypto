# Independent H1 check (PROTOCOL.md §5) from raw records.jsonl.
# Written by the validator; does not reuse check_same_algorithm.sh.
#
# Usage: jq -s --arg session NAME -f h1_check.jq sessions/NAME/records.jsonl
#
# For every (workload_id, round, warmup) key, the ic-lazy-c* record and the
# ic-layout-c* record must agree on phases[2].native.trials,
# phases[2].native.relations, phases[3].native.rank, outcome.recovered and
# phases[1].native.pair_table_entries.  Keys must be unique per arm, every
# key must have a partner in both directions, and the phase indices must be
# the phases named (oracle_setup, relations, linear_algebra).
# Also: mean phases[2].native.trials of ic-gen-layout-c* non-warm-up records.

def key: [.workload.workload_id, .round, .warmup];
def sig: { trials: .phases[2].native.trials,
           relations: .phases[2].native.relations,
           rank: .phases[3].native.rank,
           recovered: .outcome.recovered,
           pair_table_entries: .phases[1].native.pair_table_entries };
def phase_names_ok: (.phases[1].name == "oracle_setup" and .phases[2].name == "relations"
                     and .phases[3].name == "linear_algebra");
def mean: if length == 0 then null else add / length end;

[ .[] | select(.arm | test("^ic-lazy-c")) ] as $lazy
| [ .[] | select(.arm | test("^ic-layout-c")) ] as $layout
| [ .[] | select(.arm | test("^ic-gen-layout-c")) ] as $gen
| ($layout | map({key: (key | tojson), value: sig}) | from_entries) as $lmap
| ($lazy | map({key: (key | tojson), value: sig}) | from_entries) as $zmap
| [ $lazy[] | (key | tojson) as $k | {k: $k, partner: ($lmap | has($k)), equal: ($lmap[$k] == sig)} ] as $pairs
| [ $pairs[] | select(.equal | not) ] as $bad
| { session: $session,
    lazy_arm: ($lazy | map(.arm) | unique),
    layout_arm: ($layout | map(.arm) | unique),
    lazy_records: ($lazy | length),
    layout_records: ($layout | length),
    lazy_keys_unique: (($zmap | length) == ($lazy | length)),
    layout_keys_unique: (($lmap | length) == ($layout | length)),
    layout_without_lazy_partner: ([ $layout[] | (key | tojson) as $k | select(($zmap | has($k)) | not) ] | length),
    pairs: ($pairs | length),
    pairs_with_partner: ([ $pairs[] | select(.partner) ] | length),
    pairs_equal: ([ $pairs[] | select(.equal) ] | length),
    disagreements: ($bad | .[0:10]),
    null_fields_in_signature: ([ ($lazy + $layout)[] | sig | to_entries[] | select(.value == null) ] | length),
    phase_names_ok: ([ ($lazy + $layout + $gen)[] | phase_names_ok ] | all),
    gen_arm: ($gen | map(.arm) | unique),
    gen_measured_records: ([ $gen[] | select(.warmup == false) ] | length),
    gen_mean_trials_measured: ([ $gen[] | select(.warmup == false) | .phases[2].native.trials ] | mean),
    layout_mean_trials_measured: ([ $layout[] | select(.warmup == false) | .phases[2].native.trials ] | mean) }
