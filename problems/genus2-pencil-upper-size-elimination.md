# Upper-size point-driven pencil elimination and full cohort

- **Status:** `investigating`
- **Kind:** empirical bottleneck and resource limit
- **Affected work:** [four-follow-up protocol](../research/four_followups_20261021/PROTOCOL.md), [point-driven pencil collector](../research/pencil_sparse_20261022/REPORT.md)
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-22

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
- **Derived or proved:** No asymptotic lower bound for the pencil method or
  elimination is established. The 28-bit single target is not a 60-target
  size estimate.
- **Suspected explanation:** Dense fill and repeated modular updates may
  dominate as the pencil factor base grows; the raw nonzero counts and
  update counters distinguish this from candidate-verification work.
- **Missing capability:** A scalable exact elimination implementation or
  execution resources for the unchanged upper-size workloads and their
  independent replay.
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

These conditions would give an end-to-end operation-count cohort. Qualified
wall-speed claims would additionally need the repository's isolation and
external-tool checks.

## Next discriminating step

- **Action or proof obligation:** Compare a new exact sparse-elimination
  implementation on the same fixed p=186,379 target and relation stream,
  including its 25-million-coefficient cap record as the control.
- **Inputs and controls:** Frozen workload hash, seed, relation stream,
  source rho and source-scalar checks from the [protocol](../research/four_followups_20261021/PROTOCOL.md).
- **Execution limits and stop condition:** Freeze and record the alternative's
  coefficient, memory and time guards before execution; retain partial records
  on each guard or explicit resource failure. Do not silently reduce the cohort.
- **Outcomes and next actions:** A validated faster elimination can be
  replayed on the remaining targets; an equal or slower result is recorded,
  and the unchanged collector still requires the full run. A replay mismatch
  invalidates the single-target price until resolved.
- **Recheck trigger:** A new exact elimination arm or more complete upper-size targets.

## Resolution and history

2026-10-21: Nine smaller models complete; one 28-bit target independently
replayed. The upper-size full cohort remains unresolved.

2026-10-22: Degree peeling completed all nine 60-target pilot curves and two
additional 28-bit first targets. The 25-million-coefficient p=186,379 attempt
and independent replay both ended at the coefficient cap; 543/1,260 distinct
targets are solved and 717 remain open. See the [three-cap report](../research/pencil_sparse_20261022/reports/run-20261022T0647Z/RUN_REPORT.pdf)
and [study validation gate](../research/pencil_sparse_20261022/VALIDATION.json).
This record does not close or replace the requested experiment.
