# Other small gluing primes and actual torsion modules

**E20: proposed, blocked, not started.** Route 2; priority P2.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e20).
[Parent route card](../XP-20261021-twist-cover-transfer/README.md).

## Question and decision value

Do gluing primes 5, 7, 11, 13 or 17 expose additional public-parameter candidates that the 3-torsion filter misses?

A verified positive adds a specifically named construction candidate. An all-negative table excludes only this finite prime set. Rising map/construction costs are reported before expanding gluing degree.

## Inputs and method

All ten ordinary toy fixtures and all five public NIST P-curves, crossed with primes 5,7,11,13,17. Use the original public orders to reconstruct trace and characteristic. Reject a cell whose gluing prime is the characteristic or divides the selected source subgroup order.

Derive and independently check the actual necessary Frobenius conditions, then solve the anti-intertwiner linear system and determinant condition. Separate companion-matrix results from actual torsion action, especially scalar/repeated-eigenvalue cases. For each prime, select the first two positive toy fixture IDs in lexical order, if any, for explicit torsion/pairing replay; retain a no-positive status rather than generating a favorable replacement fixture.

## Controls and evidence owed

Wrong-trace and wrong-determinant matrices, the known 3-torsion panel as a regression control, characteristic-prime rejection and a repeated-eigenvalue synthetic module control. Matrix counts are not geometric certificates.

A complete 75-cell ordinary/public filter table, all exclusions, module types, candidate matrices and bounded actual-torsion replay. Actual quotients at the larger degrees remain a separately frozen extension of E19.

## Prerequisites

A generalized native module enumerator and actual-torsion checker are not yet implemented.

No new experiment dependency; the linked frozen evidence is the starting point.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB, 15 seconds per matrix cell; at most two explicit toy torsion constructions per prime with a 1,800-second cap each. No public large-prime torsion construction is included in this pilot.

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
