# Tracked-source materialization and focused device controls

These are required development fixtures using the existing CPU emulator, not
physical GPU qualification or a scientific solver campaign. The full command
keeps its five original failures.

## Before materialization: exact arithmetic case, exit 101

Raw captured output: 684 bytes; SHA-256 `750ddefbc7996113170a2cd640c21ff1621ab2250ebe4a45fa3f4e5fdcbf2006`.

```text

running 1 test
test cryptanalysis::certicom_dlp::device_tests::device_source_matches_independent_rust_full_width_arithmetic ... FAILED

failures:

failures:
    cryptanalysis::certicom_dlp::device_tests::device_source_matches_independent_rust_full_width_arithmetic

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5057 filtered out; finished in 0.00s


thread 'cryptanalysis::certicom_dlp::device_tests::device_source_matches_independent_rust_full_width_arithmetic' (79633) panicked at src/cryptanalysis/certicom_dlp/device_tests.rs:32:18:
tracked gpu/certicom source must be materialized
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## After tracked sparse materialization: all six module cases, exit 0

Raw captured output: 805 bytes; SHA-256 `886982da06bb60281893a405be25337139877419826070c2f052f7f6518dc2b4`.

```text

running 6 tests
test cryptanalysis::certicom_dlp::device_tests::device_published_targets_stop_with_real_work_and_no_scalar ... ok
test cryptanalysis::certicom_dlp::device_tests::device_rho_bsgs_and_complete_ic_recover_exact_curve_fixtures ... ok
test cryptanalysis::certicom_dlp::device_tests::worker_backend_binding_encoding_and_candidate_checks_fail_closed ... ok
test cryptanalysis::certicom_dlp::device_tests::device_source_matches_independent_rust_full_width_arithmetic ... ok
test cryptanalysis::certicom_dlp::device_tests::worker_exit_and_deadline_cannot_hang_or_return_a_solution ... ok
test cryptanalysis::certicom_dlp::device_tests::expired_sessions_reject_no_io_results_for_every_solver ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 5052 filtered out; finished in 3.30s

```
