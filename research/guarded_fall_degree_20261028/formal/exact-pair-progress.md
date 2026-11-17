# Exact-pair formalization progress — 2026-10-29

The requested result is the exact ordinary first/last pair `(4,5)` for every
unsatisfiable guarded three-abscissa system in the study's prime-field class.
**The complete exact-pair theorem remains open.** This update proves the
uniform degree-four obstruction in Lean and constructs the first actual
filtered-closure consequence used by the upper/first-fall arguments.

## Verified mathematical result

`GuardedModel.inputs α β a b c r` is exactly the six original generators in
`MvPolynomial (Fin 5) K`, with variable order **u,x,y,z,v**:

```text
g(x), g(y), g(z), S3(x,y,u), S3(u,z,r), v*(x-y)-1,
g(T)=(T-a)(T-b)(T-c).
```

For **every field K with 6 ≠ 0**, every distinct `a,b,c`, and arbitrary
`α,β,r`, `GuardedModel.no_degree_four_refutation` proves

```text
¬ FilteredClosure.Derivable (GuardedModel.inputs α β a b c r) 4 1.
```

This is a universal statement about the actual polynomial system, without
rank data or fixture assumptions. It needs neither nonsingularity nor point
lifts. `no_refutation_below_five` covers every smaller degree. With the
explicit hypothesis `1 ∈ Ideal.span F`, `lastFall_lower` excludes every
last-fall bound below five. The unit-ideal hypothesis is exposed: the
rational-unsatisfiability-to-unit-ideal bridge is not formalized here.

`FilteredClosure.Derivable` has zero, eligible-input, addition, scalar and
arbitrary polynomial multiplication rules. Multiplication tests the actual
product's ordinary total degree and permits cancellation/degree falls.
`space` is a coefficient submodule; `least` proves its universal property.
`lastFallBound_iff_refutation` proves the equivalence with the ordinary
last-fall bound for a unit ideal, including high-degree consequences.

The separating space is the full `I + K*f1 + K*f2`, where `I` vanishes on
the guarded base grid with **u left formal**. The coefficient of u² of a
degree-at-most-three polynomial is proved affine. Three-point rigidity
forces both quartic weights to vanish, so the low-degree part lies in I.
Degree additivity handles every permissible nonconstant multiplier, scalar
multipliers are handled by submodule closure, and the zero branch is
explicit. Evaluation at a distinct base pair proves the space excludes one.

The module also proves `pairDifference_derivable`: the actual quadratic
divided difference is available in degree four. Each input/multiplier degree
bound is proved on multivariate polynomials, then composed using the closure
rules. `FirstFallCore` checks top-map injectivity below degree four, the
displayed degree-four relation and its lift. Those facts are **not** labeled
a complete first-fall theorem: the weighted syzygy/Koszul definition remains
missing.

## Acceptance status

| Requested component | Status | Lean evidence or remaining obligation |
|---|---|---|
| Ordinary filtered closure and last-fall meaning | Verified | `FilteredClosure.space`, `least`, `lastFallBound_iff_refutation` |
| Universal degree-four obstruction for the six inputs | Verified | `GuardedModel.no_degree_four_refutation` |
| First-fall relation and low-degree injectivity | Verified dependencies | `pairDifference_derivable`, `FirstFallCore`; graded/Koszul bridge missing |
| Universal degree-five upper bound | Not implemented | monic normalization, staged resultant, scalar grid inversion, rational-point bridge |
| Exact first/last pair `(4,5)` | Partial | requires both preceding missing bridges; no theorem assumes these conclusions to claim completion |

See the [named problem](../../../problems/guarded-fall-degree-lean-exact-pair.md)
for the precise remaining acceptance conditions. No theorem about growing
bases, characteristic two, or Boolean descent is added by this update.

## Reproduction and evidence

Use the existing pinned Lean 4.34.1 and Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`:

```sh
lake exe cache get Mathlib.Algebra.MvPolynomial.NoZeroDivisors Mathlib.Algebra.MvPolynomial.Equiv Mathlib.Data.Finsupp.Weight Mathlib.RingTheory.Ideal.Span Mathlib.Tactic.LinearCombination
lake build
lake env lean --run ../../../tools/lean/AuditAxioms.lean -- FilteredClosure ThreePointRigidity AffineCoefficient GridLower GuardedModel FirstFallCore
lake env leanchecker --fresh --verbose FirstFallCore
```

All paths above are relative to this formal directory. The development
receipt, exact source hashes, build, axiom audit and fresh replay are retained
under `exact-pair-validation/`. The old source hashes, verification receipts
and numerical evidence remain unchanged. No new scientific experiment or
performance measurement was run.

## Sources and diagrams

Read before implementation: the study's `SOURCES.md`, both independent
upper/lower audits and `universal_five_symmetric_audit.txt`; the pinned Mathlib
sources for total degree, `finSuccEquiv` coefficient bounds and ideal spans;
Huang--Kosters--Yang--Yeo Definitions 2.1–2.2 (the previously checked primary
definition) and the study's first-fall convention. The actual Mathlib
definitions, rather than scalar scoring rules, are used in the proofs.

The [dependency diagram](exact-pair-coverage.svg) and [PDF report](exact-pair-report.pdf)
show checked and missing branches. Historical study diagrams and report PDFs
are preserved: their partial-formalization warning remains true; this dated
formal report records the changed coverage. The leaderboard, progress
timeline, curve diagrams and performance graphs are unchanged because no
measurement or curve metadata changes.

## Development errors

Initial cache requests named two nonexistent Mathlib modules (`Ideal.Span.Basic`
and `Mathlib.Tactic.Omega`). The correct imports are `Ideal.Span` and Lean's
Omega tactic. Initial proof attempts exposed scalar-action type mismatches,
numeral/successor rewrite mismatches and over-broad simplification of affine
coefficients; these were corrected in the checked source. Failed elaborations
are not included in the verified theorem count. Existing source and numerical
evidence were not edited to conceal them.
