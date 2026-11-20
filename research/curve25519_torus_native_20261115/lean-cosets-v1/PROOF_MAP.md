# Many-coset numeric instance gate

1. Structure above: Mathlib `Field`, `IsPrimitiveRoot`, natural powers,
   `Fin`, products and `Set.range`. Fixed numeric intervals are instance
   obligations, not a new generic theory of coset unions.
2. Theorems used as stated: `IsPrimitiveRoot.pow_inj`,
   `pow_ne_one_of_pos_of_lt`, `pow_eq_one_iff_dvd`, `pow_of_dvd`,
   `eq_pow_of_pow_eq_one`, `Odd.neg_pow`, and
   `Set.ncard_range_of_injective`. Their proofs remain in Mathlib.
3. Object: the image of `(j,t) ↦ h^(j.val+1+60824497*t.val)` on
   `Fin 20633 × Fin 213156431`, conditional on
   `IsPrimitiveRoot h 12965132697890207`. The actual Fp2 point/order and
   norm-one embedding are not supplied by this abstract-field contract.
4. Assembly probe: apply `pow_inj` with two bounded exponent variables;
   apply `eq_pow_of_pow_eq_one` to the primitive root `h^60824497`.
   The fixed exponents/order and product cardinality fit by decidable
   natural arithmetic. Build the probe before naming the endpoints.
5. Missing components: an independently certified actual Fp2 field/point
   instance, Elligator/cofactor/selection bridge, normalized point-base
   cardinality and extraction, complete decomposition cost/yield, geometric
   energy for any changed arity, and an admissible growing family.
6. Inheritance: multiplication/powers/negation/division from `Field`;
   exact-order and root enumeration from `IsPrimitiveRoot`; product-index
   size from existing Fin/product cardinality; image counting from
   `Set.ncard_range_of_injective`. Fixed coefficient bounds use `omega`;
   numeric equalities use `norm_num`. No new limits, axioms or native proof.

Proof chain: equality of encoded powers ⇒ equality of exponents by
`pow_inj` ⇒ equal quotient/remainder indices by the fixed numeric bounds.
Apply range cardinality to the resulting injection. Any product being one
forces its exponent sum to be divisible by M, hence by60824497; its residue
lies strictly between zero and60824497. For signs, every encoded element
has Mth power one and M is odd, so equality to a negative would imply
one=-one (characteristic two must be excluded explicitly). Membership is
the root-of-unity parametrization of each fiber after dividing by h^(j+1).

| Statement | Kind | Owner | Existing law / new obligation |
|---|---|---|---|
| Anonymous root/order probes | Assembly | Study | Apply Mathlib laws verbatim |
| Bounded exponent uniqueness | Numeric instance | Study | `pow_inj` + fixed interval arithmetic |
| Raw image cardinality | Endpoint | Study | `Set.ncard_range_of_injective` |
| Identity/inverse/sign exclusion | Numeric instance | Study | Existing root/order/odd-power laws |
| Membership fiber | Endpoint | Study | `eq_pow_of_pow_eq_one` and field rewrites |

No generic coset/cardinality theorem is copied or proved here. If assembly
requires a missing generic law or a long proof, stop this instance and add
that law in crypto-lean first. The complete new configuration, source and
dependency manifest will be pinned together after the final audit.
