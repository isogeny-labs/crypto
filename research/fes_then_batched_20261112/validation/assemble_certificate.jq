# replay-certificate.json from the six replay comparisons (validator code).
# Inputs: $m (META.json), $rr (recompute.jq over the replay reports),
#         $cmp (compare_replay.jq outputs), $receipts, $scripts.
def mean: if length == 0 then null else add / length end;
$m[0] as $M
| ($cmp | map(.identical_counts) | all) as $counts_ok
| ([ $cmp[].row_scalars[] | .verified ] | all) as $ver_ok
| ([ $cmp[].rho_scalars[] | .all_verified_replay ] | all) as $rho_ver_ok
| ([ $cmp[] | select(.calibration == "pinned") | .pinned_gae_max_rel_diff ] | max) as $pin_rel
| ([ $cmp[] | select(.calibration == "pinned") | .pinned_ratio_max_abs_diff ] | max) as $pin_ratio
| ([ $rr[0][] | .runs[] | .G_rel_err ] | max) as $replay_rebuild
| {
    schema: "replay-certificate/v1",
    study: "research/fes_then_batched_20261112",
    snapshot: $M.snapshot,
    status: (if $counts_ok and $ver_ok and $rho_ver_ok and ($pin_rel <= 1e-9) and ($pin_ratio <= 1e-12) then "verified" else "failed" end),
    independent: true,
    independence: $M.independence,
    scalars_verified: ($counts_ok and $ver_ok and $rho_ver_ok),
    scalars_verified_scope: "Every scalar leaf of all six committed reports compared with the replay (validation/compare_replay.jq): counts (targets tried, relations found, solver calls and ops, every native and work_native counter, every solver.extra counter including the bitop/v1 tally, group operations, rank and rows), seeds, planted and recovered logarithms, verified and exhausted flags, the rho per-run records of all three walks (16 runs each), labels and specs. Excluded: wall-clock leaves, the markdown rendering and the binary hash. GAE leaves: pinned instances to rel 1e-9 (float rounding of ratio x ns / ns), measured instances re-derived from the replay's counts at the replay's own measured ratio.",
    binary: $M.binary,
    build: $M.build,
    command: $M.command,
    host: $M.host,
    summary: {
      instances: ($cmp | length),
      ic_rows_compared: ([ $cmp[].rows_compared ] | add),
      ic_rows_counts_identical: ([ $cmp[].row_scalars[] | select(.counts_identical) ] | length),
      ic_rows_verified_in_replay: ([ $cmp[].row_scalars[] | select(.verified) ] | length),
      rho_runs_compared: ([ $cmp[].rho_runs_compared ] | add),
      rho_walks_per_run_identical: ([ $cmp[].rho_scalars[] | select(.per_run_identical) ] | length),
      rho_walks_compared: ([ $cmp[].rho_scalars[] ] | length),
      leaves_compared: ([ $cmp[].leaves_compared ] | add),
      leaves_identical: ([ $cmp[].leaves_identical ] | add),
      count_mismatches: ([ $cmp[].mismatches | length ] | add),
      pinned_instances_gae_max_rel_diff: $pin_rel,
      pinned_ratio_max_abs_diff: $pin_ratio,
      measured_instances_gae_rebuilt_from_replay_counts_max_rel_err: $replay_rebuild
    },
    reproduced: $M.reproduced,
    did_not_reproduce: $M.did_not_reproduce,
    runs: [ $cmp[] as $c
      | ($receipts[] | select(.name == $c.name)) as $r
      | ($rr[0][] | select(.name == $c.name)) as $rp
      | {
          run: $c.name, instance: $c.instance, calibration: $c.calibration,
          rows_compared: $c.rows_compared, rows_in_replay: $c.rows_in_replay,
          rho_runs_compared: $c.rho_runs_compared,
          leaves_compared: $c.leaves_compared, leaves_identical: $c.leaves_identical,
          count_mismatches: ($c.mismatches | length), mismatches: $c.mismatches,
          identical_counts: $c.identical_counts,
          differing_volatile: $c.differing_volatile,
          differing_calibration_ns: $c.differing_calibration_ns,
          differing_measured_gae: $c.differing_measured_gae,
          differing_pinned_gae_float: $c.differing_pinned_gae_float,
          pinned_gae_max_rel_diff: $c.pinned_gae_max_rel_diff,
          pinned_ratio_max_abs_diff: $c.pinned_ratio_max_abs_diff,
          calibration_pins_identical: $c.calibration_pins_identical,
          rho_selected_same_method: $c.rho_selected_same_method,
          rho: $c.rho_scalars,
          measured_gae: (if $c.calibration == "measured" then {
              committed_ratios: $c.measured_calibration.committed_ratios,
              replay_ratios: $c.measured_calibration.replay_ratios,
              replay_gae_rebuilt_from_counts_max_rel_err: ([ $rp.runs[].G_rel_err ] | max),
              mean_s_per_arm: ([ $c.row_scalars[] ] | group_by(.arm)
                | map({key: .[0].arm, value: {committed: ([ .[].s.committed ] | mean), replay: ([ .[].s.replay ] | mean)}}) | from_entries),
              ftb_over_if4: {
                committed: (([ $c.row_scalars[] | select(.arm == "fes-then-batched") | .s.committed ] | mean) / ([ $c.row_scalars[] | select(.arm == "inherited-f4") | .s.committed ] | mean)),
                replay: (([ $c.row_scalars[] | select(.arm == "fes-then-batched") | .s.replay ] | mean) / ([ $c.row_scalars[] | select(.arm == "inherited-f4") | .s.replay ] | mean))},
              band_from_replay_counts: ($rp.arms | map_values(.band_from_counts))
            } else null end),
          rows: $c.row_scalars,
          receipt: $r
        } ],
    scripts: $scripts
  }
