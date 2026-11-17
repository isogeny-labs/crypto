# Current-parent development validation traces

Source merge revision: `adfb858227a5a31585a12afb2cd55114dbcd2c81`.

Raw output is retained unchanged inside each fence. These checks are not
scientific observations or passing CI. The complete gate diagnostics and
native receipts are separate JSON files.

## cargo check --release --lib --example conductor_gap_confirmation --offline

```text
    Checking isogeny_algos v0.1.0 (/workspace/crypto/isogeny_algos)
    Checking volcano-cli v0.1.0 (/workspace/crypto/tools/volcano-cli)
    Checking crypto v0.1.0 (/workspace/crypto)
    Finished `release` profile [optimized] target(s) in 1m 26s

```

## cargo test --release --example conductor_gap_confirmation --offline

```text
   Compiling crypto v0.1.0 (/workspace/crypto)
    Finished `release` profile [optimized] target(s) in 9m 35s
     Running unittests examples/conductor_gap_confirmation.rs (target/release/examples/conductor_gap_confirmation-f18a5fd31bf50148)

running 12 tests
test tests::execute_and_malformed_options_fail_closed ... ok
test tests::cohort_identity_tampering_is_rejected ... ok
test tests::incomplete_cohort_target_arrays_and_indices_are_rejected ... ok
test tests::out_of_range_and_post_selected_targets_are_rejected ... ok
test tests::parser_rejects_unequal_seed_counts_and_unrecognized_fields ... ok
test tests::full_cohort_allocation_is_not_an_executed_pilot ... ok
test tests::existing_allocations_are_not_overwritten ... ok
test tests::unsupported_readiness_and_confirmation_claims_are_rejected ... ok
test tests::zero_post_selected_and_cross_class_seeds_are_rejected ... ok
test tests::seeds_cannot_be_shared_between_curves_or_replicates ... ok
test tests::replaying_a_scalar_cannot_add_a_target ... ok
test tests::missing_duplicate_or_reordered_blocks_are_rejected ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s


```

## cargo build --release --example conductor_gap_confirmation --bin backlog --offline

```text
   Compiling crypto v0.1.0 (/workspace/crypto)
    Finished `release` profile [optimized] target(s) in 7.90s

```

## python3 tools/artifacts/test_artifacts.py

```text
..............artifacts: not a sha256 digest: '../../etc/passwd'
....
----------------------------------------------------------------------
Ran 18 tests in 4.077s

OK

```

## python3 scripts/test_validation_gate.py (current verdict hydrated)

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
Ran 62 tests in 1.578s

FAILED (failures=1)

```

## python3 scripts/site/test_build.py (tracked site inputs materialized)

```text
missing site source: docs/backend-leaderboard/leaderboard.json

```

## python3 tools/artifacts/wire_workflows.py --check

```text
isolab-experiment.yml: checkout into a `path:` left unwired
isolab-experiment.yml: checkout into a `path:` left unwired

2 workflow(s) read store data without pulling it; run python3 tools/artifacts/wire_workflows.py
conductor-gap-replication.yml: needs `Pull store data this workflow reads (tools/artifacts)` for research/conductor_gap_replication_20261025
conductor-gap-size-certificates.yml: needs `Pull store data this workflow reads (tools/artifacts)` for research/conductor_gap_size_ladder_20261026

```

## target/release/backlog validate XP-20261026-conductor-gap-size-confirmation

```text
XP-20261026-conductor-gap-size-confirmation: ok

```

## Existing Git manifest metadata prefetch (unsuccessful setup attempt)

```text
fatal: bad revision '005df68659931e826c2eef55337e0dab4eaa15b1'
error: https://github.com/aburan28/crypto.git did not send all necessary objects

```
