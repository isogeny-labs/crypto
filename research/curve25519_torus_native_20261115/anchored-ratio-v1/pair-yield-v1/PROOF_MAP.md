# Instance gate: concrete arithmetic for the pair-yield obstruction

1. Structure above: Lean Nat with exponent/product/order and decidable closed
   inequalities. Finite Cartesian-product cardinality is an existing Mathlib
   law, not a new study theorem.
2. Laws used as stated: Nat's kernel-reduced decidable inequalities for two
   exact parameter obligations; existing card_product/card_image_le support
   the separate readable finite-count argument.
3. Instances: B=213156431*20633 and ell=2^252+27742317777372353535851937790883648493.
   No prime-field/group implementation is constructed in Lean here.
4. Probe: private closed-arithmetic feasibility file compiles with no axioms.
   It preceded this new map; committed endpoint edits follow this map. This
   ordering is disclosed, not misreported as a prior gate.
5. Missing components: full finite-base/native map instance and stochastic
   draw/cost bridge remain written hypotheses. Generic counting laws stay
   in their owning libraries; no local re-derivation.
6. Inheritance: integer operations/inequalities are inherited directly from
   Nat; deciding these closed concrete expressions uses no native_decide,
   custom axiom or new limit. Source toolchain remains4.35.0.

| Written sentence | Endpoint / boundary |
| --- | --- |
| signed ordered pair targets plus zero <2^87 | signed_pair_plus_zero_lt, concrete Nat inequality only |
| original ell >=2^252 | ell_ge_two_pow252, concrete Nat inequality only |
| iid raw draw hit probability <2^-165 | readable finite image/union argument; not Lean sampler theorem |

Two endpoints are independent instance obligations, with no new generic
law or dependency chain. Arithmetic does not establish exact curve order,
point-set cardinality, a root finder or native execution. [pcc:unverified]
