#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Two-center power-folded sign recovery
T-1804 • 2026-11-07 • unsigned producer evidence

The new destination state tracks z=(k+c)^m, where m divides r−1. A product of m public shifted source signs yields a character constraint on z. Two centers can resolve the remaining scalar fiber by a polynomial gcd. All arithmetic remains over the original prime source and scalar fields.

#image("flow.svg", width: 100%)

== Construction and proof boundary
For an exact-order-m root of unity ζ, the product of y−bζ^j is y^m−b^m. Multiplicativity of the quadratic character gives the folded sign. The power image is zero together with the d-th roots of unity, d=(r−1)/m; its initial polynomial is Z(Z^d−1). Each update intersects this root set with the observed sign constraint using the existing half-GCD backend.

For singleton images z0,z1, intersect (X+c0)^m−z0 and (X+c1)^m−z1. A linear gcd gives a candidate, checked against Q. For m=2 and distinct centers, subtraction gives a nonzero linear equation, so consistent images determine k. Higher m can leave ambiguous fibers. The code returns unresolved rather than enumerating these fibers.

== Recorded native checks
Validation-01: two selected methods passed,157 imported methods filtered. Across primes7,13,17,31,41,97, every divisor m from2 through8 and every scalar gave5,382 orbit identities and5,382 complete candidate-set checks.897 joint fibers matched direct simultaneous power evaluation;202 higher-power fibers were ambiguous. All quadratic fibers were linear.

The public-point adapter recovered33 inputs: all13 on E/F7, y²=x³+3, P=(1,2), r13,m3; and the frozen20 on E/F509, y²=x³+2x+10, P=(3,68), r479,m2.20 used the fiber gcd;13 used a zero-source or identity exit.725 total source calls include input validation. Cap, invalid-point, contradictory-sign, invalid-order and exhausted-query controls passed. These are software checks, with no timing panel; the original4,563-case cohort is unchanged.

== Costs and next construction
Initial image degree is d+1, with two such states and a degree-m extraction. Conservative polynomial workspace is O(d log(d+1)+m log(m+1)), plus source workspace. Each folded update still costs m source evaluations and O(m log r) group additions in this implementation; source calibration is charged separately. Modular powering and half-GCD costs, including exponentiation-based inversions, remain explicit in DERIVATION.md.

Only shifts b^m are available. No separation probability or logarithmic query bound is assumed. Favorable d+m requires a suitable divisor of r−1. On r479,m2 the initial image degree240 exceeds the earlier frozen-panel three-sign degree of at most60; this fixture checks composition, not a gain. On r97,m8 the image degree is13. These are dimensions, not measured runtimes.

Next: evaluate the orbit product in the source more cheaply than m separate geometry queries and resolve restricted-shift/fiber separation. This module establishes the transport and bounded extractor interface needed to test that construction.

== Evidence, formalization and sources
Thirty source/input pins and complete command receipts are retained. ProductMap.lean addresses only the multiplicative-map finite-product bridge; finite-field factorization, gcd extraction and source correctness are not formalized here. The available formal project lacks its built Mathlib dependency. See the proof map and Lean output before interpreting this partial lemma.

Sources: local quotient-indicator, lifted-moment and half-GCD notes; NIST DLMF27.9; Cheon–Kim2016, doi:10.1112/S1461157015000303, introduction and section2. Their auxiliary powers and collision method are not used. KB unavailable. No independent signed verdict is supplied by this producer.
