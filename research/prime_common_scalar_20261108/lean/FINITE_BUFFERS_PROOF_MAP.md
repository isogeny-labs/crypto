# Finite coefficient-buffer proof map

All claims have unsigned producer standing. No analytic convergence, native
execution, bit-cost or full ECDLP algorithm theorem is asserted.
[pcc:unverified]

| Statement | Lean declaration | Exact scope |
|---|---|---|
| Jet membership iff low coefficients divisible | FiniteBuffers.jet_mem_iff_coeff_divisible | any CommRing, scalar c, any q and series |
| Actual finite-buffer equality | FiniteBuffers.buffer_eq_iff_jet | concrete A/(2^p), source jetIdeal; PCCff4290 |
| Earlier conditional inverse stability on buffers | FiniteBuffers.actual_inverse_buffer_stability | input buffers equal and E*F=1 |
| Generic inverse prefix independence | FiniteBuffers.inverse_low_coefficients | D*I=1, E*F=1, equal input prefix |
| Zero-extension coefficient identity | FiniteBuffers.liftBuffer_coeff | vector indexed Fin q |
| Actual lifted constant unit | FiniteBuffers.lifted_actual_constant | q>0, genuine mapped rhoUnit |
| Finite constructor correctness | FiniteBuffers.actual_invertBuffer_correct | all actual a,m,p,q; no supplied full inverse; PCC24cbda |
| Actual rational-to-padic map | ResidueEncoding.toPadic; coe_toPadic | norm-bounded rational subring, standard codRestrict |
| Precision kernel | ResidueEncoding.residue_kernel | kernel exactly coefficientIdeal p |
| Actual modular ring equivalence | ResidueEncoding.coefficient_equiv_exists; coefficientEquiv | A/(2^p) equivalent to ZMod2^p; PCCffec83 |
| Coefficient residue cardinality | ResidueEncoding.coefficient_card | exactly2^p classes |
| Denominator is unit | ResidueEncoding.denominator_unit | actual norm-bounded rational r |
| Ordinary residue formula | ResidueEncoding.residue_formula | num times inverse den modulo2^p; PCCffec83 |
| Constructor commutes with coefficient maps | ResidueEncoding.invert_mapped_actual | arbitrary coefficient ring hom, actual source |
| Concrete modular inverse output | ResidueEncoding.modular_inverse_correct | actual source all a,m,p,q; PCC24cbda |
| Modular buffer information size | ResidueEncoding.modular_buffer_card | exactly2^(p*q) vectors; PCCffec83 |

The known constant unit is an explicit input to invertBuffer and comes from
the actual source rhoUnit in the correctness theorem. This is not a free full-
series inversion oracle. The ordinary scalar modular inverse and native
runtime/bit representation must still be charged/certified separately.

Canonical PCC metadata was corrected after CI caught missing decl_sha256.
Original unsigned JSON records are retained in ../pcc-pin-repair/original-claims;
canonical statement/actual source receipts and appended registrations preserve
the entire transition. No theorem statement, scope, source body, verifier or
trust policy was changed to repair the metadata.
