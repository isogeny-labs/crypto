# Actual 3-torsion and pairing certificates

**Current status, 2026-10-23: partial / blocked / unfunded.** Four actual 3-torsion/pairing/Frobenius certificate replays pass, including public P-521. Canonical extension registration, quotient/polarization identification and full formal bridge remain open.

[Current bounded results](../../research/twist_routes_1_3_20261021/reports/followup-20261023/E18.md) · [Raw publication/extraction status](../../research/twist_routes_1_3_20261021/execution-v2/README.md). Local phase receipts do not complete the original experiment.

The original 2026-10-22 design and full requirements follow. Its “not started” wording records that historical design date.


**E18: proposed, blocked, not started.** Route 2; priority P1.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e18).
[Parent route card](../XP-20261021-twist-cover-transfer/README.md).

## Question and decision value

Do the four companion-matrix candidates lift to explicit Galois-equivariant anti-isometries on actual 3-torsion, including for public P-521 parameters?

A verified anti-isometry enables E19. A mismatch is a specific module-to-geometry defect to resolve. Extension/resource exhaustion leaves that candidate unresolved; it does not refute existence.

## Inputs and method

Use the three positive fixtures at p=211, 751 and 10111 from PUBLIC_3_MODULE_ANALYSIS.json, plus public P-521. Keep all seven negative ordinary toy rows and the four negative NIST rows as filter controls. Use the exact registered models, twist parameter and public orders; no NIST target logarithm is supplied.

Construct the 3-division polynomial and a declared extension field. Search extension degrees 1 through 8 in increasing order; certify irreducibility and all coordinate conversions. Give independent order-3 points P,Q, their primitive Weil pairing, and both Frobenius matrices in explicitly related symplectic bases. Replay every candidate map and check its graph is Galois stable and reverses the pairing. Native BigInt field/torsion support is a prerequisite for P-521; the toy u128 adapter is insufficient.

## Controls and evidence owed

A matrix with the wrong determinant, a deliberately conjugated wrong Frobenius matrix, dependent torsion points and each negative module row must fail the appropriate check. An empty matrix set is not passed to a point constructor as a success.

Exact field polynomials and bases, all torsion coordinates, basis-change matrices, pairing values, Frobenius images and independent verification transcripts. Record construction operations and peak memory, separately from the certificate conclusion.

## Prerequisites

Actual torsion/pairing implementation, native BigInt adapter and an independent certificate checker are unimplemented.

No new experiment dependency; the linked frozen evidence is the starting point.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB per process, 1,800 seconds per positive curve, extension degree at most 8. Preserve each attempted field and each failure. This is a cap, not an estimated runtime.

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
