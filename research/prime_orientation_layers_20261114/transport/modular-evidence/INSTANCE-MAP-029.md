# Actual dense integral inverse via existing determinant lemma

1. Structure: finite matrices over actual integralRationals and ZMod2, determinant/reindex/map/block laws; integral rational subtype.
2. Theorems AS STATED: Matrix.det_one_add_replicateCol_mul_replicateRow with Unit rank-one index and both vectors1; det_blockDiagonal, det_reindex_self, RingHom.map_det; reduce_zero_of_bound; padicValRat.inv; Matrix.mul_nonsing_inv/nonsing_inv_mul using actual constructed determinant unit.
3. Instance: Q=2^(a-u), u<a; fiber B=I+replicateCol1*replicateRow1 in ZMod2. Its determinant1+Q=1. Actual reduction from154 equals reindex(blockDiagonal B) via residueBlocks, not assumed.
4. Chain: actual fiber entry/Mathlib determinant -> exact block decomposition -> actual reduced determinant1 -> actual rational determinant nonzero and valuation0 -> explicit integral scalar Unit -> existing nonsingular inverse two-sided laws. No generic theorem missing; each new theorem is actual instance equality or endpoint.
5. Only missing components: actual fiber and reindex equalities, scalar-unit assembly. Numeric0/1 in ZMod2 by kernel decide, never native evaluation. Count every dense entry/bit cost later; no computational efficiency from abstract inverse.
6. Inheritance: rational inverse and Subring laws, determinant and adjugate inverse from Mathlib, fixed ring reduction from acceptedTransport. Scoped to a>u; original root admitted only for u>0. No child locality/weighted mode/runtime conclusion. [pcc:unverified]
