# Boolean offsets of a quadratic ordinate coordinate

2026-11-01. Search continuation for complete prime-field factor bases with
lower structured degree and lower relation-solving work.

The new family uses `y = ±(a*x²+b*x+c+t0+sum(w_i*u_i))`, with Boolean
`u_i` and independent labels for both summands. Every rational intersection
is retained; individual graph fibres need not split completely. Lean proves
exact group semantics and canonical homogeneous first-vanishing degree
**2k+3**, including all **2k** selector variables, for two k-bit slots.

The concrete F17 base has **eight columns and sixteen signed points**, twice
the earlier split-graph fixture's sizes. Its complete system has canonical
degree **7**, compared with **9** for the same-base dense presentation.
These are proved homogeneous invariants, not measured affine solving degrees.

| Item | Status and evidence |
| --- | --- |
| Generic complete relation semantics | Checked: `PencilSelector.complete_selector_iff`, `system_implies_base_relation` |
| Actual canonical highest forms, including every bit | Checked: `PencilCanonical.canonical_top_ideal` |
| Exact upper and lower degree certificates | Checked: `PencilTop.first_vanishing`, `PencilCanonical.canonical_first_vanishing` |
| Concrete eight-column membership and signed cardinality | Checked: `PencilFixture.exact_membership`, `signed_cardinality` |
| Actual polynomial-root/group bridge | Checked: `PencilFixture.polynomial_complete`, `roots_imply_relation` |
| Same-base dense comparator | Checked polynomial identity and canonical degree9 in `PencilFixture` |
| Exact removal of one selector per slot | Checked semantics: `PencilElimination.two_selectors_iff`; projected degree still open |
| FB1 and identity export | Native exporter; 13 unit tests; byte-identical export replay |
| Matched solving/yield experiment | Protocol recorded; runner/controls not yet implemented; no job launched |
| Lower complete solving cost or golden-base identification | Not established; requires the matched experiment |
| 17-term P-256 selector and full-depth relation | Not implemented here; original promotion gates remain open |
| External novelty | Not established; see the recorded prior-art search |

## Exact fixture

- Curve: `icv1-fp5-tm1-a77cc6af`, `y²=x³+2x+2` over F17.
- Factor base: **FB1h29cc024e755c**, in [the existing FB1 dump format](fixture/FB1h29cc024e755c.json).
- `h0=4x²+9x+9`, weights `[2,10]`, offsets `{0,2,10,12}`.
- Positive fibre sizes `[4,1,2,2]`: nine positive labels and eighteen signed
  labels reduce to sixteen distinct points and eight abscissae.
- Abscissae: `{0,3,5,6,7,9,10,16}`. The two signed duplicate labels occur
  at x=9. Keep multiplicity in accounting, not as extra columns.

This fixture was hand-derived after inspecting the old F17 example. It is
not a prospective trial or evidence of natural construction yield. Curve
metadata is inherited from the previous fixture, with `registered=false`;
no group-order or registry proof is added.

## What to compare next

The full affine monomial envelopes `choose(N+D,N)` are 3432 for the new
seven-variable degree7 presentation and 55 for the two-variable dense
degree9 presentation. These derived ambient budgets are not observed F4
widths. The degree reduction alone does not lower this budget.

Exact one-bit-per-slot projection removes two variables without losing
solutions and introduces quartic residuals. Its actual canonical degree and
affine solving work are the next obligations. The prospective protocol
also includes dense, low-x, hash-x and ordinary-coordinate digit controls.
Empty fibres, duplicate labels, every sign and construction work remain charged.
Relations use field-valued roots: algebraic-extension roots are not base
points, and rational-root recovery must also be charged.

The repository defaults new CPU experiments to EKS Spot. The present tool
surface has no arbitrary-command EKS submission tool or ready AWS identity;
the inspected GitHub EKS workflow consumes ecbench specifications. This panel
is not wired into that path. The old local exception was specific to its
completed panel and is not reused. Compilation, proofs and unit correctness
checks ran locally; no benchmark or relation-yield measurement is represented
as run.

## Replay and evidence

Run `bash verify.sh NEW_RECEIPTS_DIRECTORY` with Lean4.19.0 and the Rust
toolchain on PATH. Lake reuses two unchanged, pinned relative proof packages;
Cargo uses the locked native exporter dependencies. Offline Cargo replay
requires the dependencies in the local cache. The output directory must be
new or have no `replayed-fixture` child because export refuses overwrites.

- [Formalization and exact hypotheses](FORMALIZATION.md)
- [Readable derivation, cost caveat and source review](reports/REPORT.md)
- [Illustrated PDF](reports/report.pdf) and [editable diagram](reports/construction.svg)
- [Prospective protocol](PROTOCOL.json), [fixture metadata](fixture/fixture.json)
- [Final verification](verification/README.md), [machine record](verification/metadata.json)
- [Handoff](HANDOFF.md)
- [Remaining successful-work obligation](../../../problems/factor-base-shape-to-successful-work.md)

The accepted source uses no placeholders, native evaluation or custom axioms.
The unchanged transitive auditor checks every source module in the dependency
closure named by the replay script. Failed development builds are retained
separately; they are not imported into the proof project. This directory
extends PR#3108 without modifying the sealed earlier evidence.
