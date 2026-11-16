# Concrete separation and projected target-span certificates

**Current status, 2026-10-23: partial / blocked / unfunded.** Finite coordinate/Frobenius/projector replay passes; 17 Lean theorems have a clean durable axiom audit. Exact twist-group embedding, odd-subgroup/projector, row-module and serialization bridges remain open.

[Current bounded results](../../research/twist_routes_1_3_20261021/reports/followup-20261023/E29.md) · [Raw publication/extraction status](../../research/twist_routes_1_3_20261021/execution-v2/README.md). Local phase receipts do not complete the original experiment.

The original 2026-10-22 design and full requirements follow. Its “not started” wording records that historical design date.


**E29: proposed, blocked, not started.** Route 1; priority P1.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e29).
[Parent route card](../XP-20261021-twist-trace-factor-base/README.md).

## Question and decision value

Can the existing abstract Frobenius-separation argument be connected to the actual finite-field embedding and to the exact row transformations used by an auxiliary collector?

A checked bridge constrains E23 and closes part of the original proof map. Failure identifies the exact missing hypothesis or implementation defect. It does not retire the coupled-representation or geometric routes.

## Inputs and method

The existing Lean project and exact p=59 finite-field/twist embedding, then the three frozen even/composite controls. Consume E23 relation matrices only after a separate group replay; synthetic matrices with invalid twist-log columns are negative controls.

Prove the concrete twist embedding and Frobenius identities, instantiate the odd-subgroup/projector hypotheses, and connect each recorded matrix elimination to the projected rational row module. Ship the relevant identity certificates and Lean proofs without sorry or custom axioms. Search bounded even/torsion controls for counterexamples to each deliberately weakened hypothesis. State precisely which equalities are general proofs, exhaustive finite checks or samples.

## Controls and evidence owed

Remove the odd-order assumption, use a wrong Frobenius sign, omit the factor-two normalization, inject a fake twist-log unknown and supply a noninvertible projector denominator. Each must fail verification or trigger the correctly typed exception.

Lean source and axiom audit, concrete field/embedding certificates, exact matrix transforms, target-span replay and negative controls. The proof must not conclude that equivalent encodings have equal computational cost.

## Prerequisites

Concrete finite-field/Frobenius/twist and matrix-row-module formal bridges are unimplemented.

No new experiment dependency; the linked frozen evidence is the starting point.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB, 600 seconds per Lean module/build shard and exhaustive finite checks only on the smallest frozen fixtures. Compilation failure or cap exhaustion leaves the bridge open.

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
