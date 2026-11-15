# Leaf-by-leaf comparison of a committed raw report with the validator's replay
# of the same sweep (validator code).  Every scalar leaf of both JSON documents
# is compared, classed as:
#   volatile      wall-clock leaves (wall_ns, wall_seconds, elapsed_seconds),
#                 the markdown rendering, the software block (binary hash):
#                 ignored, counted.
#   calibration   calibration/ns_* and solver ns_per_op.  Even on a PINNED
#                 instance a report stores ns = pinned ratio x the host's
#                 measured ns_per_add, so the raw ns differ run to run; on a
#                 pinned instance the RATIOS ns_x / ns_per_add are compared
#                 instead (pinned_ratio_max_abs_diff), on a measured instance
#                 they are reported (measured_calibration).
#   gae           gae, total_gae, s, s_over_rho, gae_before_solver: on a
#                 pinned instance they must agree to rel 1e-9 (float rounding
#                 of ratio x ns / ns only; max reported); on a measured
#                 instance they are re-derived from counts by recompute.jq.
#   mismatch      anything else that differs: counts, seeds, planted and
#                 recovered values, verified flags, rho per-run records,
#                 labels, specs.  Must be empty.
# Usage: jq -n --arg name koblitz-n13 --slurpfile a committed.json --slurpfile b replay.json -f compare_replay.jq
def leaves: [ paths(scalars) as $p | {key: ($p | map(tostring) | join("/")), value: getpath($p)} ] | from_entries;
def volatile: test("(^|/)(wall_ns|wall_seconds|elapsed_seconds)$") or test("^markdown$") or test("^software/");
def calib: test("^calibration/ns_per_") or test("(^|/)ns_per_op$");
def gae_like: test("(^|/)(gae|total_gae|s|s_over_rho|gae_before_solver)$");
def relerr($x; $y): if $x == $y then 0 elif ($x | type) != "number" or ($y | type) != "number" then 1e300
  else (($x - $y) | fabs) / ([($x | fabs), ($y | fabs)] | max) end;
def ratios: .calibration as $c | $c | to_entries
  | map(select((.key | startswith("ns_per_")) and .value != null) | {key, value: (.value / $c.ns_per_add)}) | from_entries;
$a[0] as $A | $b[0] as $B
| ($A.calibration_pins.pinned | length == 0) as $measured
| ($A | leaves) as $la | ($B | leaves) as $lb
| ([ $la, $lb | keys[] ] | unique) as $keys
| [ $keys[] as $k
    | select(($la | has($k) | not) or ($lb | has($k) | not) or ($la[$k] != $lb[$k]))
    | {path: $k, committed: $la[$k], replay: $lb[$k], rel: relerr($la[$k]; $lb[$k]),
       class: (if ($k | volatile) then "volatile"
               elif ($k | calib) then "calibration"
               elif ($k | gae_like) and ($k | test("^rho_reference") | not) then
                 (if $measured then "measured_gae" elif relerr($la[$k]; $lb[$k]) <= 1e-9 then "pinned_gae_float" else "mismatch" end)
               else "mismatch" end)} ] as $diff
| ($A | ratios) as $ra | ($B | ratios) as $rb
| {
    name: $name,
    instance: $A.instance,
    calibration: (if $measured then "measured" else "pinned" end),
    leaves_compared: ($keys | length),
    leaves_identical: (($keys | length) - ($diff | length)),
    differing_volatile: ([ $diff[] | select(.class == "volatile") ] | length),
    differing_calibration_ns: ([ $diff[] | select(.class == "calibration") ] | length),
    differing_measured_gae: ([ $diff[] | select(.class == "measured_gae") ] | length),
    differing_pinned_gae_float: ([ $diff[] | select(.class == "pinned_gae_float") ] | length),
    pinned_gae_max_rel_diff: ([ $diff[] | select(.class == "pinned_gae_float") | .rel ] | max // 0),
    pinned_ratio_max_abs_diff: (if $measured then null
        else ([ ($A.calibration_pins.pinned)[] as $u | (($ra[$u] // 1e300) - ($rb[$u] // -1e300)) | fabs ] | max) end),
    calibration_pins_identical: ($A.calibration_pins == $B.calibration_pins),
    mismatches: [ $diff[] | select(.class == "mismatch") ],
    measured_calibration: (if $measured then {committed_ratios: $ra, replay_ratios: $rb} else null end),
    row_scalars: [ range(0; $A.rows | length) as $i | $A.rows[$i] as $x | ($B.rows[$i] // {}) as $y
      | def sc: {arm: (if .decomposition.name == "mitm" then "pair-table" else .decomposition.solver.name end),
                 seed: .spec.seed, targets_tried: .decomposition.targets_tried,
                 relations_found: .decomposition.relations_found,
                 solver_calls: (.decomposition.solver.calls // null), solver_ops: (.decomposition.solver.ops // null),
                 rank: .linear_algebra.rank, recovered, verified, exhausted};
        ($x | sc) as $cx | ($y | sc) as $cy
      | $cx + {counts_identical: ($cx == $cy),
               total_gae: {committed: $x.total_gae, replay: $y.total_gae, rel: relerr($x.total_gae; $y.total_gae)},
               s: {committed: $x.s, replay: $y.s, rel: relerr($x.s; $y.s)},
               priced_by: (if $measured then "measured" else "pinned" end)} ],
    rho_scalars: [ ([$A.rho_reference] + ($A.rho_reference_candidates // []) + [$A.rho_reference_plain]) as $ra
      | ([$B.rho_reference] + ($B.rho_reference_candidates // []) + [$B.rho_reference_plain]) as $rb
      | range(0; $ra | length) as $j
      | {method: $ra[$j].method, runs: ($ra[$j].per_run | length),
         per_run_identical: ($ra[$j].per_run == $rb[$j].per_run),
         all_verified_committed: ([ $ra[$j].per_run[].verified ] | all),
         all_verified_replay: ([ $rb[$j].per_run[].verified ] | all),
         mean_s: {committed: $ra[$j].mean_s, replay: $rb[$j].mean_s}} ],
    rho_selected_same_method: ($A.rho_reference.method == $B.rho_reference.method),
    rows_compared: ($A.rows | length),
    rows_in_replay: ($B.rows | length),
    rho_runs_compared: (([$A.rho_reference] + ($A.rho_reference_candidates // []) + [$A.rho_reference_plain]) | map(.per_run | length) | add),
    identical_counts: ([ $diff[] | select(.class == "mismatch") ] | length == 0)
  }
