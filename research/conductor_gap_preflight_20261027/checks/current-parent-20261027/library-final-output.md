# Completed full-library failure output

The full run completed with exit **101**; the unmodified raw failure footer
is below. The complete raw log is local-only and hash-pinned in
[library-final.json](library-final.json), not claimed uploaded. Nine sandbox
network failures cleared in the separately retained focused reruns; they are
not removed from this original result.

```text
failures:

---- cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity stdout ----

thread 'cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity' (63732) panicked at src/cryptanalysis/ecbench/compare.rs:2448:9:
assertion failed: zero.legacy_comparison_id.is_none()

---- cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians stdout ----

thread 'cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians' (64128) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1126:37:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"

---- cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed stdout ----

thread 'cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed' (64132) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1126:37:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"

---- cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them stdout ----

thread 'cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them' (64133) panicked at src/cryptanalysis/ecbench/hyper_ic.rs:1203:55:
called `Result::unwrap()` on an `Err` value: "method `ic.hyperelliptic` has no parameter `branches`"

---- cryptanalysis::pollard_collab::cairn::tests::a_key_submitter_signs_what_cairn_verifies stdout ----

thread 'cryptanalysis::pollard_collab::cairn::tests::a_key_submitter_signs_what_cairn_verifies' (67224) panicked at src/cryptanalysis/pollard_collab/cairn.rs:1217:61:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }

---- cryptanalysis::pollard_collab::cairn::tests::an_older_node_without_progress_turns_heartbeats_off_quietly stdout ----

thread 'cryptanalysis::pollard_collab::cairn::tests::an_older_node_without_progress_turns_heartbeats_off_quietly' (67226) panicked at src/cryptanalysis/pollard_collab/cairn.rs:1217:61:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }

---- cryptanalysis::pollard_collab::cairn::tests::generic_artifacts_use_the_persisted_commit_reveal_outbox stdout ----

thread 'cryptanalysis::pollard_collab::cairn::tests::generic_artifacts_use_the_persisted_commit_reveal_outbox' (67229) panicked at src/cryptanalysis/pollard_collab/cairn.rs:1217:61:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }

---- cryptanalysis::pollard_collab::cairn::tests::heartbeats_carry_the_sessions_own_counters_at_the_cadence stdout ----

thread 'cryptanalysis::pollard_collab::cairn::tests::heartbeats_carry_the_sessions_own_counters_at_the_cadence' (67230) panicked at src/cryptanalysis/pollard_collab/cairn.rs:1217:61:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }

---- cryptanalysis::pollard_collab::cairn::tests::pending_commitments_survive_a_restart stdout ----

thread 'cryptanalysis::pollard_collab::cairn::tests::pending_commitments_survive_a_restart' (67231) panicked at src/cryptanalysis/pollard_collab/cairn.rs:1217:61:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }

---- cryptanalysis::pollard_collab::cairn::tests::points_travel_through_cairn_as_commit_reveal_claims_and_solve stdout ----

thread 'cryptanalysis::pollard_collab::cairn::tests::points_travel_through_cairn_as_commit_reveal_claims_and_solve' (67232) panicked at src/cryptanalysis/pollard_collab/cairn.rs:1217:61:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }

---- cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http stdout ----

thread 'cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http' (67233) panicked at src/cryptanalysis/pollard_collab/cairn.rs:1217:61:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }

---- cryptanalysis::pollard_collab::net::tests::foreign_job_is_refused stdout ----

thread 'cryptanalysis::pollard_collab::net::tests::foreign_job_is_refused' (67243) panicked at src/cryptanalysis/pollard_collab/net.rs:359:84:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }

---- cryptanalysis::pollard_collab::net::tests::three_nodes_in_a_chain_converge stdout ----

thread 'cryptanalysis::pollard_collab::net::tests::three_nodes_in_a_chain_converge' (67245) panicked at src/cryptanalysis/pollard_collab/net.rs:304:88:
called `Result::unwrap()` on an `Err` value: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }


failures:
    cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity
    cryptanalysis::ecbench::hyper_ic::tests::ecbench_recovers_planted_logs_on_thread_jacobians
    cryptanalysis::ecbench::hyper_ic::tests::ecbench_runs_are_deterministic_in_the_seed
    cryptanalysis::ecbench::hyper_ic::tests::ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them
    cryptanalysis::pollard_collab::cairn::tests::a_key_submitter_signs_what_cairn_verifies
    cryptanalysis::pollard_collab::cairn::tests::an_older_node_without_progress_turns_heartbeats_off_quietly
    cryptanalysis::pollard_collab::cairn::tests::generic_artifacts_use_the_persisted_commit_reveal_outbox
    cryptanalysis::pollard_collab::cairn::tests::heartbeats_carry_the_sessions_own_counters_at_the_cadence
    cryptanalysis::pollard_collab::cairn::tests::pending_commitments_survive_a_restart
    cryptanalysis::pollard_collab::cairn::tests::points_travel_through_cairn_as_commit_reveal_claims_and_solve
    cryptanalysis::pollard_collab::cairn::tests::the_client_speaks_enough_http
    cryptanalysis::pollard_collab::net::tests::foreign_job_is_refused
    cryptanalysis::pollard_collab::net::tests::three_nodes_in_a_chain_converge

test result: FAILED. 4902 passed; 13 failed; 122 ignored; 0 measured; 0 filtered out; finished in 2572.04s

error: test failed, to rerun pass `--lib`

```
