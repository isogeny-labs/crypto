# Focused library validation traces

The full library run has no final summary or confirmed exit status; its
execution session disappeared after the environment reattached. Its retained
local log is 429,118 bytes, SHA-256
`c4413bcd7d9a33a35be26903771c8391cda53a427730354361c273beb972a80d`.
It reports terminal status for 5,015 distinct tests out of 5,016 declared.
The unconfirmed case is
`cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
The full raw log has not been published to artifact storage; this is not a
complete-suite pass. These focused traces preserve the five observed failures.
The library's `src/` tree is byte-identical at f416adefe and current parent
2efb7830 (`2d7ba8b11b1414648c9f3ced61546fab002bd31e`).

Each case below ran as `TEST_BINARY --exact CASE --nocapture`, exit 101.
The bounds case subsequently passes after materializing its tracked fixture.

```text

running 1 test

thread 'cryptanalysis::ecbench::bounds::tests::committed_version_1_bounds_re_derive_under_their_recorded_version' (39835) panicked at src/cryptanalysis/ecbench/bounds.rs:2435:38:
called `Result::unwrap()` on an `Err` value: "/workspace/crypto/research/ecbench_bounds_challenges_20261006/sessions/prime-toy-reference-e1/session.json: No such file or directory (os error 2)"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::bounds::tests::committed_version_1_bounds_re_derive_under_their_recorded_version ... FAILED

failures:

failures:
    cryptanalysis::ecbench::bounds::tests::committed_version_1_bounds_re_derive_under_their_recorded_version

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5015 filtered out; finished in 0.00s

```

```text

running 1 test

thread 'cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity' (39838) panicked at src/cryptanalysis/ecbench/compare.rs:2448:9:
assertion failed: zero.legacy_comparison_id.is_none()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity ... FAILED

failures:

failures:
    cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5015 filtered out; finished in 0.00s

```

```text

running 1 test

thread 'cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians' (39841) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1126:37:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians ... FAILED

failures:

failures:
    cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5015 filtered out; finished in 0.09s

```

```text

running 1 test

thread 'cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed' (39844) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1126:37:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed ... FAILED

failures:

failures:
    cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5015 filtered out; finished in 0.04s

```

```text

running 1 test

thread 'cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them' (39847) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1203:55:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them ... FAILED

failures:

failures:
    cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5015 filtered out; finished in 0.06s

```

## Bounds check after materialization

Command: the same binary and exact case, after
`scripts/sparse-checkout.sh add --path research/ecbench_bounds_challenges_20261006/`.
Exit status: 0.

```text

running 1 test
test cryptanalysis::ecbench::bounds::tests::committed_version_1_bounds_re_derive_under_their_recorded_version ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5015 filtered out; finished in 0.04s

```
