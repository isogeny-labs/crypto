# Heldout family selection and an honest solver portfolio

**Current status, 2026-10-23: partial / blocked / unfunded.** Final labels 32–63 remain unopened; no final heldout run. E21 diagnostic selection is not a substitute for a frozen complete-cost selector.

[Current bounded results](../../research/twist_routes_1_3_20261021/reports/followup-20261023/RESULTS.md) · [Raw publication/extraction status](../../research/twist_routes_1_3_20261021/execution-v2/README.md). No local run of this experiment has executed.

The original 2026-10-22 design and full requirements follow. Its “not started” wording records that historical design date.


**E28: proposed, blocked, not started.** Route 3; priority P2.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e28).
[Parent route card](../XP-20261021-twist-symmetry-solver/README.md).

## Question and decision value

Can a selector using only public instance features improve cold one-target IC after all search and selection costs are included?

A complete-cost reduction on the sealed final population supports an adaptive engineering improvement and justifies independent replication of this frozen selector. A training-only gain, oracle result or failed heldout transfer does not support that conclusion.

## Inputs and method

Only completed training outputs from E21/E22/E26 and the complete counters from E27. Freeze at most four candidate configurations and one selector. Final evaluation uses fresh target labels 32 through 63 on the four ladder-replicate-1 holdout curves; earlier labels 16 through 31 are validation, not fresh final evidence.

Use public coefficients, field size, base size, degree/sparsity and budgeted pilot statistics as features. Freeze a deterministic rule on the training curves and labels 0 through 15; hash it before final targets are generated. Charge feature extraction, pilot failures and any per-target tuning. Report cold one-target cost first; any amortized reuse is a separate workload with an explicit target count and setup interval.

## Controls and evidence owed

Best frozen fixed IC configuration, matched rho, a random selector with the same pilot budget, an A/A selector and a hindsight oracle clearly labeled non-executable. Neither verifier-only known logs nor final-target outcomes may enter the selector.

Selector source/hash and allowed features, every candidate/pilot attempt, choices, failures, verified final scalars and paired complete costs. Evaluate all final cells, including unsupported choices; do not replace them after inspection.

## Prerequisites

Verified candidates, complete counters, a frozen selector and unopened final target set are required.

Dependent designs: E21, E22, E26, E27.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB, at most four configurations and two budgeted pilot probes per target; 120 seconds total per selector cell, including probes. Final resource estimates await E27.

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
