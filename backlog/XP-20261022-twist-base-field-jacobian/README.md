# Base-field Jacobian arithmetic and independent arbitrary-class replay

**E24: proposed, blocked, not started.** Route 2; priority P1.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e24).
[Parent route card](../XP-20261021-twist-cover-transfer/README.md).

## Question and decision value

How much of the present cover cost comes from the Fp6 chart, and do arbitrary-class maps replay in an independent same-field representation?

A verified base-field stage improvement enables a correctly priced cover pipeline comparison. Missing independent arithmetic or exceptional charts leaves transfer validation partial. No destination decomposition solver is supplied merely by changing charts.

## Inputs and method

All ten frozen selected cover models and their explicit chart conversions. Replay all 3,551 classes on the smallest Jacobian, the original 256 public sampled classes on each other model, and all original map/associativity controls. Add E19 models only after their curve/map certificates pass.

Implement the actual balanced divisor conventions over Fp, including both infinity points and exceptional branches. Supply a checked conversion to the existing Fp6 chart and certify base-field descent. Use an independently implemented divisor law or an applicable external CAS for arbitrary-class addition, pullback and norm. Compare native operation counts on identical classes, not point counts as a substitute for divisor replay. Include degree-3/4 irreducible support factorization, rather than only split support.

## Controls and evidence owed

Origin, ramification, both infinity points, non-split supports, inverses, repeated support and the entire smallest class population. A mismatched infinity convention, wrong field conversion or non-descended class must fail.

Explicit model and divisor conventions, chart/field conversion certificates, independent class-map replay, factorization coverage, stage counters, peak memory and unsupported branches. Keep order certificates separate from group-law certificates.

## Prerequisites

Balanced Fp arithmetic, independent arbitrary-class checker and complete non-split factorization support are missing.

No new experiment dependency; the linked frozen evidence is the starting point.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB, 3,600 seconds per fixture with per-class checkpoints. Preserve partial exhaustive progress and failures; do not replace the required whole smallest population with samples.

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
