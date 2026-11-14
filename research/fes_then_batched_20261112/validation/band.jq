# The 5a band table (validator code): from recompute.json, per instance, S(w) at
# w = 0, 0.1, 1 for fes-then-batched, inherited-f4, fes-then-inherited, the pair
# table and the matched rho; the order verdicts; and the gae_band basis used in
# VALIDATION.json (for each w, the instance with the smallest S_ftb(w)/S_rho(w),
# the convention of docs/ic/runs/VALIDATION.json).
# Usage: jq -f band.jq recompute.json
def W: ["0", "0.1", "1"];
def tri($x): {"0": $x[0], "0.1": $x[1], "1": $x[2]};
. as $all
| [ $all[] | . as $i
    | {
        name: .name, instance: .instance, regime: .regime, r: .r,
        calibration: .calibration_priced_by,
        rho_method: .rho.method,
        S: {
          "fes-then-batched": tri(.arms["fes-then-batched"].band_from_counts),
          "inherited-f4": tri(.arms["inherited-f4"].band_from_counts),
          "fes-then-inherited": tri(.arms["fes-then-inherited"].band_from_counts),
          "pair-table": tri(.arms["pair-table"].band_from_counts),
          "rho": tri(.rho.band)
        },
        S_at_run_calibration: {
          "fes-then-batched": .arms["fes-then-batched"].mean_s_recomputed,
          "inherited-f4": .arms["inherited-f4"].mean_s_recomputed,
          "fes-then-inherited": .arms["fes-then-inherited"].mean_s_recomputed,
          "pair-table": .arms["pair-table"].mean_s_recomputed,
          "rho": .rho.mean_s_recomputed
        },
        counts_mean: (.arms | map_values({group_gae: .mean_group_gae, U: .mean_U, n_C: .mean_n_C, targets_tried: .mean_targets_tried, relations: .mean_relations})),
        ratio_ftb_over_if4: tri([ range(0; 3) as $k | $i.arms["fes-then-batched"].band_from_counts[$k] / $i.arms["inherited-f4"].band_from_counts[$k] ]),
        ratio_ftb_over_rho: tri([ range(0; 3) as $k | $i.arms["fes-then-batched"].band_from_counts[$k] / $i.rho.band[$k] ]),
        orders: (.band_orders | map_values({order: (if .order == null then null else tri(.order) end), verdict, strict_order_holds})),
        diagnostics: {
          bitop_band_ftb: (if .arms["fes-then-batched"].diag_bitop_band == null then null else tri(.arms["fes-then-batched"].diag_bitop_band) end),
          bitop_band_if4: (if .arms["inherited-f4"].diag_bitop_band == null then null else tri(.arms["inherited-f4"].diag_bitop_band) end),
          all_counted_band: (.arms | map_values(if .diag_all_counted_band == null then null else tri(.diag_all_counted_band) end)),
          rho_ext_k1: tri(.rho.rho_ext_k1),
          rho_ext_kn1: tri(.rho.rho_ext_kn1)
        }
      } ] as $inst
| ([ W[] as $w | ($inst | min_by(.ratio_ftb_over_rho[$w])) as $b
     | {key: $w, value: {ic: $b.S["fes-then-batched"][$w], rho: $b.S.rho[$w], instance: $b.name, ratio: $b.ratio_ftb_over_rho[$w]}} ] | from_entries) as $basis
| {
    schema: "validator-band/v1",
    rule: "S(w) = [G - C + w (U + n_C)] / sqrt r, computed exactly as (group adds + doubles + w (U + n_C)) / sqrt r (G - C equals the group ops to <= 1e-12 GAE/sqrt r, recompute.json band_formula_vs_counts_max_abs_err). C, n_C: price_phase's ten native counters with a non-null unit (pinned on Koblitz, measured on random binary) plus the solver's own ops. U: *_uncharged natives (none in these reports) plus work_native.table_insert_attempts behind counted:table_insert_attempts (2026-11-08 default reading; the pair table's per-step inserts; 0 for descent-algebraic). Rho: no unpriced counter emitted, U_rho = 0, S_rho(w) = mean s (lower bound at w > 0). Mean over the arm's 3 repeats (rho: 16 runs).",
    weights: [0, 0.1, 1],
    instances: $inst,
    gae_band_basis: "for each weight, the instance with the smallest S(fes-then-batched)/S(rho) (docs/ic/runs/VALIDATION.json convention)",
    gae_band: ($basis | map_values({ic, rho})),
    gae_band_detail: $basis,
    summary: {
      ftb_vs_if4: [ $inst[] | {name, order: .orders.ftb_vs_if4.order, verdict: .orders.ftb_vs_if4.verdict} ],
      ftb_vs_if4_strict_holds_on: ([ $inst[] | select(.orders.ftb_vs_if4.strict_order_holds) ] | length),
      ftb_vs_pair_strict_holds_on: ([ $inst[] | select(.orders.ftb_vs_pair.strict_order_holds) ] | length),
      every_ic_arm_above_rho_at_every_w: ([ $inst[] | .orders.ftb_vs_rho, .orders.if4_vs_rho, .orders.fti_vs_rho, .orders.pair_vs_rho | .order | to_entries | all(.value == ">") ] | all),
      instances: ($inst | length)
    }
  }
