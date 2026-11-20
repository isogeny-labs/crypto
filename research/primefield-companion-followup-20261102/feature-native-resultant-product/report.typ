#set page(paper: "a4", margin: 18mm)
#set text(size: 11pt)
= Scalar resultant accumulation
Prime-field projected-source development check

#image("flow.svg", width: 100%)

== Construction
Evaluate the source polynomial down the existing full-field remainder tree. Each leaf returns one Horner value. Each internal node returns the product of its children. The root returns the same product of source–destination differences used by the projected residual.

== Exact structural accounting
For n destination leaves: n−1 child-combination multiplications, compared with n multiplications in the old fold initialized at one. No returned evaluation vector or vector concatenation. One saved scalar per recursion level. Polynomial trees, remainder vectors and division temporaries remain. These are derived counts, not timing or peak-memory measurements.

== Checks and evidence
See receipt.json and product.log for the native checks. Direct difference products and the preceding vector evaluator are the two arithmetic references. Additional checks cover repeated and noncanonical points, constant/zero/empty polynomials, empty arrays, foreign domains, and the paired projected-source fixtures. Both paths share some polynomial arithmetic; this is producer validation.

== Scope and continuation
Projection remains after additive field work. The full-value source reference is retained. Next: account for the complete source and decoder costs and investigate whether root structure can reduce the polynomial work itself. No cloud or recovery comparison ran.

Sources: DERIVATION.md; FLINT remainder-tree evaluation documentation. KB unavailable. Historical inputs and original cohort preserved.
