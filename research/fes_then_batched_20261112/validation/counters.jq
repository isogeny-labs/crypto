# AGENTS.md 5a listing: every counter of every arm with its mean count over the
# arm's repeats, classified as the validator's band reads it (validator code).
#   C        native counter a calibration priced (price_phase's ten, unit non-null)
#   C_solver the solver's own ops (priced by ns_per_op)
#   U        charged at w by the band: *_uncharged natives, and
#            work_native.table_insert_attempts behind `counted:table_insert_attempts`
#   counted_not_charged  other work_native counters (not in TABLE_WORK_COUNTERS)
#   solver_bitop_tally   bitop/v1 keys of solver.extra (diagnostic band only)
#   descriptive          native counters that are not operations of their own
#                        (trials, relations, walk_steps, rank, ...), or are
#                        priced inside group ops
# Usage: jq --arg name koblitz-n13 -f counters.jq results/koblitz-n13.json
def PRICED: {"sqrt_solves": "ns_per_sqrt", "as_solves": "ns_per_as_solve", "s4_pairs": "ns_per_s4_pair",
  "lookups": "ns_per_lookup", "row_ops": "ns_per_row_op", "legendre_symbols": "ns_per_legendre",
  "inversions": "ns_per_inversion", "frobenius_maps": "ns_per_frobenius",
  "canonicalisations": "ns_per_canon", "target_guard_probes": "ns_per_lookup"};
def mean: if length == 0 then null else add / length end;
def arm: if .decomposition.name == "mitm" then "pair-table" else .decomposition.solver.name end;
def phases: [ ["factor_base", .factor_base.cost], ["setup", .decomposition.setup],
  ["decomposition", .decomposition.cost], ["linear_algebra", .linear_algebra.cost], ["verify", .verify] ];
. as $rep | $rep.calibration as $cal
| [ $rep.rows[] | . as $row
    | ([ phases[] | .[1] ]) as $ps
    | ([ $ps[] | select((.work_native // {}) | has("counted:table_insert_attempts")) ] | length > 0) as $marker
    | { arm: ($row | arm),
        entries: (
          [ phases[] as [$pn, $p] | ($p.native // {}) | to_entries[]
            | .key as $k
            | {key: "\($pn).native.\(.key)", value: .value,
               class: (if (PRICED | has($k)) then (if $cal[PRICED[$k]] == null then "unpriced_no_unit" else "C" end)
                       elif (.key | endswith("_uncharged")) then "U"
                       elif (.key == "solver_ops") then "C_solver(same count as solver.ops)"
                       else "descriptive" end)} ]
          + [ phases[] as [$pn, $p] | ($p.work_native // {}) | to_entries[] | select(.key | startswith("counted:") | not)
            | {key: "\($pn).work_native.\(.key)", value: .value,
               class: (if .key == "table_insert_attempts" and $marker then "U" else "counted_not_charged" end)} ]
          + (if $row.decomposition.solver == null then [] else
              [ {key: "solver.ops", value: $row.decomposition.solver.ops, class: "C_solver"} ]
              + [ $row.decomposition.solver.extra | to_entries[] | select(.key | startswith("bitop_")) | select(.key != "bitop_complete")
                  | {key: "solver.extra.\(.key)", value: .value, class: "solver_bitop_tally"} ] end)
          + [ phases[] as [$pn, $p] | {key: "\($pn).group_ops.adds+doubles", value: ($p.group_ops.adds + $p.group_ops.doubles), class: "group_ops (1 GAE each)"} ]
        ) } ]
| group_by(.arm)
| map({ arm: .[0].arm,
        counters: ([ .[].entries[] ] | group_by(.key)
                   | map({counter: .[0].key, class: .[0].class, mean_count: ([ .[].value ] | mean)})
                   | map(select(.mean_count != 0 or .class == "U"))) })
| {name: $name, instance: $rep.instance,
   arms: (map({key: .arm, value: .counters}) | from_entries),
   rho: {
     emitted_per_run_fields: ($rep.rho_reference.per_run[0] | keys),
     unpriced_counters_emitted: [],
     note: "The matched rho records emit no unpriced counter (no canonicalisation, distinguished-point test, Frobenius map or table operation counts); only steps (walk group additions) and S. U_rho = 0 in the strict band, so rho is a lower bound at w > 0; rho_ext_* charge k native ops per walk step as a sensitivity.",
     mean_steps: ([ $rep.rho_reference.per_run[].steps ] | mean)
   }}
