# Explicit zero-weight ecbench comparison disagrees with its legacy-ID test

- **Status:** `open`
- **Kind:** implementation/test contract mismatch; intended compatibility semantics unresolved
- **Affected work:** full `cargo test --release --lib` validation on baseline `a679b78eea8687d9f3bbe04bb42b166b512cfeae`; [quotient-calibration relocation validation](../research/quotient_calibration_20261025/reports/run-20261025T1845Z/VALIDATION.md)
- **Next action owner:** ecbench comparison maintainer; unassigned
- **Last updated:** 2026-10-25

## Problem

The unchanged baseline does not pass
`cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity`:
an explicit `Some(0.0)` pricing weight produces no legacy comparison ID, but
the test requires one. This prevents a claim of a green whole-library gate
for the relocation branch, independently of its passing standalone preflight.

## Evidence and uncertainty

- **Observed:** the full release-library attempt reports this test failed.
  A focused rerun on the same unchanged library source also exits 101, with
  `assertion failed: zero.legacy_comparison_id.is_some()` at
  `src/cryptanalysis/ecbench/compare.rs:2433`.
- **Observed:** the implementation at the recorded baseline, line 1163,
  sets `legacy_comparison_id: weight.is_none().then_some(legacy_id)`.
  The test calls the weighted comparison with `Some(0.0)` and expects that
  field to be present. The branch changes no `src/`, root Cargo manifest,
  lockfile or library test source.
- **Evidence:** reproduction command and full log hashes/archive are in the
  linked validation report. The native quotient-calibration tests pass and
  do not exercise this comparison API.
- **Derived:** the displayed implementation condition is false for
  `Some(0.0)`, explaining this assertion failure under those exact inputs.
- **Suspected explanation:** the implementation and test encode different
  compatibility contracts. Which one is intended is unresolved.
- **Missing capability:** an agreed compatibility contract and its repair;
  no mathematical obstruction is claimed.
- **Not established:** this failure does not invalidate the native source
  preflight, certify any target map or say anything about attack cost.

## What would unblock it

- [ ] Decide and document whether an explicitly requested zero pricing
  weight must retain the old unweighted comparison identity.
- [ ] Align the implementation and regression tests with that contract,
  without changing frozen historical evidence or disabling the test.
- [ ] Pass the named focused test and the complete release-library suite
  on the repaired source, preserving logs and source hashes.

These conditions resolve this named gate failure. They do not execute any
target calibration or establish broader research claims.

## Next discriminating step

Run the existing native test with
`cargo test --release --lib cryptanalysis::ecbench::compare::tests::weighted_comparison_prices_verified_pairs_and_records_weight_identity -- --exact --nocapture`.
Compare the `None`, `Some(0.0)` and `Some(1.0)` API contracts against their
recorded identities before editing either side. Stop at this single contract
and its tests; no benchmark or new performance campaign is needed. A pass
after an agreed repair permits a complete suite rerun; a failure retains the
blocker. Recheck when the comparison contract or test changes.

## Resolution and history

2026-10-27: the trace-line implementation's whole-library attempt reproduced
the same unresolved compatibility area on base
`c8a37174b026362993b8c3becb75d520c5bc1d81`. The current assertion is now the
opposite of the historical one: `zero.legacy_comparison_id.is_none()` at
`compare.rs:2448` fails. Preserve the older receipt above; it describes that
older source, not the current symptom. The trace-line change does not alter
`compare.rs`, and an exact rerun after restoring its tracked session fixture
still fails. See
`research/trace_c1_2lp_20261027/validation/fixture-rechecks.log`.
The compatibility contract and whole-library gate remain open.

Unresolved. Discovered during the 2026-10-25 relocation gate. The separate
missing bounds-session fixture was materialized and its focused test passed;
it is not evidence of this compatibility defect. No library repair or test
suppression is included in the relocation PR.


### 2026-10-27 repair update

The current main baseline c8a37174 has the opposite mismatch from the historical baseline above: its implementation returns a legacy ID for explicit zero weight while its existing regression test requires no legacy ID. The CI repair preserves the current test contract: explicit pricing includes `pricing_weight` in its comparison identity, even at zero; only `None` carries a legacy unweighted ID. No historical comparison is edited. See `docs/ci-repair-20261027.md`; focused/final verification is still required before resolving this record.
