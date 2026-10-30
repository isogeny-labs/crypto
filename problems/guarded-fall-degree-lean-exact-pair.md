# Guarded fall-degree exact-pair Lean formalization

- **Status:** resolved for the original prime-field guarded three-abscissa model
- **Kind:** formalization implementation gap, resolved
- **Affected work:** [ordinary guarded proof](../research/guarded_fall_degree_20261028/proofs/universal_five_symmetric_audit.txt), [complete Lean proof](../research/guarded_fall_degree_20261028/formal/complete-exact-pair.md), [current proof map](../research/guarded_fall_degree_20261028/formal/PROOF_MAP.json)
- **Next action owner:** none for the mathematical gap; publication and reproducible checks follow the PR workflow
- **Last updated:** 2026-10-29

## Problem

The original blocked goal was the complete Lean proof of the exact ordinary
first/last pair `(4,5)` for every unsatisfiable inverse-guarded three-abscissa
short-Weierstrass system over a prime field of characteristic greater than
three, with a nonsingular curve, rational affine factor-base lifts and target,
and both signs allowed. It is now proved by
`GuardedExactPair.exact_pair_prime`, including attainment and minimality.

The generator set and degree conventions are preserved. Neither a numerical
DAG nor an assumption asserting the desired degree-five refutation is used
as a substitute for the universal conclusion.

## Evidence and uncertainty

- **Observed:** the default pinned Lake build checks the final theorem and all
  dependencies. Exact source hashes, build/audit logs, fresh replay outcomes
  and library validation are bound by the [versioned full-proof receipt](../research/guarded_fall_degree_20261028/formal/complete-exact-pair-validation/verification.json).
- **Derived or proved:** `CubicNormalization.monic_normalization` performs
  actual degree-five closure derivations through explicit cubic rotations
  and a constructed interpolation inverse. `GuardedReduction` derives the
  required families from the six original inputs in degree four.
- **Derived or proved:** `StagedResultant.resultGrid_derivable` extracts the
  scalar resultant using degree-checked affine-grid shifts;
  `GridClosure.grid_refutation` derives one using explicit nine-point
  interpolation and scalar-grid shifts.
- **Derived or proved:** rational base-point lifts split the first specialized
  quadratic at explicit field roots, including repeated roots/two-torsion.
  Polynomial UNSAT makes both second-quadratic values nonzero. The scalar
  product identity discharges the nonzero-resultant condition. The resulting
  derivation gives the unit ideal; it is not an extra theorem premise.
- **Derived or proved:** `GuardedTops` proves actual degrees and top components.
  `WeightedFirstFall` defines the graded top map and classical degree-specific
  Koszul combinations, proves minimal first fall four, and verifies a nonzero
  lift of degree at most three. The existing uniform degree-four obstruction
  supplies exact last fall five in `GuardedLastFall.exact_last_five`.
- **Suspected explanation:** none; the implementation gap is closed by the
  actual checked source chain.
- **Missing capability:** none for this exact-pair theorem. A general Mathlib
  resultant API bridge, algebraic-closure converse and abstract quotient
  isomorphism are unnecessary for the direct proof route.
- **What this evidence does not establish:** growing-base, characteristic-two
  or Boolean/Weil-descent formalizations; E12 closure certification; literal
  standard-curve fixture primality/provenance/UNSAT verification; group-law
  equivalence between polynomial solutions and signed point decompositions.
  The exact theorem's UNSAT premise is explicitly polynomial `NoSolution`.

## What unblocked it

- [x] Explicit degree-preserving cubic rotation and staged monic normalization,
  with every intermediate multiplication capped at five.
- [x] Staged scalar extraction and explicit finite-grid inverse, with no assumed
  refutation or assumed closure cap.
- [x] Direct rational-root and polynomial-UNSAT/unit-ideal bridge, including
  repeated roots and two-torsion. No full group-law formalization is needed
  for the stated polynomial UNSAT hypothesis.
- [x] Actual weighted leading forms and graded top-syzygy/Koszul first-fall
  definition, with exact first fall and a genuine lower-degree lift.
- [x] One faithful original-hypothesis prime-field theorem with `p>3`; build,
  transitive axiom audit and versioned fresh-kernel verification records.

The final statement has the original curve and rational-target hypotheses.
A stronger general field theorem also exists; its weaker requirements do not
change the original wrapper or silently replace its acceptance criterion.

## Reproduction and recheck

- **Action:** run the default pinned Lake build, repository axiom auditor and
  the per-module fresh replays documented in the full report.
- **Inputs:** exactly `GuardedModel.inputs`, with indexed-family equality
  proved by `GuardedExactPair.generator_range`; same original degree model.
- **Limits:** proof/development verification only; no scientific sweep or
  performance claim is introduced by this resolution.
- **Recheck trigger:** a change to theorem hypotheses, generator presentation,
  source hashes, pinned dependencies or proof-checking tools.

## Resolution and history

- 2026-10-29, earlier update: the complete uniform degree-four obstruction was
  formalized; the upper and first-fall bridges still remained open. The dated
  partial reports and receipts are preserved.
- 2026-10-29, full update: `GuardedExactPair.exact_pair_prime` proves the original
  exact pair. The source proof map identifies each discharged obligation and
  the alternative direct constructions that remove unnecessary library bridges.
  Other polynomial presentations remain separate open work.
