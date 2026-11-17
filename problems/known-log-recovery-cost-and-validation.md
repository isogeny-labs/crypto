# Known-log arbitrary-target recovery cost and independent validation

- **Status:** `open`
- **Kind:** native-cost calibration gap; independent validation and comparator gaps
- **Affected work:** [known-log arbitrary-target study](../research/ecbench_known_log_targets_20261021/README.md)
- **Next action owner:** unassigned ecbench accounting and independent validation task
- **Last updated:** 2026-10-21

## Problem

The frozen small-curve study establishes local verified arbitrary-target
recovery and a lower cold emitted-cost total than its specified abscissa IC
pipeline on the 16-, 20- and 24-bit fixtures. It does not establish a fully
calibrated runtime improvement, an independently validated promotion, the best
IC configuration, a fastest-tool result or an ECDLP complexity improvement.
Negation rho remains cheaper in every measured cell. The 12-bit comparison
changes ordering inside the native-cost band.

## Evidence and uncertainty

- **Observed:** Eight retained sessions contain 4,484 executions: 4,340 verified,
  144 original projected-control setup errors, and 3,610 identical measured
  replays. The [native receipt cross-check](../research/ecbench_known_log_targets_20261021/verification/study-audits.json)
  binds the totals and confirms transported point sets and counted A/A equality.
- **Observed:** The [primary](../research/ecbench_known_log_targets_20261021/reports/run-20261021T0636Z-primary/summary.json)
  and [holdout](../research/ecbench_known_log_targets_20261021/reports/run-20261021T0636Z-holdout/summary.json)
  summaries price serialized native operations at 0, 0.1 and 1 GAE. Group setup,
  queries, recovery and verification are charged cold. All wall comparisons
  are L0 with zero admitted L2 pairs. Replays are on the measuring host.
- **Derived or proved:** A known-log full-point relation gives one equation in
  the single unknown target scalar. Its usefulness does not require solving
  unknown base logs. Generator transport is a scalar-coordinate conversion;
  exhaustive fixed-seed tests verify it, not a hardness change.
- **Suspected explanation:** Avoiding unknown-base linear algebra and repeated
  relation acquisition accounts for some counted savings. Its complete machine
  cost and any general scaling law remain unmeasured.
- **Missing capability:** Calibrated complete native pricing (including work
  not serialized by existing arms), L2 or stronger isolated wall measurements,
  independent replay and headline recomputation, and matched strongest
  rho/negation-BSGS/external implementations on identical public targets.
- **What this evidence does not establish:** An asymptotic improvement,
  standard-curve recovery advantage, causal cofactor gain, novel algebraic
  structure, actual multi-target amortization or fastest implementation.

The shared cost-pricing mechanism overlaps
[rootfinding-complete-cost-calibration](rootfinding-complete-cost-calibration.md);
the frozen known-log study has its own validation and comparator obligations.

## What would unblock it

- [ ] Independently replay every verified measured execution on another host
  class with the exact targets, method/source/binary bindings and recovered
  scalars checked. Preserve the original errors as errors.
- [ ] Have an independent validator recompute the headline per-curve totals,
  target-cluster intervals, A/A controls and 0/0.1/1 band from committed raw
  records; add real certificates meeting `docs/VALIDATION_GATE.md`.
- [ ] Serialize and calibrate remaining native work on all compared arms and
  repeat under L2 or stronger isolation with admitted A/A intervals, cold setup,
  online phases, failures and peak memory retained.
- [ ] Freeze and execute a same-instance factor-base-size sweep against the
  strongest applicable rho and optimized negation BSGS, plus the fastest
  available external implementation; verify equal answers before comparing.

The first two items permit validation of the present *modeled-cost* result.
The latter two permit a qualified complete-cost comparison at declared sizes.
None alone establishes an asymptotic improvement or transfers toy measurements
to deployed curves.

## Next discriminating step

- **Action or proof obligation:** Independently replay and recompute the frozen
  study while implementing complete native-cost serialization. Then freeze a
  new B sweep with fresh holdout targets; do not overwrite the current evidence.
- **Inputs and controls:** The exact registered curves, public target seeds,
  same-binary matched rho, identical-method A/A, transported-generator control,
  both conventional and projected-abscissa bases, and optimized negation BSGS.
- **Execution limits and stop condition:** Use the existing per-execution
  120-second timeout and recorded 4 GiB address-space limit for replay. Freeze
  any new sweep's full budget before measuring; retain refusals, timeouts and
  resource errors without substituting targets.
- **Outcomes and next actions:** A matched lower cold total validates a bounded
  implementation result; a loss after pricing is retained as a loss. A baseline
  change requires a new comparison. Infrastructure failure changes no
  mathematical conclusion. Rho superiority remains a measured result.
- **Recheck trigger:** Independent runner/reviewer availability, complete native
  instrumentation or access to a strongest matching external implementation.

## Resolution and history

2026-10-21: Opened after the primary, holdout, generator and corrected-cofactor
panels and all local measured replays completed. Unresolved. No leaderboard or
benchmark-index admission is claimed by this record.
