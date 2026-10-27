# Development validation traces

These commands validate software and repository inputs; they are not scientific measurements.

## artifact-tests

```text
...............artifacts: not a sha256 digest: '../../etc/passwd'
............
----------------------------------------------------------------------
Ran 27 tests in 3.815s

OK
```

## gate-tests

```text
.............................................F................
======================================================================
FAIL: test_gate_passes_on_the_committed_tree (__main__.Committed.test_gate_passes_on_the_committed_tree)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/workspace/crypto/scripts/test_validation_gate.py", line 529, in test_gate_passes_on_the_committed_tree
    self.assertEqual(g.check(REPO), [])
AssertionError: Lists differ: ['leaderboard board:icv1-fp10-t5-192cb216:[57707 chars]ile"] != []

First list contains 44 additional elements.
First extra element 0:
'leaderboard board:icv1-fp10-t5-192cb216:docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json: unvalidated: validator report raw_data ../../../research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json: ../../../research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json is not a file; sweep verdict validated_with_caveats, pricing_ok=False, rho_reference_ok=True, replay_ok=True, recomputed_ok=True, protocol_frozen_ok=partial (recognized, not passing); row is not in the pre-gate freeze'

Diff is 62631 characters long. Set self.maxDiff to None to see it.

----------------------------------------------------------------------
Ran 62 tests in 1.528s

FAILED (failures=1)
```

## site-tests

```text
missing site source: docs/backend-leaderboard/leaderboard.json
```

## workflow-wiring

```text
conductor-gap-replication.yml: needs `Pull store data this workflow reads (tools/artifacts)` for research/conductor_gap_replication_20261025
conductor-gap-size-certificates.yml: needs `Pull store data this workflow reads (tools/artifacts)` for research/conductor_gap_size_ladder_20261026
isolab-experiment.yml: checkout into a `path:` left unwired
isolab-experiment.yml: checkout into a `path:` left unwired

2 workflow(s) read store data without pulling it; run python3 tools/artifacts/wire_workflows.py
```

## cargo-check

```text
    Checking crypto v0.1.0 (/workspace/crypto)
    Finished `release` profile [optimized] target(s) in 1m 20s
```

## example-tests

```text
   Compiling crypto v0.1.0 (/workspace/crypto)
    Finished `release` profile [optimized] target(s) in 5m 21s
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

## native-build

```text
   Compiling crypto v0.1.0 (/workspace/crypto)
    Finished `release` profile [optimized] target(s) in 3.46s
```

## backlog-validation

```text
XP-20261026-conductor-gap-size-confirmation: ok
```
