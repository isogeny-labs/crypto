# Upper-size point-driven pencil elimination and full cohort

- **Status:** `investigating`
- **Kind:** empirical bottleneck and resource limit
- **Affected work:** [four-follow-up protocol](../research/four_followups_20261021/PROTOCOL.md), point-driven pencil collector
- **Next action owner:** PR #2304 follow-up
- **Last updated:** 2026-10-21

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
  growing-size cost conclusion for the pencil path.

## What would unblock it

- [ ] Run the remaining 719 frozen targets, retaining per-target failures and
  complete phase, field, native, setup, elimination and source-rho counts.
- [ ] Independently replay all records, verify both recovered scalars on the
  source curve, recompute all 36 prices per model and compare with the frozen
  two-point control.

These conditions would give an end-to-end operation-count cohort. Qualified
wall-speed claims would additionally need the repository's isolation and
external-tool checks.

## Next discriminating step

- **Action or proof obligation:** Profile row-update and stored-nonzero growth
  under the unchanged collector and compare an exact alternative sparse
  elimination on the same first 28-bit target.
- **Inputs and controls:** Frozen workload hash, seed, relation stream,
  source rho and source-scalar checks from the [protocol](../research/four_followups_20261021/PROTOCOL.md).
- **Execution limits and stop condition:** Record actual wall and memory use;
  terminate only on the frozen collector cap or an explicit resource failure,
  retaining partial records. Do not silently reduce the cohort.
- **Outcomes and next actions:** A validated faster elimination can be
  replayed on the remaining targets; an equal or slower result is recorded,
  and the unchanged collector still requires the full run. A replay mismatch
  invalidates the single-target price until resolved.
- **Recheck trigger:** A new exact elimination arm or more complete upper-size targets.

## Resolution and history

2026-10-21: Nine smaller models complete; one 28-bit target independently
replayed. The upper-size full cohort remains unresolved.
This record does not close or replace the requested experiment.
