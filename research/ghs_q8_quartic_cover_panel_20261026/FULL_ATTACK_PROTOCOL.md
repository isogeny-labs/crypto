# Fixed order-554 transfer attack trial

Frozen 26 October 2026 after the quotient and trace certificates, before
relation collection. Source is `E_105: v²+uv=u³+u²+b` over the Conway
`GF(512)`, with `b` encoded as 105, subgroup order `r=277`,
`G=(w^478,w^433)`, `Q=(w^77,w^119)`. The independent Magma certificate
gives separate degree-two quotient fibers for `G`, `Q`, and the identity.
The Sage trace certificate checks that their traced pullback classes
`TG,TQ` are nonzero and `TQ=7TG`.

Build the smooth quartic over `GF(512)` from the frozen 15 coefficients.
Enumerate its nine `GF(8)` points and take the eight nonbase point classes,
multiplied by the cofactor 2, as a factor base in the order-277 subgroup.
Use a deterministic random walk with seed `20261026`: at each step add one
factor-base class. Retain coefficient vectors for repeated classes as
relations. Stop when both transferred inputs have decompositions and
the relation matrix has one-dimensional right kernel, or after 2,000
steps. Solve for the ratio of the two decomposition evaluations in
`GF(277)`, and verify the answer on the original source pair.

Record separate times for field/curve/Jacobian setup, target map,
relation collection, linear algebra, and source verification. Count
Jacobian additions in the walk. Preserve failed and bounded outcomes;
do not infer an attack cost from a partial stage. Compare measured
one-target wall time against the independently retained 32-trial rho
baseline on the same `(E_105,G,Q)`.
