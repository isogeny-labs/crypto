# Round04 instance gate and proof boundary

1. **Structure above.** Mathlib `QuadraticAlgebra (Polynomial ℚ) (-1) (-1)`
   and its commutative ring instance, representing Q[W,omega] with
   omega²+omega+1=0; existing `Ideal.span`, `Ideal.Quotient.mk` and
   `Polynomial (ZMod 7/19/79)` structures. No new ring, field or group law.
2. **Theorems used as stated.** `QuadraticAlgebra.ext`, `re_mul`, `im_mul`,
   `re_add`, `im_add`, and inherited ring normalization certify the literal
   two-component polynomial identities. `Ideal.mem_span_singleton` and
   `Ideal.Quotient.eq_zero_iff_mem` turn divisibility into zero in the
   quotient. `Polynomial.irreducible_of_degree_le_three_of_not_isRoot`
   certifies the concrete cubic at7. No generic transfer law is restated.
3. **Objects as instances.** The native u3 kernel C(W), the two nontrivial
   beta polynomials actually emitted by the recognizer, and native exact
   quotient witnesses live in the existing quadratic polynomial algebra.
   A=W, beta2, beta4 are output certificates, never recognizer inputs.
   The source and target residue polynomials use the actual coefficients
   and roots recorded by the native model. No abstract evaluator is assumed.
4. **Assembly probe.** First type-check existing quadratic-ring projection
   laws and the ideal-quotient endpoint from a divisibility contract; also
   check these laws at the literal W/coefficient algebra and the concrete
   residue polynomial. Only then add concrete root identities and endpoints.
5. **Missing components only.** The CM endomorphism interpretation of C,
   kernel degree/Galois/Weber field bridge, irreducible-reduction implication
   over K, oriented specialization at the two primes, native LLL soundness
   and coverage, target Smart solver and CM-unit scalar recoverability are
   written/native arguments, not formalized by these polynomial endpoints.
   No uniform height/precision bound or arbitrary-curve evaluator exists
   here. Generic CM/specialization interfaces would belong in crypto-lean
   before a study consumer; they are not replaced by assumed conclusions.
6. **Inheritance.** See the operation table. Every operation is inherited
   from Mathlib. No heartbeat/recursion increase, custom axiom or native_decide.

| Operation | Inherited line |
| --- | --- |
| Rational coefficient polynomial ring | `Polynomial ℚ` commutative ring instance |
| Quadratic coefficients and multiplication | `QuadraticAlgebra` pair and commutative ring instance |
| Projection of products/sums | `QuadraticAlgebra.re_mul`, `im_mul`, `re_add`, `im_add` |
| Literal coefficient equality | kernel-checked `ring`/`norm_num` normalization |
| Polynomial equality via rational evaluation | `Polynomial.funext`, inherited evaluation ring laws |
| Divisibility from a displayed quotient | existing `dvd` witness constructor |
| Membership in the principal ideal | `Ideal.mem_span_singleton` |
| Vanishing after quotient | `Ideal.Quotient.eq_zero_iff_mem` |
| Concrete cubic irreducibility at7 | `Polynomial.irreducible_of_degree_le_three_of_not_isRoot` |
| Source/target factorization | inherited finite coefficient rings and `ring` |

The written algebraic chain is: C(beta)=C(W)*q(W) by the literal
two-component identity; therefore C(W) divides C(beta) by the displayed q;
therefore C(beta) belongs to (C(W)) by `mem_span_singleton`; therefore its
image in the quotient is zero by `eq_zero_iff_mem`. The first equality is
an instance certificate. The later arrows apply existing laws as stated.
No quotient-field property is inferred from these endpoints alone.

| Planned statement | Kind / owning module | Exact scope |
| --- | --- | --- |
| `beta_two_identity`, `beta_four_identity` | instance / WeberRootBoundary | exact polynomial multiples of C(W) |
| `beta_two_cleared_identity`, `beta_four_cleared_identity` | instance / WeberRootBoundary | denominator-cleared component identities for IDC1 |
| `beta_two_root`, `beta_four_root` | endpoint / WeberRootBoundary | their C-values vanish in the principal-ideal quotient |
| `auxiliary_irreducible` | instance / WeberRootBoundary | W³+3W+5 irreducible over F7 |
| `source_factorization`, `target_factorization` | instance / WeberRootBoundary | literal F79/F19 kernel factorization |
| Source-selected global class / scalar decoding | written/native only | no complete Lean transfer theorem |

The named literal definitions may be opened once in each finite equality
certificate because no prior theorem contains these new coefficient values.
All structural operations continue to use the existing projection laws.
This is an instance calculation, not unfolding an elliptic or CM interface.
