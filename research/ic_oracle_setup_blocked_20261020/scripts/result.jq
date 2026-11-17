# RESULT.json from analysis/A-1k-8k.json and analysis/B-16k.json (jq -s, in that
# order). Copies and rounds nothing: every value is a field of the two analyses.
def arm($as; $name): [ $as[].arms[] | select(.arm == $name) ] | first;
def q(c): { sum: c.setup_wall_ratio_sum, min: c.setup_wall_pair_ratio.min, max: c.setup_wall_pair_ratio.max };
. as $as
| [ $as[].per_size[] ] as $sizes
| [ $sizes[] | .equality[] ] as $eq
| ($as[0].aa_control) as $aa
| { schema: "ic_oracle_setup_blocked.result/v1",
    curve: "icv1-fp32-t7319-90da43e9",
    sessions: [ $as[].session_id ],
    isolation: "L1",
    executions: (([ $eq[].executions ] | add) + $aa.equality.executions),
    identical_executions: (([ $eq[].identical ] | add) + $aa.equality.identical),
    equality_all_identical: (([ $eq[] | .executions == .identical ] | all) and ($aa.equality.executions == $aa.equality.identical)),
    aa_setup_ratio_sum: $aa.wall.setup_wall_ratio_sum,
    aa_setup_pair_ratio: [ $aa.wall.setup_wall_pair_ratio.min, $aa.wall.setup_wall_pair_ratio.max ],
    rows: [ $sizes[] as $s | $s.N as $n
      | { N: $n,
          pairs: $s.blocked_vs_batch_wall.pairs,
          setup_ms: { scalar: arm($as; "scalar-\($n)").wall_ms_mean.oracle_setup,
                      batch: arm($as; "batch-\($n)").wall_ms_mean.oracle_setup,
                      blocked: arm($as; "blocked-\($n)").wall_ms_mean.oracle_setup },
          run_ms: { scalar: arm($as; "scalar-\($n)").run_wall_ms_mean,
                    batch: arm($as; "batch-\($n)").run_wall_ms_mean,
                    blocked: arm($as; "blocked-\($n)").run_wall_ms_mean },
          blocked_over_batch: q($s.blocked_vs_batch_wall),
          blocked_over_scalar: q($s.blocked_vs_scalar_wall),
          batch_over_scalar: q($s.batch_vs_scalar_wall),
          run_blocked_over_batch: $s.blocked_vs_batch_wall.run_wall_ratio_sum,
          run_blocked_over_scalar: $s.blocked_vs_scalar_wall.run_wall_ratio_sum,
          rss_gib: { scalar: (arm($as; "scalar-\($n)").max_rss_kib.max / 1048576),
                     batch: (arm($as; "batch-\($n)").max_rss_kib.max / 1048576),
                     blocked: (arm($as; "blocked-\($n)").max_rss_kib.max / 1048576) },
          S_ratio_blocked_over_batch: $s.blocked_vs_batch_S.S_ratio,
          S_w0: arm($as; "blocked-\($n)").S_w0_mean,
          setup_gae: arm($as; "blocked-\($n)").setup_gae_mean,
          gae: arm($as; "blocked-\($n)").gae_mean,
          gap_to_rho: $s.blocked_vs_rho_S.S_ratio } ] }
