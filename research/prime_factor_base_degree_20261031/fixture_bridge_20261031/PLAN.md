# Concrete F17 group/degree bridge

User request: identify and pursue the next goal after the generic group-law
formalization. Goal: one faithful Lean statement connecting the concrete F17
split graph/base, zeroes of the actual polynomial generators, actual curve
addition, all four sign branches and canonical homogeneous degree 5 to 3.

## Frozen proof obligations

1. Rebuild the exact published generic group-law source and run the unchanged
   whole-module transitive-axiom auditor. Check old sealed source hashes first.
2. Prove the concrete graph polynomial has the exact CompleteSplit identity
   and the concrete short curve has nonzero Mathlib discriminant.
3. Prove evaluation of each original polynomial generator is its named scalar
   residual; use these exact generators in both the root and top-ideal sides.
4. Prove raw polynomial roots iff actual signed factor-base relations for
   every nonsingular finite target, including tangents and repeated abscissae.
5. Bundle this with the earlier canonical first-vanishing degree theorem,
   keeping the exact invariant and all sign branches. Prove identity-target
   coverage separately rather than forcing infinity into finite coordinates.
6. Build and audit every new module with no placeholders, custom axioms or
   native evaluation. Retain exact commands, dependency pins, source hashes,
   failed development records and successful receipts.
7. Supply an illustrated source-linked report with an inspected PDF, update
   the parent README, and publish a focused regular PR.

## Sources read before implementation

- SecantFixture.lean, CanonicalFixture.lean, DegreeBridge.lean and
  SecantTopForms.lean in the sealed ../proof project: concrete graph/base,
  exact generator definitions and canonical top ideals.
- ../group_law_20261031/FORMALIZATION.md and actual Lean sources: exact group
  semantics and CompleteSplit assumptions.
- Pinned Mathlib MvPolynomial/Eval.lean, Polynomial/Eval/Defs.lean,
  EllipticCurve/Affine.lean and Group.lean: evaluation homomorphisms,
  nonsingularity and the actual point group.
- Angdinata and Xu, ITP 2023, DOI 10.4230/LIPIcs.ITP.2023.6, the primary
  group-law formalization reused here; local full text was reread.

This is proof-only continuation, not a new factor-base experiment, degree
measurement or cost benchmark. No numerical dashboard row is promoted.
The earlier homogeneous first-vanishing degree must not be relabeled affine
F4 solving degree. No claim of literature-wide novelty is made.

## Environment and custody

Fresh-main worktree at 2741765c9 (descendant of merged #3095). The old shared
worktree is preserved. All sixteen prior group-project SHA256SUMS entries
pass. Conductor check returned 127; the documented absent-installation
fail-open applies. A network-free git lazy-fetch attempt failed; explicit
network permission recovered the fresh checkout. Initial fetch updated main
but returned 1 on an unrelated uninitialized submodule; no submodule content
is required or changed.

The first dependency-layout attempt may require both prior Lake projects;
preserve any duplicate-module/provider failure before choosing a verified
source-reuse layout. No prior sealed proof directory will be rewritten.
