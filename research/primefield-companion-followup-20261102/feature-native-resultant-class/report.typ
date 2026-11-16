#set page(paper: "a4", margin: 18mm)
#set text(size: 11pt)
= Resultant-class composition
One terminal projection for the alternative geometry backend

#image("flow.svg", width: 100%)

== Construction
The Euclidean chain returns a typed multiplicative quotient representative. Compose the main and optional tail representatives. For order four, include the square of the existing square-class accumulator: its projection equals the quadratic-character correction. Project once, then apply the outer geometry sign.

== Exact checks
Ten selected native methods pass. The new quotient test checks 260165 field-element pairs, kernel-equivalent representatives, squaring, negation and mixed-field rejection. The 2500 geometry cases retain 1349 valid agreements and 1151 common rejections; test instrumentation confirms exactly one projection for every valid case and zero for rejected cases. Earlier determinant, root-product and paired-source checks remain passing.

== Local cost change
The previous Euclidean geometry uses one main projection, an optional tail projection and an optional order-four character exponentiation. The composed path uses one projection, plus a representative square for the order-four correction. This is exact local call accounting, not a timing or total-cost comparison.

== Boundaries and next work
Private representatives are not full field resultants; no additive operations are exposed. Prime modulus remains caller-certified. Polynomial Euclidean divisions and geometry point lists remain. Next: half-GCD transition batching with exact degree-drop/sign bookkeeping, implicit orbit-product evaluation and compact nonlinear decoder construction.

Sources and proof: DERIVATION.md; prior resultant code and FLINT documentation. KB unavailable. Unsigned producer checks; no cloud or recovery comparison.
