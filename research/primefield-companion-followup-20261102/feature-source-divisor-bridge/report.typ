#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Bridging the actual source divisor
Native expression audit • T-1785 • 2026-11-07

The actual source uses moving multipliers of the query Q, rather than the fixed kernel roots used in the previous Miller construction. The new compiler follows its geometry recurrence and rewrites every coordinate difference as a signed quotient of division polynomials. This identifies the exact function that a replacement must reproduce.

#image("flow.svg", width: 100%)

== Positive construction
The compiler preserves signs, integer exponents and the chosen integer representatives of the multipliers. It adds the existing raw-division monomial, uses the previously derived seed cancellation and reduces exponents modulo the projection order. The resulting expression agrees with the old projected source in26 native software comparisons: all nonzero points at r7 and r13, and8 selected points at r479.

== Exact bridge constraint
For r7 the projected source has representative J=−ψ₉ψ₃²/ψ₄. On the F₅ fixture, J has order1 at a geometric point of exact order9. A product of functions supported on the order7 subgroup, times a rational fourth power and a constant, cannot have that divisor. Thus the fixed-subgroup Miller factors do not directly replace this lift. This does not exclude identities holding only on the finite subgroup, other supports or pullbacks. No extension-field computation is performed.

== Evidence and limitations
Three selected native methods pass;29 imported methods are filtered out. Thirteen source/fixture hashes are checked before and after. The first build's integer-type mismatch, source snapshot and errors are retained in validation-01; validation-02 passes after a fixture cast correction. Successful builds retain unused-code warnings. The reference shares the division engine, so PCC-20261107-16e37b is unsigned producer evidence, not independent verification. No benchmark, recovery or cloud experiment ran.

== Next construction
Investigate finite-subgroup reduction and regular interval products of division values, retaining the odd-weight sign. The240-term order479 expression is a reference representation, not a faster evaluator. Original cohort and production source remain unchanged.

== Sources
Actual source, projected residual and seed-cancellation derivations; original shared-projective-grid and normalized-partial-block code. Stange2026, section1, divisor and x-relation: https://link.springer.com/article/10.1007/s40993-026-00741-2. Sage division-polynomial documentation. KB unavailable. Complete formulas, domains, source paths, commands and failures are in DERIVATION.md and validation receipts.
