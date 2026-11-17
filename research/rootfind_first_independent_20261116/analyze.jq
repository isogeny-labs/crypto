if length != 3 then error("expected exactly three curve records") else . end
| if map(.field_degree) != [13, 19, 23] then error("curve order changed") else . end
| if map(.subgroup_order) != [2003, 130873, 2095853] then error("subgroup changed") else . end
| if map(.sample_targets) != [2003, 1024, 1024] then error("sample changed") else . end
| if map(.seed_rank) != [5, 11, 23] then error("seed ranks changed") else . end
| if map(.base_sha256) != [
    "d9f69121c63a04c3f744c5be0e4ea488a030db0315794a4e05c7a9d3cd341807",
    "013a0d4223494500cd06b48bf41b51dda2dd5add0009d1b89cdb0b7bd49c0a5a",
    "686cba2fafe973e49139c2722df47bf728340e25a4df5faad8b51dfa91df3877"
  ] then error("factor-base identity changed") else . end
| map(
    if .status != "passed" or .schema != "rootfind-first-independent-screen/v1"
       then error("invalid source record") else . end
    | if .first_full_rank == null or .candidate_full_rank == null or .all_full_rank == null
       then error("censored full-rank arm") else . end
    | if .first_final_rank != .columns_with_target
         or .candidate_final_rank != .columns_with_target
         or .all_final_rank != .columns_with_target
       then error("final rank incomplete") else . end
    | if [ .first_full_rank.scan, .candidate_full_rank.scan, .all_full_rank.scan ]
         | any(.modeled_group_adds != (.subtractions + .pair_readds))
       then error("group proxy does not add") else . end
    | {
        field_degree,
        curve_id,
        base_sha256,
        subgroup_order,
        base_points,
        columns_with_target,
        seed_rank,
        sample_targets,
        nonidentity_queries,
        hit_targets,
        first_full_rank,
        candidate_full_rank,
        all_full_rank,
        candidate_later_independent_rows,
        candidate_dependent_skips,
        delta_candidate_minus_first: {
          queries: (.candidate_full_rank.queries - .first_full_rank.queries),
          modeled_group_adds: (.candidate_full_rank.scan.modeled_group_adds - .first_full_rank.scan.modeled_group_adds),
          lookups: (.candidate_full_rank.scan.lookups - .first_full_rank.scan.lookups),
          pair_readds: (.candidate_full_rank.scan.pair_readds - .first_full_rank.scan.pair_readds),
          row_attempts: (.candidate_full_rank.scan.row_attempts - .first_full_rank.scan.row_attempts)
        },
        candidate_passes_operation_prediction:
          (.candidate_full_rank.scan.modeled_group_adds < .first_full_rank.scan.modeled_group_adds)
      }
  ) as $cells
| {
    schema: "rootfind-first-independent-results/v1",
    status: "completed_fixed_stream_screen",
    run_id: "run-20261116T1342Z",
    protocol_commit: "0e413634e86a9358d431aaf2e0a3dc77ce5e9b40",
    source_commit: "83407456e0f052d4cf51d533b1fda824063b2287",
    harness_commit: "ea2fc503bfb431f1b2ad7f5a9ec24c1241fcba9f",
    branch_head_at_run: "42671e8c465116fa9991b8964a5618905d56e4e9",
    test_executable_sha256: "f58ad80e4402725f22ee51dfe14db1b8bb0af12a2c56f22cddbbf6de5b6c0247",
    raw_sha256: { stdout: $stdout_sha, stderr: $stderr_sha, meta: $meta_sha },
    checked_scalar: 641,
    scalar_check_scope: "source harness asserts 641 at full rank and independently re-adds each triple; no separate reviewer replay",
    cost_unit: "modeled batch subtraction or pair re-addition, one group-addition proxy each",
    isolation: "L0 macOS shared host",
    cells: $cells,
    candidate_passes_all_cells: ($cells | all(.candidate_passes_operation_prediction))
  }
