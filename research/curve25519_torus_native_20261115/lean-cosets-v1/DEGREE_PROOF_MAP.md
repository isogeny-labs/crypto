# Dense polynomial degree: instance gate

Structure above: Mathlib `Polynomial F` for a field F, finite index types
and the existing conditional injection. Apply
`Polynomial.eq_zero_of_natDegree_lt_card_of_eval_eq_zero` verbatim.
Its contrapositive is the endpoint; no root-count law is re-proved here.

Object: any nonzero polynomial evaluating to zero at all encoded values of
`Fin20633 × Fin213156431` for an h of the stated exact order.
Assembly: injection is `Curve25519CosetInstance.indexed_injective`; the
index cardinality is the existing Fin/product instance, numerically
4398056640823. Compare natural degree with that cardinality, apply the
existing zero-polynomial conclusion, and contradict the nonzero hypothesis.

| Statement | Kind | Owner | Inheritance |
|---|---|---|---|
| membership_polynomial_degree | Numeric endpoint | Study | Mathlib root-count law + existing injection |

Missing application components: exact actual-field instance, an explicit
polynomial or circuit interface, and any algorithmic cost lower bound.
The endpoint constrains degree of a single univariate polynomial vanishing
on the raw coordinates. It says nothing about sparse support size,
arithmetic-circuit size, quotient-coordinate degree, elimination cost,
multivariate solvers, or the total attack exponent. Dense coefficient
storage is an explicitly selected representation, not forced on all solvers.

The ten-endpoint source/configuration already pinned by ac25ef retains its
bytes. This new module imports it; no new axiom, limit override, generic law
or composite opening is needed. Sources checked (KB unavailable): Mathlib
Algebra/Polynomial/Roots.lean at the existing pinned Mathlib revision.
