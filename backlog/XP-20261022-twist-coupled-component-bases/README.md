# Restrictions coupling the two Frobenius components

**Current status, 2026-10-23: partial / blocked / unfunded.** 120 coupled census cells and 456 evaluated relation cells; all evaluated projected row sets match the same-base rational control. Full polynomial collector and complete costs remain missing.

[Current bounded results](../../research/twist_routes_1_3_20261021/reports/followup-20261023/E21.md) · [Raw publication/extraction status](../../research/twist_routes_1_3_20261021/execution-v2/README.md). Local phase receipts do not complete the original experiment.

The original 2026-10-22 design and full requirements follow. Its “not started” wording records that historical design date.


**E21: proposed, blocked, not started.** Route 1; priority P1.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e21).
[Parent route card](../XP-20261021-twist-trace-factor-base/README.md).

## Question and decision value

Can a predicate involving both the rational and anti-rational components find useful relations more cheaply than direct collection on exactly the same projected base?

Only a lower verified cost per independent projected row on heldout data justifies a solver/pipeline extension. Equal projected rows with no cost decrease close the tested predicate. No result bypasses Frobenius separation.

## Inputs and method

Use the ten ordinary fixtures and the fixed training/holdout split above. For general extension points P define `T=P+Frob(P)` and `U=P-Frob(P)`. Test `x(T)+lambda*x(U) in W_h` and `x(T)*x(U) in W_h`, with lambda=1,2,3 and h=2,4,8; the product predicate is one family independent of lambda, giving 12 distinct configurations. Here x(U) uses the embedded twist coordinate and `W_h={0,...,h-1}` in Fp.

First enumerate all extension points at p=59,211,251. On larger fixtures use at most 2^16 deterministic public point proposals per predicate. Record every trace fiber and construct signed projected bases of sizes 8,12,16 in canonical encoding order. Compare the auxiliary system, direct rational collection on exactly that base, a rational elimination of the auxiliary predicate, and a matched random base. Handle T=O or U=O as separately enumerated branches. Freeze at most two configurations using training data before opening holdout outcomes.

## Controls and evidence owed

Pure-twist helpers, uncoupled predicates with matched acceptance rate, duplicated witnesses, shuffled auxiliary labels, exact same projected base and the strongest ordinary decomposition method. Unsupported or undersized bases remain failed cells.

Exact projected sets, fiber histograms, canonical signed rows, target-row span, verified scalar recovery and complete construction/collection/projection costs. Distinguish a new row from a cheaper way to find an existing row.

## Prerequisites

Coupled-family constructors, complete counters and independent projected-row replay are required.

No new experiment dependency; the linked frozen evidence is the starting point.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB; at most 2^20 counted group operations per relation cell and 60 seconds as a separate fail-safe. All construction and failed cells count. Exhaustive and sampled populations are reported separately.

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
