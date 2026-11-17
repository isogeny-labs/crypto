# Actual dense origin reduction and block instance

1. Structure: actual integralRationals subring, reduce1 ring homomorphism to ZMod2, finite matrix map/reindex; canonical actual Ports a and high/low finite product.
2. Laws used AS STATED: reduce_zero_of_bound, RingHom.map_sub/map_one/map_neg, weight_oneModTwo.integral, finProdFinEquiv/finCongr, Nat.mod_eq_of_lt and add_mul_mod_self_left. Next determinant uses existing Matrix.det_one_add_replicateCol_mul_replicateRow/det_blockDiagonal/det_reindex_self/isUnit_iff_isUnit_det, not a newly duplicated law.
3. Actual instance: dependent integral Uxy for matching ports, actual dense Sxy as negative Uxy for distinct matching ports and0 otherwise; physical ports identified by r+2^u*h with h<2^(a-u),r<2^u and u<=a.
4. Probe153 chain: VBound1(U-1) -> reduce1(U)=1 -> actual integral S reduces to distinct-same-residue indicator. Exact finite equivalence/entry formula shows residue equal iff low labels equal. No favorable determinant or inverse premise. The imported SchurComplement law is primary-source checked before use.
5. Missing components are actual subtype/reduction/index equalities. Generic matrix determinant laws already available; no owner edit. Preserve readonly/source/precall/raw boundaries and one process/default limits.
6. Inheritance: integralRationals operations from Subring; canonical map from reduce; bit decomposition from existing finProdFinEquiv. Full determinant and inverse not yet claimed; matrix density and preparation costs remain explicit. [pcc:unverified]
