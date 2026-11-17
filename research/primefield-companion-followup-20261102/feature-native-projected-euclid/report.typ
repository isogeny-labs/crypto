#set page(paper: "a4", margin: 18mm)
#set text(size: 11pt)
= Projected Euclidean resultant
Prime-field alternative backend; order-two/order-four image

#image("flow.svg", width: 100%)

== Construction
Accumulate resultant reciprocity factors along the Euclidean remainder chain. Reduce each leading-coefficient exponent modulo the projection order; keep the sign separately. Perform one final projection per resultant. Build root polynomials recursively without retaining product-tree nodes. Polynomial arithmetic stays in the base field.

== Exact development checks
1080 polynomial pairs agree with projected Sylvester determinants, including 69 zero determinants. Another 245 root-product cases agree with direct products and the prior evaluator. Nine selected native methods pass, including existing paired-source and 2500 synthetic geometry cases. Generator replay is byte-identical. Receipt, raw logs and transitive source hashes are retained. This is producer validation.

== Costs that remain
Ordinary Euclidean division can require many inversions and reductions; no half-GCD is implemented. A main and tail resultant each receive a projection. No timing or memory advantage is measured. The geometry still explicitly visits at least ceil(sqrt(r+1))−1 entries. Current decoder compilation enumerates the scalar field and forms dense systems.

== Next constructions
Batch Euclidean transitions with correct degree-drop and sign accounting; combine representatives before final projection. Seek an implicit orbit product with a cheap evaluator, and a compact nonlinear decoder constructor. Prior backends remain intact for comparison.

Sources: DERIVATION.md and FLINT polynomial GCD/resultant documentation. KB unavailable. No cloud job or recovery comparison; original cohort unchanged.
