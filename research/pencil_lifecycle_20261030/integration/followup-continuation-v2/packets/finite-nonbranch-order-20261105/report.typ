#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Finite nonbranch local multiplicity
T-1613 · 2026-11-05 · [pcc:unverified] · Prospective source, not kernel-checked

== Derived construction
Work in the actual affine curve ring A = K[x,y]/(y² − F(x)), with separable quintic F and selected rational point P = (x0,y0). Assume 2, b and y0 are nonzero and n(x0) = b y0. The conjugate point P′ = (x0,−y0) also lies on the curve. If n(x) − b y were the zero element of A, evaluation at P′ would give 2 b y0 = 0, a contradiction. The function vanishes at P but is not the zero function.

The existing separable-domain theorem makes localization into the point stalk injective. Its actual projective-stalk isomorphism preserves nonvanishing. Mathlib addVal_eq_top_iff therefore excludes infinity for this germ's valuation. Reusing the existing norm-order equality forces all three support valuations finite. ENat.toNat_add then yields their natural-number sum without discarding infinity.

#image("steps.svg", width: 100%)

== Source artifact and checks still needed
FiniteNonbranchOrder.lean.txt proposes five declarations, including a specialization that derives the norm equation from source coefficients and Mumford divisibilities. The existing norm proof is reused. No compiler or axiom readback has run; exact elaboration remains untested. A distinct author must review source fidelity before acceptance.

Control obligations: b = 0 and n = 0 permit the zero function. At y0 = 0 or in characteristic two the conjugate-point argument is inconclusive. Localization injectivity requires the actual domain. ENat.toNat(top) = 0, so finite orders must be established first. These method limitations are not all counterexamples to stronger geometry.

== Remaining global construction
One nonzero rational function must be identified on the same integral projective curve, with compatible maps to every codimension-one local field. Include all closed points and residue degrees, repeated supports, branch orders, infinity pole orders and finite support. Then identify its principal divisor and concrete class quotient with the source output. The present rational nonbranch result does not supply those global steps.

== Sources
DERIVATION.md records the argument and stronger degree-based alternatives; sources.json pins actual local Lean sources. Primary references: Stacks Project 00PD (DVRs and nonzero uniformizer factorization), https://stacks.math.columbia.edu/tag/00PD; 02RJ (orders of nonzero rational functions), https://stacks.math.columbia.edu/tag/02RJ. The local APIs are addVal_eq_top_iff, ENat.toNat_add, actual point evaluation and pointRingEquiv. No new measured cost or cohort result is reported.
