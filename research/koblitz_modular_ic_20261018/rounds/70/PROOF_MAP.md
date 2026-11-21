# R70 proof map

| Claim | Exact evidence | Boundary |
|---|---|---|
| The final two-class obstruction has zero kernel | Accepted `sixth-frobenius70-02` complete 24-by-2 binary map; rows 1,2 are `(1,0),(1,1)`; `RECORD_RESULTS.py` checks their saved bytes | Native map construction is external to Lean |
| Those two rows force zero coefficients | `Frobenius70.saved_row_minor_zero`, compiled and axiom-audited | Applied to the native row certificate |
| `V6=O` | Accepted native 32-by-32 field identity, old-order basis, quotient length zero and all basis-square checks | Affine `A=F2[z]` only |
| Integral closure lies in `V0` | Nondegenerate trace Gram from R61/R65; trace of an integral element times each order basis vector lies in integrally closed `A` | Mathematical trace-integrality argument; not Lean-formalized here |
| Integral closure stays in all six steps | `Frobenius70.closure_subset_descend`, compiled/audited | Assumes trace-dual containment and closure under squaring |
| Affine order equals integral closure | `Frobenius70.closure_eq_order_of_six_steps`, compiled/audited, applied to native `V6=O` | Source-field identification and native equality are explicit premises |

The strict native gate rejects `sixth-frobenius70-01` despite its late
marker and exit 0 because it has zero-column Singular errors. The first
Lean compile is rejected. The accepted revised native and final Lean
runs are separate fresh attempts, both bound to historical Git bytes.
No theorem here certifies the proper/projective model, polarization,
level-77 map, order-11 action, or reverse divisor evaluation.
