Prime-field bit-restriction local identities
Date: 2026-10-29

Scope
-----
PrimeBitLocal.lean adds eight symbolic dependency lemmas for the written
extensions/large_curves/prime_bit_restricted_upper.txt argument. The first
five hold over arbitrary commutative rings:

1. quadratic_linear_cancellation: the explicit quadratic/linear combination
   equals the quadratic's evaluation at alpha.
2. odd_s3_diagonal: S3(t,t,u) has the stated linear-in-u specialization.
3. diagonal_constant_relation: its constant term differs from the squared
   cubic derivative by the explicit cubic multiple.
4. odd_s3_at_cubic_root: cubic-root specialization of that identity.
5. derivative_ne_zero_at_cubic_root: a nonzero discriminant expression
   prevents the cubic and its derivative from vanishing together.

The last three have an explicit Field assumption:

6. odd_s3_diagonal_ne_zero: a cubic root with nonzero derivative produces
   a nonzero constant S3 specialization.
7. odd_s3_diagonal_ne_zero_of_discriminant: derives the same conclusion
   from the explicit nonzero discriminant expression.
8. normalized_quadratic_linear_refutation: multiplying the local
   cancellation by its explicitly nonzero evaluation's inverse yields one.

These are actual symbolic Lean proofs, not checks of recorded numerical
scores. They DO NOT prove the full last-fall upper bound 3*ell+2.
In particular they do not formalize V_(F,D), Boolean normalization degree
bounds, support-aware assignment localization, rational-lift/root splitting,
coprimality of specialized quadratics, auxiliary-bit surjectivity, lookup
semantics, or the final unit-ideal/last-fall bridge. The custom oddS3 formula
is not connected to a Mathlib elliptic-curve summation-polynomial object.

The older FallDegreeIdentities.lean, its twelve lemmas, original receipts,
source SHA-256 and four existing identity ProofRefs are unchanged. This
addition registers a second default Lake library target and writes its own
new verification artifacts only.

Checks and foundations
----------------------
The actual default Lake build compiled both libraries successfully (770 jobs).
Every new declaration prints its transitive axioms. The first five use
propext and Quot.sound. The three field consequences additionally use the
standard Classical.choice foundation. No sorry, custom axiom, native_decide,
or unchecked native evaluation is used.

Exact commands, the fresh-kernel result, source hash and remaining obligations
are recorded separately in prime-bit-verification.json, prime-bit-build.log,
prime-bit-fresh-check.log and prime-bit-source.sha256. Fresh replay uses Lean's
built-in kernel checker; it is not a separate external kernel implementation.

Pinned reproduction
-------------------
Lean: leanprover/lean4:v4.34.1, compiler commit
5045d0056413266e57c625dcd7c365b10e377c52.
Mathlib: d13f23b723b8a846827a245b89c10fc7d3f11612.
The transitive revisions remain pinned by lake-manifest.json.

With the pinned toolchain on PATH and dependencies installed:
  lake build
  lake env leanchecker --fresh --verbose PrimeBitLocal

The task restored the official pinned compiler after the workspace restart
removed /tmp's earlier installation, and reused the existing pinned Mathlib
source and build cache. The ignored cache links and downloaded toolchain are
development artifacts, not source files or proof shortcuts.

Literature and formula boundary
-------------------------------
The formulas and hypotheses follow the study's written proof and SOURCES.md:
Semaev, arXiv:1504.01175 Section 2, and Kosters--Yeo, arXiv:1503.08001
Section 2, as recorded in the parent task. The present module proves the
displayed polynomial dependencies directly. It makes no novelty claim.
