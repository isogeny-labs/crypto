# Full permutation symmetry of the actual canonical Semaev polynomials

The Lean modules prove full input-permutation invariance of the existing
canonical recursive polynomial at every arity≥3. The actual polynomial,
coordinate action and coefficient-field scalars are identified explicitly.
The final whole gate passes90declarations/0failures/0trusted. Exact local replay of **PCC-20261113-cda3d2** passes at2026-11-13T18:41:11Z; signed standing
remains unverified. See [verification receipt](verification.json). [pcc:unverified]

For every field K, actual Weierstrass curve W and stage n, let
P=recursivePolynomial W n, with arity n+3. The closed certificate proves
the following, with no characteristic or nonsingularity restriction for
this algebraic polynomial identity. [pcc:unverified]

- Its actual multivariate image `canonicalMv W n` is `IsSymmetric`.
- For every permutation σ of all n+3 inputs, `permuteCanonical K n σ P=P`.
- The same action sends every explicit input coordinate i to coordinate σ(i).
- It fixes every actual coefficient-field scalar `C(coefficientEmbedding n a)`.
- The outermost coordinate0 is the existing polynomial variable X.

These are checked equalities through explicit ring equivalences from the
original `InputRing` polynomial. The tower object is defined as the image
of the existing polynomial; its recurrence and degree are proved from
the existing source and resultant naturality. [pcc:unverified]

The induction proves every adjacent swap. The S3 base uses its literal
formula; the S4 base uses actual-curve cross-pair symmetry. The newest
three inputs use the proved source-ordered S4-block normalization, and
older swaps inherit through actual coefficient/resultant maps. A verified
multivariate/tower equivalence identifies each adjacent action with its
actual variable renaming. Mathlib's finite adjacent-generator theorem
then proves all permutations; the result is pulled back to the original
canonical polynomial. [pcc:unverified]

The inherited rational b=(0,0,0,1) S4 control remains a direct disproof of
the unrestricted independent-b extension. The parent packet proves those
coefficients cannot come from a rational Weierstrass curve. It does not
refute the qualified source. This packet confirms the actual-curve
symmetry claim; higher absolute irreducibility and general splits k≥3
remain open. [pcc:unverified]

Reproduce in `lean/` with `lake build`, or from repository root with
`tools/lean/proof-gate.sh research/semaev_symmetry_audit_20261113/lean`.
Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
No holes, custom axioms, native evaluation, raised limits, global
elaboration flags or waivers are introduced. `Tower` is an explicit
transparent type abbreviation for the actual iterated polynomial rings.
[pcc:unverified]

Sources reviewed: Semaev2004 Theorem1/p2 symmetry; Kosters–Yeo2015
Section2/p3 general S3/recursion; existing actual canonical/transport
source and the immutable source-order normalization packet; Mathlib
finite permutation generation and multivariate rename/equivalence APIs.
KB unavailable; bounded primary/repository/library fallback, no novelty
claim. No new native experiment, paid compute or agent storage upload.
The [original fourteen-item audit](../../problems/fourteen-target-audit-proof-and-execution-gaps.md)
retains its remaining requirements. No corpus or leaderboard status is
promoted; existing performance graphs are unchanged because no timing
experiment is included. [pcc:unverified]
