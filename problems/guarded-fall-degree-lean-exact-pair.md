# Guarded fall-degree exact-pair Lean formalization

- **Status:** open
- **Kind:** formalization implementation gap
- **Affected work:** [ordinary guarded three-abscissa proof](../research/guarded_fall_degree_20261028/proofs/universal_five_symmetric_audit.txt), [Lean proof map](../research/guarded_fall_degree_20261028/formal/PROOF_MAP.json)
- **Next action owner:** current fall-degree formalization task
- **Last updated:** 2026-10-29

## Problem

The requested complete Lean proof of the exact ordinary first/last fall pair
`(4,5)` is not implemented: the degree-five construction and the weighted
syzygy/Koszul formalization remain missing.

The original target is every unsatisfiable inverse-guarded three-abscissa
short-Weierstrass system over a prime field of characteristic greater than
three, with a nonsingular curve, rational affine factor-base lifts and target,
and both signs allowed. No claim is made for another encoding or factor-base
cardinality. A checked numerical DAG, or a theorem assuming the desired
degree-five refutation, would not satisfy this acceptance criterion.

## Evidence and uncertainty

- **Observed:** the new Lean modules define ordinary filtered closure by
  finite derivation trees, prove its least-closed-subspace property, formalize
  the three-point coefficient rigidity argument, and apply the complete
  separating-space argument to the actual six inputs.
- **Derived or proved:** `GuardedModel.no_degree_four_refutation` proves
  `1 ∉ V(F,4)` for every field with `6 != 0` and any three distinct base
  abscissas. `GuardedModel.lastFall_lower` proves the last-fall lower bound
  with an explicit unit-ideal hypothesis. Curve nonsingularity and point
  lifts are not needed for this lower bound.
- **Derived or proved:** `GuardedModel.pairDifference_derivable` gives an
  actual degree-four derivation of the quadratic divided difference.
  `FirstFallCore` proves low-degree top-map injectivity and the explicit
  homogeneous relation, without pretending to define the Koszul quotient.
- **Suspected explanation:** none; this is a named missing formalization,
  not evidence of a mathematical obstruction to the written proof.
- **Missing capability:** formal monic reduction with the intermediate
  degree caps, staged resultant extraction, finite-grid inversion, the
  rational-root/point bridge, and weighted syzygy/Koszul infrastructure.
- **What this evidence does not establish:** a complete Lean proof of
  `d_F <= 5`, the first-fall equality, the exact pair, or any transfer to
  growing bases or Boolean descent. Existing recorded Lean checks remain
  historical records and are not rewritten as checks of these new statements.

## What would unblock it

- [ ] Formalize degree-preserving monic normalization and each legal
  multiplication/reduction in the degree-five construction.
- [ ] Prove the resultant and finite-grid inversion steps in the same
  polynomial presentation, with no assumed refutation or assumed closure cap.
- [ ] Connect rational-point unsatisfiability to the algebraic condition used
  in the upper proof, including repeated roots and two-torsion cases.
- [ ] Define the weighted leading-form syzygy/Koszul first-fall convention
  and prove the exact first-fall statement for the six original generators.
- [ ] Combine the original hypotheses into one faithful exact-pair theorem,
  build it, audit its transitive axioms and fresh-kernel replay it.

These conditions together establish the requested formal result; completing
only the lower-bound proof does not discharge the original task.

## Next discriminating step

- **Action or proof obligation:** implement monic coefficient reduction as a
  filtered-closure derivation, proving the transient degree-five cap before
  composing it into the resultant construction.
- **Inputs and controls:** the six generators in `GuardedModel.inputs`, the
  same pinned Lean/Mathlib revisions, and the written proof above. Frozen
  fixture files are not substitutes for universal hypotheses.
- **Execution limits and stop condition:** development proof checking only;
  no scientific sweep, new curve fixtures, or performance claims.
- **Outcomes and next actions:** a checked reduction lemma unlocks staged
  normalization; a failed build remains a proof obligation, not evidence
  against the mathematical theorem.
- **Recheck trigger:** a new checked upper-bound dependency or complete
  weighted-syzygy definition.

## Resolution and history

Unresolved. On 2026-10-29 the full uniform degree-four obstruction was
formalized, replacing the earlier identity-only status for this part.
