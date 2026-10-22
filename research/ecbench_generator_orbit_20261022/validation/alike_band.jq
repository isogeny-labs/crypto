# Diagnostic (not a headline): a §5a band applied alike to every arm, IC and
# rho, over every native operation the `*_full` records charge.
#
# Usage: jq -s --slurpfile fp src/cryptanalysis/ecbench/full_prices.json \
#           --arg session NAME -f alike_band.jq sessions/NAME/records.jsonl
#
# Per record:
#   group = GAE of group operations: IC = sum over phases of adds + doubles
#           (full_price.rs charges each at 1); rho = G minus its "natives" phase.
#   ops   = native unit-operations: sum over native counters of
#           count * (sum of the unit multiplicities of its rule), the rule
#           table being full_price.rs `rule` at 45a11de2eb (e.g. a
#           curve-equation check is 2 field_mul + 2 field_sqr = 4 operations,
#           the factor-base byte floor 1/8 word XOR per byte).  Counters the
#           rule charges as group operations or outcomes are not operations.
#   S_alike(w) = (group + w * ops) / sqrt(r).
# Consistency check: group + sum(count * mult * unit ratio of the
# full_prices.json row for the slug) must reproduce G (cost.total_gae).

def rules: {
  "as_solves": [["as_solve", 1.0]], "legacy_as_solves_uncharged": [["as_solve", 1.0]],
  "lookups": [["lookup", 1.0]], "target_guard_probes": [["lookup", 1.0]],
  "hash_probes_uncharged": [["lookup", 1.0]], "factor_point_inserts_uncharged": [["lookup", 1.0]],
  "pair_table_entries": [["lookup", 1.0]], "abscissae_scanned_uncharged": [["lookup", 1.0]],
  "point_lifts_uncharged": [["lookup", 1.0]], "table_inserts_uncharged": [["lookup", 1.0]],
  "table_queries_uncharged": [["lookup", 1.0]], "distinguished_points": [["lookup", 1.0]],
  "row_ops": [["row_op", 1.0]], "frobenius_maps": [["frobenius", 1.0]],
  "canonicalisations": [["canon", 1.0]], "canonicalisations_uncharged": [["canon", 1.0]],
  "inversions": [["field_inv", 1.0]],
  "lift_field_squares_uncharged": [["field_sqr", 1.0]], "min_key_field_squares_uncharged": [["field_sqr", 1.0]],
  "lift_field_multiplies_uncharged": [["field_mul", 1.0]],
  "curve_equation_checks_uncharged": [["field_mul", 2.0], ["field_sqr", 2.0]],
  "point_negations_uncharged": [["word_xor", 1.0]], "partition_hashes_uncharged": [["word_xor", 1.0]],
  "factor_base_vec_bytes_floor_uncharged": [["word_xor", 0.125]]
};

def weights: [0, 0.1, 1];
def mean: add / length;

($fp[0].instances) as $rows
| [ .[] | select(.warmup == false and .outcome.status == "verified")
    | . as $r
    | ($r.arm | test("^ic-")) as $ic
    | ($rows[$r.workload.curve.slug] // error("no full_prices row for \($r.workload.curve.slug)")) as $u
    | (if $ic then [ $r.phases[] | .native | to_entries[] ]
       else [ $r.phases[] | select(.name == "natives") | .native | to_entries[] ] end) as $nat
    | ([ $nat[] | select(rules[.key] != null) | .value * ([ rules[.key][] | .[1] ] | add) ] | add // 0) as $ops
    | ([ $nat[] | select(rules[.key] != null) | .value as $k | [ rules[.key][] | $k * .[1] * $u[.[0]] ] | add ] | add // 0) as $native_gae
    | (if $ic then ([ $r.phases[] | .adds + .doubles ] | add)
       else $r.cost.total_gae - ([ $r.phases[] | select(.name == "natives") | .gae ] | add // 0) end) as $group
    | ($r.workload.curve.r | sqrt) as $sq
    | { arm: $r.arm, ops: $ops, group: $group,
        g_rel_err: ((($group + $native_gae) - $r.cost.total_gae) / $r.cost.total_gae | fabs),
        s: [ weights[] as $w | ($group + $w * $ops) / $sq ] } ]
| { session: $session,
    max_g_rel_err: ([ .[].g_rel_err ] | max),
    arms: ( group_by(.arm)
            | map({ arm: .[0].arm, n: length,
                    mean_native_ops: ([ .[].ops ] | mean),
                    mean_group_gae: ([ .[].group ] | mean),
                    mean_s_alike: [ range(0; 3) as $i | ([ .[].s[$i] ] | mean) ] }) ) }
