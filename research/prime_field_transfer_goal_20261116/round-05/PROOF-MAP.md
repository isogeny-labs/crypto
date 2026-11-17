# Round05 metric instance gate

1. **Structure above:** Mathlib `Matrix (Fin 2) (Fin 2) ℝ`, `Matrix.PosDef`,
   `Matrix.PosSemidef`, matrix conjugate transpose and multiplication. No new
   inner-product or LLL structure is defined in this study.
2. **Theorems used as stated:** `Matrix.posSemidef_conjTranspose_mul_self`,
   `Matrix.PosDef.one` and `Matrix.PosDef.add_posSemidef`. Finite matrix
   extensionality and real arithmetic discharge only literal entries.
3. **Object as instance:** G=!![2,-1;-1,2], R=!![1,-1], S=!![1,1]. Show
   G-I=RᴴR and3I-G=SᴴS. Then apply the existing PSD laws and positive-plus-PSD
   law. These are the exact pair coefficients of native coefficient_inner.
4. **Assembly probe:** type-check the existing-law conclusion I+AᴴA is positive
   definite for a1x2 real matrix and at the literal row R before endpoints.
5. **Missing components:** native vector encoding, repeated block metric and
   LLL implementation equivalence are not formalized; neither is coverage,
   representation height, uniform precision, CM specialization or full transfer.
   The original goal remains open. Any reusable missing LLL/CM law belongs in
   crypto-lean first, not as a study-local copy.
6. **Inheritance:** matrix ring operations and conjugate transpose are Mathlib
   instances; literal factorization uses entry extensionality and norm_num;
   PSD uses conjugate-transpose product; positive definiteness uses I plus PSD.

Proof chain: G-I=RᴴR (literal entries); G-I is PSD (existing law); G=I+(G-I)
(ring/module law); G is positive definite (existing positive-plus-PSD law).
Similarly3I-G=SᴴS implies PSD by the existing law. This verifies the concrete
coefficient metric; it is not a proof of the native program or a transfer.

| Planned declaration | Kind / owner | Scope |
| --- | --- | --- |
| `lower_certificate`, `upper_certificate` | instance / EisensteinMetric | two literal2x2 matrix identities |
| `lower_psd`, `upper_psd` | endpoint / EisensteinMetric | existing PSD law at displayed rows |
| `positive` | endpoint / EisensteinMetric | existing PD-plus-PSD law at G |

No limit increases, custom axioms or native_decide. New literal definitions
are opened only in the finite entry certificates; no composite LLL or elliptic
interface is unfolded. Standard foundations and the unchanged gate are audited
once after the actual endpoint/axiom files are ready.

Final owner namespace: EisensteinMetricV4; Real.instStarOrderedRing is imported from Mathlib.Data.Real.StarOrdered. Assembly04 and final gate02 pass; failed earlier sources remain text archives. Exact unsigned statement bindings are in receipts/round-05/lean-claims-01.json.
