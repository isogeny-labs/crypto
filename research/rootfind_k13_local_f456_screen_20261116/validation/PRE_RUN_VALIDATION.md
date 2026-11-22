# Pre-run validation

The required `cargo test --release --lib` was started from source commit `a597dccdaf` with Rust 1.99.1 and a shared release target directory. It compiled successfully, then was interrupted after known failures and long-running tail tests. Exit status was 101 (SIGINT); this is **not a passing suite**. The raw log is `validation/cargo-test-release-lib.log` (SHA-256 `8c18743a25121a9f98b1b5c29d42af8d1740f36de396c6f7badfe191e37e85df`). At interruption, completed test lines were 6406 passing, 23 failing and 87 ignored; remaining tests had no final result.

A focused replay of `registered_prime_curve_resolves_its_slug_and_ec1` fails because the curve is not marked registered. `python3 scripts/check_curve_names.py` independently reports that `src/cryptanalysis/ecbench/registry_index.txt` and `src/cryptanalysis/curve_aliases.json` disagree with `docs/curves/registry.json`; its broad lint also reports many unrelated historical names. A focused replay of `chained_s3_strategy_feeds_verified_relation_rank_and_target_solve` fails its `capped.trials == 0` assertion (`16` observed). Neither source path is changed by this screen. Other failures' causes are not classified here.

The K13 `ecbench plan --json` validated the frozen spec as 2 registered workloads, 8 arms and 48 executions. The analyzer passed syntax and synthetic exact-pair/timeout checks. Build and focused ecbench test results are recorded separately before measurement. This validation limit bars treating the broad source tree as green; the local screen remains descriptive.

## Failing tests at interruption

- `cryptanalysis::ecbench::band::tests::default_band_differs_from_the_committed_file_by_the_insert_term_only`
- `cryptanalysis::ecbench::band::tests::pre_2026_11_08_pricing_reproduces_the_committed_targetwalk_band_file`
- `cryptanalysis::ecbench::band::tests::pricing_table_work_charges_the_walk_arms_and_resolves_the_rho_rank`
- `cryptanalysis::ecbench::band::tests::table_work_without_its_counted_marker_is_not_charged`
- `cryptanalysis::ecbench::compare::tests::default_and_pre_2026_11_08_readings_differ_in_the_band_and_the_id_only`
- `cryptanalysis::ecbench::spec::tests::a_committed_sessions_plan_still_expands_identically`
- `cryptanalysis::ecbench::workload::tests::registered_prime_curve_resolves_its_slug_and_ec1`
- `cryptanalysis::ecbench::workload::tests::registry_entry_matches_the_full_parse`
- `cryptanalysis::koblitz_index_calculus::tests::chained_s3_strategy_feeds_verified_relation_rank_and_target_solve`
- `ml_dataset::report::tests::checked_actual_worker_seal_verify_and_report_use_mock_counters`
- `ml_dataset::report::tests::checked_corrupt_partial_is_unpromoted_and_sealed_corruption_is_rejected`
- `ml_dataset::report::tests::empty_source_is_partial_and_oversized_json_is_rejected_before_parsing`
- `ml_dataset::report::tests::checked_registryless_start_only_survives_seal_verify_and_report`
- `ml_dataset::report::tests::partial_report_preserves_claims_pmu_and_attempts_without_fabricating_rows`
- `ml_dataset::report::tests::symlinks_and_overlapping_destinations_fail_without_creating_source_files`
- `ml_dataset::report::tests::tampering_and_snapshot_changes_never_replace_report_outputs`
- `ml_dataset::restore::tests::cross_host_roundtrip_replays_solved_cells_and_preserves_binding`
- `ml_dataset::restore::tests::oversized_remote_object_is_rejected_before_download_and_existing_hash_audit_remains`
- `ml_dataset::restore::tests::refuses_symlink_output_and_preserves_incomplete_local_attempt`
- `ml_dataset::restore::tests::rejects_receipt_substitution_disallowed_remote_and_different_run_caps`
- `ml_dataset::restore::tests::remote_corruption_retains_attempt_without_activating_manifest_then_retry_succeeds`
- `ml_dataset::tests::wide_prime_f4_central_replay_and_rehashed_tamper_both_twists`
- `ml_dataset::tests::wide_prime_m3_central_rehashed_tamper_and_censoring`
