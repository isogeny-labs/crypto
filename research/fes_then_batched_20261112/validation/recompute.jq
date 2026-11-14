# Validator recompute of ONE `ic bench --sweep` report of
# research/fes_then_batched_20261112 (results/<instance>.json).
#
# Written by the validator; shares no code with the producer's make_report.sh,
# results.json or the ic binary.  The pricing rule is transcribed from
# src/cryptanalysis/ic_boundary.rs `price_phase` (group adds + doubles at 1 GAE;
# the ten PRICED_NATIVE counters at ns_<unit> / ns_per_add; solver ops at
# solver.ns_per_op / ns_per_add) and is CHECKED here by rebuilding every phase's
# `gae` and every row's `total_gae` from the counts (fields *_rel_err).
#
# Band (AGENTS.md 5a, Table B formula of docs/ic/runs/VALIDATION.json):
#   S(w) = [G - C + w (U + n_C)] / sqrt r
#   G   = row total_gae
#   C   = GAE the calibration (pinned OR measured) put on native work:
#         PRICED_NATIVE counters with a non-null unit, plus solver ops
#   n_C = the operation count behind C
#   U   = unpriced operation counts: every native `*_uncharged` counter (none in
#         these reports) plus, per the 2026-11-08 rule, the phase sum of
#         work_native.table_insert_attempts when some phase carries the marker
#         `counted:table_insert_attempts`.
# Rho arm: the per_run records emit no unpriced counter, so U_rho = 0 and
# S_rho(w) = s (strict reading); `rho_ext_*` charge k native ops per walk step
# as a sensitivity (validator construction, not gate evidence).
#
# Usage: jq --arg name koblitz-n13 -f recompute.jq results/koblitz-n13.json

def PRICED: [
  ["sqrt_solves", "ns_per_sqrt"], ["as_solves", "ns_per_as_solve"],
  ["s4_pairs", "ns_per_s4_pair"], ["lookups", "ns_per_lookup"],
  ["row_ops", "ns_per_row_op"], ["legendre_symbols", "ns_per_legendre"],
  ["inversions", "ns_per_inversion"], ["frobenius_maps", "ns_per_frobenius"],
  ["canonicalisations", "ns_per_canon"], ["target_guard_probes", "ns_per_lookup"]
];
def W: [0, 0.1, 1];
def mean: if length == 0 then null else add / length end;
def relerr($a; $b):
  if $a == $b then 0
  else ((($a - $b) | fabs) / ([($a | fabs), ($b | fabs), 1e-300] | max)) end;
def arm: if .decomposition.name == "mitm" then "pair-table" else .decomposition.solver.name end;
def phase_list: [
  ["factor_base", .factor_base.cost], ["setup", .decomposition.setup],
  ["decomposition", .decomposition.cost], ["linear_algebra", .linear_algebra.cost],
  ["verify", .verify]
];

# One phase -> its priced native work under calibration $cal.
def priced_native($cal):
  . as $p
  | reduce PRICED[] as $pu ({gae: 0, ops: 0, gaps: []};
      ($p.native[$pu[0]] // 0) as $n
      | if $n > 0 then
          (if $cal[$pu[1]] == null then .gaps += [$pu[0]]
           else .gae += ($n * $cal[$pu[1]] / $cal.ns_per_add) | .ops += $n end)
        else . end);

def uncharged_sum: [ (.native // {}) | to_entries[] | select(.key | endswith("_uncharged")) | .value ] | add // 0;
def work_ops: [ (.work_native // {}) | to_entries[] | select(.key | startswith("counted:") | not) | .value ] | add // 0;

. as $rep
| $rep.calibration as $cal
| ($rep.rows[0].r) as $r
| ($r | sqrt) as $sq
| ($rep.rho_reference.mean_s) as $rho_s
| [ $rep.rows[] | . as $row
    | (.decomposition.solver) as $sv
    | (phase_list) as $ph
    | ([ $ph[] | .[1] | (.group_ops.adds + .group_ops.doubles) ] | add) as $group
    | ([ $ph[] | .[1] | priced_native($cal) ]) as $nat
    | (if $sv == null then {gae: 0, ops: 0}
       else {gae: ($sv.ops * $sv.ns_per_op / $cal.ns_per_add), ops: $sv.ops} end) as $sol
    | ([ $ph[] | .[1] ] | map(select((.work_native // {}) | has("counted:table_insert_attempts"))) | length > 0) as $marker
    | (if $marker then ([ $ph[] | .[1].work_native.table_insert_attempts // 0 ] | add) else 0 end) as $tia
    | ([ $ph[] | .[1] | uncharged_sum ] | add) as $unch
    | ([ $nat[] | .gae ] | add) as $nat_gae
    | ([ $nat[] | .ops ] | add) as $nat_ops
    | ($nat_gae + $sol.gae) as $C
    | ($nat_ops + $sol.ops) as $nC
    | ($tia + $unch) as $U
    | ([ range(0; $ph | length) as $i
         | ($ph[$i][1]) as $p
         | ($p.group_ops.adds + $p.group_ops.doubles + $nat[$i].gae
            + (if $ph[$i][0] == "decomposition" then $sol.gae else 0 end)) as $rec
         | relerr($rec; $p.gae) ] | max) as $phase_err
    | ($group + $C) as $G_rec
    | (if $sv != null and ($sv.extra.bitop_complete // 0) == 1 then
         ($sv.extra.bitop_word_ops + ($sv.extra.bitop_hash_probes // 0) + ($sv.extra.bitop_field_adds // 0)
          + ($sv.extra.bitop_field_muls // 0) + ($sv.extra.bitop_field_sqrs // 0) + ($sv.extra.bitop_field_invs // 0))
       else null end) as $bitop
    | ([ $ph[] | .[1] | work_ops ] | add) as $work_all
    | {
        arm: ($row | arm),
        label: $row.label,
        seed: $row.spec.seed,
        recovered: $row.recovered,
        verified: $row.verified,
        exhausted: $row.exhausted,
        targets_tried: $row.decomposition.targets_tried,
        relations_found: $row.decomposition.relations_found,
        rows: $row.linear_algebra.rows,
        rank: $row.linear_algebra.rank,
        solver_calls: ($sv.calls // null),
        solver_ops: ($sv.ops // null),
        solver_op_unit: ($sv.op_unit // null),
        priced_by: ($sv.priced_by // null),
        budget_exceeded: ($sv.budget_exceeded // null),
        n_vars: ($row.decomposition.system.n_vars // null),
        group_gae: $group,
        C: $C, n_C: $nC, U: $U,
        U_table_insert_attempts: $tia, U_uncharged: $unch,
        table_marker: $marker,
        native_gaps: ([ $nat[] | .gaps[] ] | unique),
        G_reported: $row.total_gae,
        G_recomputed: $G_rec,
        G_rel_err: relerr($G_rec; $row.total_gae),
        phase_gae_max_rel_err: $phase_err,
        G_minus_C_vs_group_abs_err: (($row.total_gae - $C - $group) | fabs),
        s_reported: $row.s,
        s_recomputed: ($G_rec / $sq),
        s_rel_err: relerr($G_rec / $sq; $row.s),
        s_over_rho_reported: $row.s_over_rho,
        s_over_rho_recomputed: (($G_rec / $sq) / $rho_s),
        rho_s_matches_reference: ($row.rho_s == $rho_s),
        band: [ W[] as $w | ($row.total_gae - $C + $w * ($U + $nC)) / $sq ],
        band_from_counts: [ W[] as $w | ($group + $w * ($U + $nC)) / $sq ],
        diag_bitop_band: (if $bitop == null then null
                          else [ W[] as $w | ($group + $w * ($U + $nat_ops + $bitop)) / $sq ] end),
        diag_all_counted_band: (if $bitop == null and $sv != null then null
                          else [ W[] as $w | ($group + $w * ($unch + $work_all + $nat_ops + ($bitop // 0))) / $sq ] end),
        solver_bitop_total: $bitop,
        work_native_ops: $work_all
      } ]
| . as $runs
| ($rep.regime) as $regime
| ([ $rep.rho_reference ] + ($rep.rho_reference_candidates // [])) as $rhos
| ($rep.instance | capture("f2m(?<n>[0-9]+)") | .n | tonumber) as $n
| def rho_summary:
    . as $x
    | { method: $x.method,
        runs: ($x.per_run | length),
        all_verified_recomputed: ([ $x.per_run[].verified ] | all),
        all_verified_reported: $x.all_verified,
        mean_s_reported: $x.mean_s,
        mean_s_recomputed: ([ $x.per_run[].s ] | mean),
        mean_s_rel_err: relerr([ $x.per_run[].s ] | mean; $x.mean_s),
        mean_steps: ([ $x.per_run[].steps ] | mean),
        walk_gae_minus_steps_range: ([ $x.per_run[] | ((.s_walk * $sq) - .steps) | (. * 1e6 | round) / 1e6 ] | [min, max]),
        setup_gae_mean: ([ $x.per_run[] | (.s - .s_walk) * $sq ] | mean),
        seeds: [ $x.per_run[].seed ],
        planted: [ $x.per_run[].planted ],
        band: [ W[] as $w | ([ $x.per_run[].s ] | mean) ],
        rho_ext_k1: [ W[] as $w | ([ $x.per_run[].s ] | mean) + $w * ([ $x.per_run[].steps ] | mean) / $sq ],
        rho_ext_kn1: [ W[] as $w | ([ $x.per_run[].s ] | mean) + $w * ($n + 1) * ([ $x.per_run[].steps ] | mean) / $sq ]
      };
  ([ $rhos[] | rho_summary ]) as $rs
| ($rs[0]) as $rho
| ([ "pair-table", "inherited-f4", "fes-then-inherited", "fes-then-batched" ]) as $arms
| (reduce $arms[] as $a ({};
     ([ $runs[] | select(.arm == $a) ]) as $rr
     | .[$a] = {
         runs: ($rr | length),
         all_verified: ([ $rr[].verified ] | all),
         none_exhausted: ([ $rr[].exhausted | not ] | all),
         mean_s_reported: ([ $rr[].s_reported ] | mean),
         mean_s_recomputed: ([ $rr[].s_recomputed ] | mean),
         mean_s_over_rho_reported: ([ $rr[].s_over_rho_reported ] | mean),
         mean_s_over_rho_recomputed: (([ $rr[].s_recomputed ] | mean) / $rho.mean_s_recomputed),
         mean_targets_tried: ([ $rr[].targets_tried ] | mean),
         mean_relations: ([ $rr[].relations_found ] | mean),
         mean_group_gae: ([ $rr[].group_gae ] | mean),
         mean_U: ([ $rr[].U ] | mean),
         mean_n_C: ([ $rr[].n_C ] | mean),
         mean_C: ([ $rr[].C ] | mean),
         band: [ range(0; 3) as $i | ([ $rr[].band[$i] ] | mean) ],
         band_from_counts: [ range(0; 3) as $i | ([ $rr[].band_from_counts[$i] ] | mean) ],
         band_formula_vs_counts_max_abs_err: ([ $rr[] | range(0; 3) as $i | (.band[$i] - .band_from_counts[$i]) | fabs ] | max),
         diag_bitop_band: (if ([ $rr[].diag_bitop_band ] | any(. == null)) then null
                           else [ range(0; 3) as $i | ([ $rr[].diag_bitop_band[$i] ] | mean) ] end),
         diag_all_counted_band: (if ([ $rr[].diag_all_counted_band ] | any(. == null)) then null
                           else [ range(0; 3) as $i | ([ $rr[].diag_all_counted_band[$i] ] | mean) ] end),
         op_units: ([ $rr[].solver_op_unit ] | unique),
         max_G_rel_err: ([ $rr[].G_rel_err ] | max),
         max_phase_gae_rel_err: ([ $rr[].phase_gae_max_rel_err ] | max),
         max_s_rel_err: ([ $rr[].s_rel_err ] | max),
         native_gaps: ([ $rr[].native_gaps[] ] | unique)
       }))
  as $A
| def order($x; $y):
    [ range(0; 3) as $i
      | if $x[$i] < $y[$i] then "<" elif $x[$i] > $y[$i] then ">" else "=" end ];
  def verdict($o):
    if ($o | all(. == "<")) then "holds: first below second at every w"
    elif ($o | all(. == ">")) then "holds: first above second at every w"
    elif ($o | all(. == "<" or . == "=")) then "weak: first <= second, tie at some w (strict order does not hold across the band)"
    elif ($o | all(. == ">" or . == "=")) then "weak: first >= second, tie at some w (strict order does not hold across the band)"
    else "flips inside the band: unresolved" end;
  def strict($o): ($o | all(. == "<")) or ($o | all(. == ">"));
{
  name: $name,
  instance: $rep.instance,
  regime: $regime,
  degree: $n,
  r: $r,
  sqrt_r: $sq,
  calibration_priced_by: (if ($rep.calibration_pins.pinned | length) > 0 and ($rep.calibration_pins.measured | length) == 0 then "pinned"
                          elif ($rep.calibration_pins.pinned | length) == 0 then "measured" else "mixed" end),
  calibration_ratios: ($cal | to_entries | map(select(.key | startswith("ns_per_")) | select(.value != null)
                       | {key: .key, value: (.value / $cal.ns_per_add)}) | from_entries),
  cost_qualification: $rep.cost_qualification,
  report_all_verified: $rep.all_verified,
  report_status: $rep.status,
  rows: ($runs | length),
  rho: $rho,
  rho_candidates: $rs,
  rho_rule: $rep.rho_reference_rule,
  rho_rule_check: {
    selected_is_lowest_mean: (([ $rs[] | .mean_s_recomputed ] | min) == $rho.mean_s_recomputed),
    same_seeds_all_candidates: (([ $rs[] | .seeds ] | unique | length) == 1),
    candidate_methods: [ $rs[] | .method ]
  },
  planted_link: {
    ic_recovered_by_seed: ([ $runs[] | {seed: .seed, recovered: .recovered} ] | unique),
    rho_planted_values: ($rho.planted | unique),
    every_ic_recovered_is_a_rho_planted_value: ([ $runs[].recovered ] | all(. as $v | $rho.planted | index($v) != null))
  },
  arms: $A,
  ratios: {
    ftb_over_if4: ($A["fes-then-batched"].mean_s_recomputed / $A["inherited-f4"].mean_s_recomputed),
    ftb_over_pair: ($A["fes-then-batched"].mean_s_recomputed / $A["pair-table"].mean_s_recomputed),
    ftb_over_fti: ($A["fes-then-batched"].mean_s_recomputed / $A["fes-then-inherited"].mean_s_recomputed),
    if4_over_pair: ($A["inherited-f4"].mean_s_recomputed / $A["pair-table"].mean_s_recomputed)
  },
  band_orders: {
    ftb_vs_if4: {order: order($A["fes-then-batched"].band_from_counts; $A["inherited-f4"].band_from_counts)},
    ftb_vs_pair: {order: order($A["fes-then-batched"].band_from_counts; $A["pair-table"].band_from_counts)},
    ftb_vs_rho: {order: order($A["fes-then-batched"].band_from_counts; $rho.band)},
    if4_vs_rho: {order: order($A["inherited-f4"].band_from_counts; $rho.band)},
    fti_vs_rho: {order: order($A["fes-then-inherited"].band_from_counts; $rho.band)},
    pair_vs_rho: {order: order($A["pair-table"].band_from_counts; $rho.band)},
    ftb_vs_if4_diag_bitop: {order: (if $A["fes-then-batched"].diag_bitop_band == null then null
                                   else order($A["fes-then-batched"].diag_bitop_band; $A["inherited-f4"].diag_bitop_band) end)},
    ic_min_vs_rho_ext_kn1: {order: order([ range(0; 3) as $i | ([ $A[] | .band_from_counts[$i] ] | min) ]; $rho.rho_ext_kn1)}
  } | with_entries(.value += {verdict: (if .value.order == null then null else verdict(.value.order) end),
                              strict_order_holds: (if .value.order == null then null else strict(.value.order) end)}),
  runs: $runs
}
