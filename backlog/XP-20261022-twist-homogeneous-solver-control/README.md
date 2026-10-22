# Applicable homogeneous signature baseline with complete output certificates

**E25: proposed, blocked, not started.** Route 3; priority P0.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e25).
[Parent route card](../XP-20261021-twist-symmetry-solver/README.md).

## Question and decision value

Can a same-characteristic signature solver be compared fairly after homogenization, saturation and dehomogenization are fully verified and priced?

Only verified equal tasks admit a stage comparison. If no applicable signature implementation is available, that baseline remains missing rather than being replaced by a wrong-characteristic run.

## Inputs and method

Start with p=59,211,251, target seed 0 from the original solver fixtures, arities 2 and 3, all four original encodings and grevlex/lex: 48 input systems. After passing applicability/correctness, extend to the original 1,930 configured V3 cells with their original identities, not newly favorable replacements.

First verify the mode and executable really support the requested finite characteristic; if they do not, mark that backend inapplicable and select a supported implementation. Construct the homogenized ideal, saturate by the homogenizing variable where required, and separately record components at infinity. Check dehomogenized ideal membership both ways and replay all projected solutions. Certify a claimed complete basis by every required S-pair reduction and generator inclusion, or an independent complete-basis comparison. Ideal equality alone does not certify a Gröbner basis.

## Controls and evidence owed

Wrong-characteristic output, intentionally truncated bases, lost affine branches, spurious infinity components, same-field direct F4/q0 and a homogeneous positive control. Include all encoding/decoding, saturation and solution-extraction work.

Exact transformed inputs, actual field/mode/version, complete-basis or explicit incomplete status, projected solution sets, operations, memory and all failures. Compare with the fastest available applicable external implementation; name unavailable proprietary references.

## Prerequisites

Applicable signature backend, homogenization/saturation adapter and complete-basis checker are not yet pinned.

No new experiment dependency; the linked frozen evidence is the starting point.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB, 30 seconds per engine/system with the same hard envelope for each compared engine. Freeze timeout semantics and record extraction/checking separately before extending the grid.

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
