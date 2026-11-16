# Native allocation example tests

Command: `cargo test --release --example conductor_gap_confirmation --offline`.
Exit status: 0. The captured output is retained unchanged inside this fence.

```text
    Blocking waiting for file lock on build directory
   Compiling crypto v0.1.0 (/workspace/crypto)
    Finished `release` profile [optimized] target(s) in 5m 12s
     Running unittests examples/conductor_gap_confirmation.rs (target/release/examples/conductor_gap_confirmation-f18a5fd31bf50148)

running 12 tests
test tests::execute_and_malformed_options_fail_closed ... ok
test tests::cohort_identity_tampering_is_rejected ... ok
test tests::incomplete_cohort_target_arrays_and_indices_are_rejected ... ok
test tests::out_of_range_and_post_selected_targets_are_rejected ... ok
test tests::parser_rejects_unequal_seed_counts_and_unrecognized_fields ... ok
test tests::replaying_a_scalar_cannot_add_a_target ... ok
test tests::seeds_cannot_be_shared_between_curves_or_replicates ... ok
test tests::unsupported_readiness_and_confirmation_claims_are_rejected ... ok
test tests::zero_post_selected_and_cross_class_seeds_are_rejected ... ok
test tests::full_cohort_allocation_is_not_an_executed_pilot ... ok
test tests::existing_allocations_are_not_overwritten ... ok
test tests::missing_duplicate_or_reordered_blocks_are_rejected ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

```
