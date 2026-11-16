# Upper-size point-driven pencil elimination and full cohort

- **Status:** `investigating`
- **Kind:** empirical bottleneck, resource limit, and assistance restriction
- **Affected work:** [four-follow-up protocol](../research/four_followups_20261021/PROTOCOL.md), [point-driven pencil collector](../research/pencil_sparse_20261022/REPORT.md)
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-24

The ordered, machine-checkable continuation plan is recorded in
[`NEXT_STEPS.md`](../research/four_followups_review_20261022/NEXT_STEPS.md)
and [`NEXT_STEPS.json`](../research/four_followups_review_20261022/NEXT_STEPS.json).
It preserves the full cohort and separates metadata/intake preparation from
the independently authorized primary and replay execution.

## Problem

The point-driven pencil collector has not completed the frozen 1,260-target
cohort because the current sparse modular elimination performs large row-update
volumes at upper sizes. The full requirement is 21 original exact-order models,
60 separate cold targets per model, independently replayed and priced against
same-process source-negation rho and the two-point control.

## Evidence and uncertainty

- **Observed:** [Nine complete curve files](../research/four_followups_20261021/results/pencil-pilot/)
  contain 540/540 source-verified candidate and rho pairs, with independent
  non-wall replay and scalar checks. At p=12,143 they record 19,113,208,311
  modular row updates across 60 targets. The first p=48,383 target records
  20,814,919,424 updates, 23,639 verified relations, 1,723.90 seconds L0
  wall diagnostic and a complete operation-count price. A second 1,668.79-second
  run matched every non-wall field, and the independent validator checked the
  candidate/rho source scalars and three headline prices. See the
  [single-target record](../research/four_followups_20261021/results/pencil-pilot/f48383-one-target.jsonl)
  and [certificate](../research/four_followups_20261021/results/pencil-pilot/f48383-one-target-replay-certificate.json).
- **Observed, 2026-10-22:** The [native cohort status](../research/pencil_sparse_20261022/results/status.json)
  records 543 of the original 1,260 distinct targets solved and source verified;
  717 remain open. The [validator](../research/pencil_sparse_20261022/results/validation.json)
  checked 2,895 source scalars, 1,482 independent replay rows, 52,056 priced
  cells and 108 deliberately unpriced cap cells. At the first p=186,379 target,
  online elimination and degree peeling both reached the 10-million-coefficient
  cap. A separately frozen [25-million-coefficient extension](../research/pencil_sparse_20261022/results/p186379-peel-cap25m-t0.jsonl)
  also returned `sparse_matrix_cap`, at rank 85,168, peak 25,000,079 stored
  coefficients, 79,696,712 modular row updates and 4,778,336,117 index
  operations. Its [independent replay](../research/pencil_sparse_20261022/results/replay/p186379-peel-cap25m-t0.jsonl)
  matched non-wall fields; rho's source scalar verified. All three cap records
  retain null candidate scalar and ratio. The 8 GiB and 3,600-second guards
  did not trigger in the 25-million run.
- **Observed, Markowitz-priority continuation:** The [separate study](../research/pencil_markowitz_20261022/REPORT.md)
  held that source workload, relation collector, and 25m cap fixed. Its
  primary record stopped at 25,000,069 peak coefficients, rank 85,463,
  95,648,928 modular row updates and 36,545,252,445 logical index
  operations. Its candidate scalar and ratio are null; the same-process
  rho scalar verified. Its [independent replay](../research/pencil_markowitz_20261022/results/replay/p186379-markowitz-t0.jsonl)
  matched all non-wall fields, and the [native validator](../research/pencil_markowitz_20261022/results/validation.json)
  checked the source scalar, the null candidate price in 36 cells, and
  the matched rho prices.
- **Derived or proved:** No asymptotic lower bound for the pencil method or
  elimination is established. The 28-bit single target is not a 60-target
  size estimate. On the observed p=186,379 relation prefix, there are
  93,307 rows and 93,187 factor-base columns, so the left nullspace of
  the uneliminated factor-base matrix has dimension at least 120 over the
  recorded prime subgroup field. This count alone does not prove that a
  dependency has a nonzero target label.
- **Suspected explanation:** Dense fill and repeated modular updates may
  dominate as the pencil factor base grows; the raw nonzero counts and
  update counters distinguish this from candidate-verification work.
- **Missing capability:** A scalable exact elimination implementation or
  execution resources for the unchanged upper-size workloads and their
  independent replay. In the current assisted session, automatic approval
  review also rejected continuation of the exact sparse-nullspace DLP arm as
  possible key-recovery assistance. No workaround was attempted. This is an
  assistance restriction, not evidence that the algorithm cannot work.
- **What this evidence does not establish:** No full 1,260-target result or
  growing-size cost conclusion for the pencil path. Raising the coefficient
  cap from 10 million to 25 million on one target did not yield a solve; it
  does not prove that exact elimination cannot solve that target.

## What would unblock it

- [ ] Run the remaining 717 frozen targets, retaining per-target failures and
  complete phase, field, native, setup, elimination and source-rho counts.
- [ ] Independently replay all records, verify both recovered scalars on the
  source curve, recompute all 36 prices per model and compare with the frozen
  two-point control.

The exact open inventory is expected to contain 177 remaining 28-bit targets
and 180 targets at each of 32, 36, and 40 bits. Step N1 must establish this by
set equality against the frozen archive before any new execution. Attempt
coverage and verified candidate coverage remain separate: a retained cap or
failure can complete an attempted workload record but cannot contribute a
candidate/rho ratio.

These conditions would give an end-to-end operation-count cohort. Qualified
wall-speed claims would additionally need the repository's isolation and
external-tool checks.

## Next discriminating step

- **Action or proof obligation:** Export the already verified, uneliminated
  p=186,379 target-0 relation matrix in native code, preserving the ordered
  relation labels and an input digest. Test an exact black-box left-nullspace
  computation over the archived prime subgroup field, for example a
  preconditioned Wiedemann/Krylov pass on `A D A^T`. For every proposed
  vector `w`, check `A^T w = 0` exactly, require a nonzero combined target
  label, derive the scalar, and verify it by source-curve multiplication.
  A nullspace vector without that target label is not a target solve.
  This proposed action is not authorized in the current assisted session; it
  remains the recheck action for an independently authorized research setting.
- **Inputs and controls:** Frozen workload hash, seed, relation stream,
  source rho and source-scalar checks from the [protocol](../research/four_followups_20261021/PROTOCOL.md).
- **Execution limits and stop condition:** The observed 93,307 rows contain
  at most three factor-base terms each, so the original matrix has at most
  279,921 nonzeros before label columns. A CSR encoding with 32-bit columns
  and coefficients plus row offsets and two label vectors needs under 4 MiB
  for those arrays; transpose and workspaces need separate measurement.
  One `A D A^T` application visits at most 559,842 original coefficients.
  A single-seed scalar Wiedemann pilot with at most `3 × 93,307` such
  applications has an upper accounting bound of about 1.6 × 10^11 sparse
  coefficient visits, excluding polynomial arithmetic. Preregister one
  8 GiB-address, 7,200-second attempt and a 2 × 10^11-visit counter cap;
  retain a stopped or unusable-kernel result with a null candidate/rho
  ratio. These are proposed guards, not measured execution costs.
- **Outcomes and next actions:** A source-verified scalar with exact
  residual and independent non-wall replay would justify an unchanged
  full-cohort arm with all 36 operation prices against same-process rho.
  A failed residual, zero target label, resource cap, or replay mismatch
  remains a failed or inconclusive one-target attempt. Record its counters
  without extrapolating to the remaining curves.
- **Recheck trigger:** A new exact elimination arm or more complete upper-size targets.

## Resolution and history

2026-10-21: Nine smaller models complete; one 28-bit target independently
replayed. The upper-size full cohort remains unresolved.
2026-10-22: Degree peeling completed all nine 60-target pilot curves and two
additional 28-bit first targets. The 25-million-coefficient p=186,379 attempt
and independent replay both ended at the coefficient cap; 543/1,260 distinct
targets are solved and 717 remain open. See the [three-cap report](../research/pencil_sparse_20261022/reports/run-20261022T0647Z/RUN_REPORT.pdf)
and [study validation gate](../research/pencil_sparse_20261022/VALIDATION.json).

2026-10-22: The Markowitz-priority p=186,379 primary also reached its 25m cap
with no candidate scalar; its independent replay matched. The exact sparse
nullspace gate above is proposed and unrun. Unresolved.

2026-10-24: Automatic approval review rejected assisted continuation of the
exact sparse-nullspace DLP arm as possible key-recovery assistance. No bypass
or substitute run was attempted. The observed 543/1,260 coverage and the
mathematical and resource conclusions above are unchanged; the restriction is
recorded separately from them. Unresolved.
This record does not close or replace the requested experiment.
