# Current-parent focused diagnostic traces

The original full run has 13 observed failures: four ecbench cases and nine
loopback-network cases. A representative default-sandbox trace establishes
that local TCP bind is denied. The focused reruns below use permitted network
access for those public local-server fixtures; they do not replace the full
suite or authorize any cloud/scientific campaign. Library source matches
parent `543bbd3e` byte-for-byte.

## Original sandbox diagnostic

```text

running 1 test

thread 'cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http' (69395) panicked at src/cryptanalysis/pollard_collab/cairn.rs:1217:61:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http ... FAILED

failures:

failures:
    cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.00s


```

## cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity --nocapture`. Exit 101.

```text

running 1 test

thread 'cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity' (69406) panicked at src/cryptanalysis/ecbench/compare.rs:2448:9:
assertion failed: zero.legacy_comparison_id.is_none()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity ... FAILED

failures:

failures:
    cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.01s


```

## cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians --nocapture`. Exit 101.

```text

running 1 test

thread 'cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians' (69408) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1126:37:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians ... FAILED

failures:

failures:
    cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.08s


```

## cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed --nocapture`. Exit 101.

```text

running 1 test

thread 'cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed' (69410) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1126:37:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed ... FAILED

failures:

failures:
    cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.02s


```

## cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them --nocapture`. Exit 101.

```text

running 1 test

thread 'cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them' (69412) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1203:55:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them ... FAILED

failures:

failures:
    cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.01s


```

## cryptanalysis::pollard_collab::cairn::tests::a_key_submitter_signs_what_cairn_verifies

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::cairn::tests::a_key_submitter_signs_what_cairn_verifies --nocapture`. Exit 0.

```text

running 1 test
test cryptanalysis::pollard_collab::cairn::tests::a_key_submitter_signs_what_cairn_verifies ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.08s


```

## cryptanalysis::pollard_collab::cairn::tests::an_older_node_without_progress_turns_heartbeats_off_quietly

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::cairn::tests::an_older_node_without_progress_turns_heartbeats_off_quietly --nocapture`. Exit 0.

```text

running 1 test
[cairn] this node has no POST /progress (built before the dashboard); heartbeats off, claims unaffected
test cryptanalysis::pollard_collab::cairn::tests::an_older_node_without_progress_turns_heartbeats_off_quietly ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.00s


```

## cryptanalysis::pollard_collab::cairn::tests::generic_artifacts_use_the_persisted_commit_reveal_outbox

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::cairn::tests::generic_artifacts_use_the_persisted_commit_reveal_outbox --nocapture`. Exit 0.

```text

running 1 test
test cryptanalysis::pollard_collab::cairn::tests::generic_artifacts_use_the_persisted_commit_reveal_outbox ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.00s


```

## cryptanalysis::pollard_collab::cairn::tests::heartbeats_carry_the_sessions_own_counters_at_the_cadence

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::cairn::tests::heartbeats_carry_the_sessions_own_counters_at_the_cadence --nocapture`. Exit 0.

```text

running 1 test
test cryptanalysis::pollard_collab::cairn::tests::heartbeats_carry_the_sessions_own_counters_at_the_cadence ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.00s


```

## cryptanalysis::pollard_collab::cairn::tests::pending_commitments_survive_a_restart

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::cairn::tests::pending_commitments_survive_a_restart --nocapture`. Exit 0.

```text

running 1 test
test cryptanalysis::pollard_collab::cairn::tests::pending_commitments_survive_a_restart ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.00s


```

## cryptanalysis::pollard_collab::cairn::tests::points_travel_through_cairn_as_commit_reveal_claims_and_solve

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::cairn::tests::points_travel_through_cairn_as_commit_reveal_claims_and_solve --nocapture`. Exit 0.

```text

running 1 test
test cryptanalysis::pollard_collab::cairn::tests::points_travel_through_cairn_as_commit_reveal_claims_and_solve ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.06s


```

## cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http --nocapture`. Exit 0.

```text

running 1 test
test cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.00s


```

## cryptanalysis::pollard_collab::net::tests::foreign_job_is_refused

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::net::tests::foreign_job_is_refused --nocapture`. Exit 0.

```text

running 1 test
test cryptanalysis::pollard_collab::net::tests::foreign_job_is_refused ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.02s


```

## cryptanalysis::pollard_collab::net::tests::three_nodes_in_a_chain_converge

Command: `target/release/deps/crypto_lib-af1c9b7f8f6f9b1d --exact cryptanalysis::pollard_collab::net::tests::three_nodes_in_a_chain_converge --nocapture`. Exit 0.

```text

running 1 test
test cryptanalysis::pollard_collab::net::tests::three_nodes_in_a_chain_converge ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5036 filtered out; finished in 0.43s


```
