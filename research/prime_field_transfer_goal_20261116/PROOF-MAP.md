# Round-one Lean instance gate

This formalizes one concrete incompatibility of proposed component labels.
It does not formalize the Neron model, a universal model constructor or an
arbitrary-point efficient transfer.

1. **Structure above:** Mathlib `WeierstrassCurve.Affine.Point` additive
   group and `AddMonoidHom` into `ZMod 5`.
2. **Theorem used as stated:** `map_neg`. On any additive group homomorphism,
   the value at −P is the negative of its value at P. No proof of this law is
   copied. `addOrderOf_eq_prime` reads the exact order from the point chain.
3. **Object as instance:** the literal nonsingular curve
   `y²+xy=x³−64x+288` over `ZMod 59`, P=(11,28), Q=(11,20), with requested
   labels 1 and 3 in `ZMod 5`.
4. **Assembly probe:** with arbitrary additive group, P,Q, Q=−P and a
   homomorphism f to `ZMod 5`, close `f Q=−f P` using rewriting and `map_neg`.
   Check it before the concrete source; retain the probe as text.
5. **Missing components only:** nonsingularity, point membership, Q=−P and
   the two doubling coordinates establishing order 5. Label incompatibility
   then follows from `map_neg` and `3 ≠ −1` in `ZMod 5`.
6. **Inheritance:** below. No generic law, upstream extension, resource-limit
   increase, user axiom or `native_decide` is introduced.

| Operation | Inherited owner / concrete obligation |
| --- | --- |
| Source field | Mathlib `ZMod.instField`, prime 59 |
| Source ellipticity | Nonzero literal discriminant |
| Point group | Mathlib affine point instance |
| Negation | `Point.neg_some`, literal y-coordinate equality |
| Addition | Mathlib affine addition formulas on actual points |
| Order 5 | `addOrderOf_eq_prime` |
| Required label relation | `map_neg` |
| Contradiction | Kernel-checked inequality in `ZMod 5` |

| Statement | Kind | Owner / hypotheses |
| --- | --- | --- |
| Literal point identities | Instance obligations | `ComponentLabelGate`, fixed curve and points |
| `P_order` | Endpoint | Existing prime-order law applied to the coordinate chain |
| `no_compatible_labels` | Endpoint | Existing homomorphism law and literal labels |
| `nodal_parameter_identity` | Instance obligation | One exact polynomial in `Polynomial ℤ`, discharged by `ring` |

The geometric interpretation of labels 1 and 3 comes from the native exact
rational points [13]S and [13]T and Cremona's classical component formula.
The Lean conclusion is exactly that those requested labels cannot be the
values of an additive homomorphism on the encoded source points. It does not
claim to formalize the component quotient or forbid every alternative lift.

The nodal identity only checks denominator-cleared parametrization of the
nodal cubic used in the target coordinate control. It does not implement
the Tate uniformization theorem. Its separate native IDC1 certificate points
to this exact Lean source; issuer observations are not independent replay.
