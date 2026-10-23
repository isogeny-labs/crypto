# Complete primitive accounting and a cold one-target replay

**Current status, 2026-10-23: partial / blocked / unfunded.** Full primitive accounting, symmetric calibration and complete cold one-target replay remain unimplemented and unrun. High-level proxy counts are not a calibrated cost.

[Current bounded results](../../research/twist_routes_1_3_20261021/reports/followup-20261023/RESULTS.md) · [Raw publication/extraction status](../../research/twist_routes_1_3_20261021/execution-v2/README.md). No local run of this experiment has executed.

The original 2026-10-22 design and full requirements follow. Its “not started” wording records that historical design date.


**E27: proposed, blocked, not started.** Route 3; priority P0.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e27).
[Parent route card](../XP-20261021-twist-symmetry-solver/README.md).

## Question and decision value

Do any apparent IC stage or pipeline savings survive complete symmetric pricing of construction, solver, projection, verification and linear algebra?

A lower verified cost than the frozen best IC baseline is an IC improvement within this workload. Beating rho is a separate test. An unexplained sensitivity reversal or missing material cost blocks a complete-cost claim.

## Inputs and method

First instrument the original 24 arms on p=59,211,251. Then replay the full original ten-curve/eight-target grid with all 1,920 attempts, including empty-base cells. Freeze the best verified ordinary-IC baseline for each workload before evaluating any new E21/E23/E26 pipeline.

Instrument primitive field operations through all reference and candidate code, including inversions with a frozen algorithm, extension conversion, rejected candidates, F4/symbolic work, roots, signs/deduplication, matrix work and target recovery. Derive any GAE conversion from the declared counted reference law and expose its assumptions. Keep raw counts, setup and online intervals. Apply the 0/0.1/1 GAE sensitivity band symmetrically to any remaining unpriced units; if material units remain, the complete-cost conclusion remains unresolved. Obtain admitted isolation and A/A stability before making wall-time claims.

## Controls and evidence owed

Identical-base ordinary IC, the strongest matched rho available for each curve, a same-binary A/A arm, cold construction per target and independently recomputed counters. An error is an attempted cost, not a zero-cost solved workload.

Every phase and failed attempt, target-span stopping and verified scalar, peak RSS, count definitions, complete or explicitly incomplete costs, independent replay/aggregation and matched rho reference. Preserve the original campaign as a separate frozen version.

## Prerequisites

Complete counters, admitted calibration/isolation, independent aggregation and a runnable frozen protocol are missing.

No new experiment dependency; the linked frozen evidence is the starting point.

## Resources and cost derivation

Proposed pilot: one CPU and 4 GiB per arm, 120 seconds per one-target cell, checkpoint after each arm/target. Full-grid runtime remains unestimated; pilot measurements are required before scheduling it.

These are proposed process caps, not calibrated runtime/disk estimates. Full
core-hour and wall-hour estimates remain null until a pilot and runnable
protocol exist. Funding is zero, below the existing funded seed cards; no
allocation, lab publication or spending is performed. The card retains an
intentional failing command and is not runnable.

## Before ready and before done

- [ ] Implement native Rust driver and an independent checker.
- [ ] Freeze exact inputs, source, controls, seeds, counters, complete cost intervals and resource estimates.
- [ ] Commit and hash PROTOCOL.md; pin merged code; fill checkpoints and real verifier/run argv.
- [ ] Validate the ready card and preserve every failed or inconclusive cell.
- [ ] Land the bounded result, independent replay, report, diagram and inspected PDF.
- [ ] Meet the stated decision criterion; distinguish construction, stage and pipeline results.

Novelty and any IC speedup remain unestablished. The original E00–E17
requirements are preserved. This card records a proposal, not a completed run.
