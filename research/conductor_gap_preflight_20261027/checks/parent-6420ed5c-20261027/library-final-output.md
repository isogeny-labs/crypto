# Complete native failure footer

Only the final failure section is quoted below. The complete 436,010-byte raw
log remains local-only; its digest and the preserved observer-count correction
are in [the final receipt](library-final.json). No full-log artifact is disguised
as Markdown. The five original failures remain failed.

```text
failures:

---- cryptanalysis::certicom_dlp::device_tests::device_published_targets_stop_with_real_work_and_no_scalar stdout ----

thread 'cryptanalysis::certicom_dlp::device_tests::device_published_targets_stop_with_real_work_and_no_scalar' (76900) panicked at src/cryptanalysis/certicom_dlp/device_tests.rs:32:18:
tracked gpu/certicom source must be materialized

---- cryptanalysis::certicom_dlp::device_tests::device_rho_bsgs_and_complete_ic_recover_exact_curve_fixtures stdout ----

thread 'cryptanalysis::certicom_dlp::device_tests::device_rho_bsgs_and_complete_ic_recover_exact_curve_fixtures' (76901) panicked at src/cryptanalysis/certicom_dlp/device_tests.rs:32:18:
tracked gpu/certicom source must be materialized

---- cryptanalysis::certicom_dlp::device_tests::device_source_matches_independent_rust_full_width_arithmetic stdout ----

thread 'cryptanalysis::certicom_dlp::device_tests::device_source_matches_independent_rust_full_width_arithmetic' (76902) panicked at src/cryptanalysis/certicom_dlp/device_tests.rs:32:18:
tracked gpu/certicom source must be materialized

---- cryptanalysis::certicom_dlp::device_tests::expired_sessions_reject_no_io_results_for_every_solver stdout ----

thread 'cryptanalysis::certicom_dlp::device_tests::expired_sessions_reject_no_io_results_for_every_solver' (76903) panicked at src/cryptanalysis/certicom_dlp/device_tests.rs:32:18:
tracked gpu/certicom source must be materialized

---- cryptanalysis::certicom_dlp::device_tests::worker_backend_binding_encoding_and_candidate_checks_fail_closed stdout ----

thread 'cryptanalysis::certicom_dlp::device_tests::worker_backend_binding_encoding_and_candidate_checks_fail_closed' (76904) panicked at src/cryptanalysis/certicom_dlp/device_tests.rs:32:18:
tracked gpu/certicom source must be materialized


failures:
    cryptanalysis::certicom_dlp::device_tests::device_published_targets_stop_with_real_work_and_no_scalar
    cryptanalysis::certicom_dlp::device_tests::device_rho_bsgs_and_complete_ic_recover_exact_curve_fixtures
    cryptanalysis::certicom_dlp::device_tests::device_source_matches_independent_rust_full_width_arithmetic
    cryptanalysis::certicom_dlp::device_tests::expired_sessions_reject_no_io_results_for_every_solver
    cryptanalysis::certicom_dlp::device_tests::worker_backend_binding_encoding_and_candidate_checks_fail_closed

test result: FAILED. 4931 passed; 5 failed; 122 ignored; 0 measured; 0 filtered out; finished in 1904.05s

error: test failed, to rerun pass `--lib`
```
