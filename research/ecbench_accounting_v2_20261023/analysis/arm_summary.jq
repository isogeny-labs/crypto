# Per-arm summary of measured (non-warm-up) verified records of one session, from the records themselves.
# jq -s -f arm_summary.jq records.jsonl
def mean: if length == 0 then null else add / length end;
def fixed($r): [$r.phases[] | select(.name == "setup" or .name == "automorphism_setup") | .gae] | add // 0;
[ .[] | select(.warmup == false) ] as $all
| [ $all | group_by(.arm)[]
    | map(select(.outcome.status == "verified")) as $v
    | { arm: .[0].arm, method: .[0].method.id, measured: length, verified: ($v | length),
        floor_s: (.[0].boundaries.floor_s), floor_rule: (.[0].boundaries.floor_rule // "first rule (A=2)"),
        A: (.[0].boundaries.automorphisms_available),
        mean_s: ($v | map(.cost.s) | mean),
        mean_ratio_to_floor: ($v | map(.boundaries.ratio_to_floor) | mean),
        mean_total_gae: ($v | map(.cost.total_gae) | mean),
        setup_gae_sum_over_total_sum: (if ($v | length) == 0 then null else (($v | map(fixed(.)) | add) / ($v | map(.cost.total_gae) | add)) end),
        setup_share_mean_of_runs: ($v | map(fixed(.) / .cost.total_gae) | mean),
        setup_share_recorded_mean: (($v | map(.detail.setup_share) | map(select(. != null))) | mean),
        runs_with_long_cycles: ($v | map(select((.counters.cycles_longer_than_window // 0) > 0)) | length),
        lost_walk_steps_sum: ($v | map(.counters.lost_walk_steps // 0) | add),
        uncharged_counters: ($v | map(.cost.unpriced) | add | unique) } ]
