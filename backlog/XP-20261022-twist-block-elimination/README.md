# Frobenius blocks, hybrid elimination and bounded invariant encodings

**E26: proposed, blocked, not started.** Route 3; priority P2.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e26).
[Parent route card](../XP-20261021-twist-symmetry-solver/README.md).

## Question and decision value

Can splitting rational/anti-rational variable blocks or eliminating a small helper block reduce certified solver work on the same projected relation task?

A verified heldout stage gain can proceed to a complete pipeline comparison under E27. Otherwise record a loss, tie or incomplete cell for each frozen encoding; no coordinate reformulation is automatically called an advance.

## Inputs and method

The ten original ordinary fixtures, base sizes 8,12,16 and arities 2,3,4. Freeze four candidates: direct addition chain; Frobenius block order; enumeration of one at-most-eight-value helper followed by elimination; and one explicitly derived first-pair invariant encoding. Use training targets first and retain every failed encoding.

Prove and exhaustively test projection equivalence on the smallest fixtures, including signs, denominators and identity branches. If field equations are added, declare the finite-field task change and compare the same constrained task in every arm. Record Macaulay dimensions/nonzeros, reductions, complete-basis certificates and output lifting. Freeze variable orders and at most one selected candidate before heldout targets; use E25 for an applicable external reference.

## Controls and evidence owed

The best direct rational low-degree chain, all original encodings, identical finite bases and target rows, wrong-sign/denominator negative systems, enumeration overhead and an A/A run. Lower displayed degree or ideal equality without completeness cannot pass.

Native systems and exact projection certificates, complete/incomplete solver statuses, stage work, root extraction, memory and cost per independent verified projected row.

## Prerequisites

Candidate derivations, native adapters, full completeness/lifting checks and counters are missing.

Dependent designs: E25.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB, 30 seconds per solver cell; cap helper enumeration at eight values and at most four encodings. All branch solves are charged to the same candidate cell.

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
