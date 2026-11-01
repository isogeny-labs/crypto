#set page(paper: "us-letter", flipped: true, margin: 0.5in)
#set document(date: datetime(year: 2026, month: 10, day: 31))
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
#set par(leading: 0.45em)
= Lean proof of the canonical top-ideal degree reduction
2026-10-31 · prime fields · two summands · formalization following PR #3077

The canonical highest-total-degree ideals of the displayed secant and dense
systems have first vanishing homogeneous degrees d+1 and 2d+1 for d≥2,
under the explicit field/coefficient/lower-degree hypotheses. The F17 fixture
is checked at 5→3 for all targets and all four signs. Each original equation's
actual total degree is proved; the statement does not assume which component
is highest.

#align(center)[#image("coverage.svg", width: 94%)]

Headline declarations: #link("../DegreeBridge.lean")[DegreeBridge.canonical_strict_reduction]
and #link("../CanonicalFixture.lean")[CanonicalFixture.degree_five_to_three].
Full proof map: #link("../FORMALIZATION.md")[FORMALIZATION.md].

#pagebreak()
= Exact scope and verification

*Invariant:* top(p) is the homogeneous component at p.totalDegree.
The actual ideal is the span of these tops. FirstVanishing J n states that
every degree-n homogeneous polynomial belongs to J and that, at each smaller
degree, some homogeneous polynomial does not. The quotient-map equivalence
is proved in Lean.

*Hypotheses:* K is a field, 2≠0, d≥2, and both graph leading coefficients
are nonzero. Graph lower terms have total degree below d; the dense monic
membership lower terms have total degree below 2d. Targets and coefficients
r,s,A,B are arbitrary for the degree statement. Exact group-relation meaning
uses the separate geometric hypotheses in the earlier written proof.

#table(columns: (1.25fr, 3fr), inset: 5pt, stroke: 0.5pt + luma(80%),
  [Canonical highest forms], [DegreeBridge proves the actual total degree of every generator and equality of canonical and computed top ideals.],
  [Higher degrees], [Mathlib's exact monomial-ideal support criterion connects the exponent lemmas to actual ideal membership and homogeneous quotient images.],
  [Quadratic case], [The ideal remains nonmonomial. All cubics vanish; a map into an iterated square-zero extension proves that mx survives in degree two.],
  [Concrete F17 base], [Exact split and full intersection, cardinality 4, nonsingularity, signed curve lifts, actual membership product and four-branch canonical degree 5→3 are checked.],
  [Exact secant algebra], [The cubic identity, residual-to-factorization implication and selected-point curve membership are checked over any commutative ring.],
  [Kernel checks], [Eleven modules build with warnings treated as failures. The unchanged transitive auditor checks 121 declarations, including generated equations:0 failures,0 native trust.],
  [Remaining formal scope], [Full elliptic-curve point addition/group-equivalence is written, not formalized. No Hilbert-series coefficient, F4-degree or runtime theorem is inferred.],
)

*Foundations and pins:* Lean 4.19.0; Mathlib v4.19.0,
commit `c44e0c8ee63ca166450922a373c7409c5d26b00b`. Dependencies are locked.
All transitive axioms are confined to propext, Classical.choice, Quot.sound.
There are no placeholders, custom axioms or native-evaluation exceptions.
The cache returned proxy CONNECT 403, so the pinned dependency was built from
source. No network-policy bypass was attempted. lean4checker was unavailable;
no fresh-checker result or passing cache-download gate is claimed.

*Reproduce:* #link("../README.md")[commands and archive],
#link("../verification/build.log")[build],
#link("../verification/axioms.log")[axiom audit],
#link("../verification/metadata.json")[source hashes and environment].
Failed development logs remain in a deterministic archive, separate from
the final successful receipts.

*Provenance:* field/curve parameters match
`icv1-fp5-tm1-a77cc6af`, candidate FB1hed5b19904cc8.
The Lean files check polynomial data, not storage-hash encoding or group order.
Sources: parent SECANT_GRAPH/THEORY;
#link("https://github.com/leanprover-community/mathlib4/tree/c44e0c8ee63ca166450922a373c7409c5d26b00b/Mathlib/RingTheory/MvPolynomial")[Mathlib ideal/homogeneous theory];
#link("https://github.com/leanprover-community/mathlib4/blob/c44e0c8ee63ca166450922a373c7409c5d26b00b/Mathlib/Algebra/TrivSqZeroExt.lean")[square-zero extension].
Literature-wide novelty remains open. This proof-only continuation adds no
cryptographic experiment, CAS rerun or cost/leaderboard claim.
