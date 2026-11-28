# Actual linear curve remainder restricted to finite kernel

1. Structure: any FieldK, actual f:K[X], actual72coefficient encoding and70row anchored operator. Explicit valid-point equations only for evaluation consequences.
2. Laws AS STATED: modByMonicHom restricted scalars, LinearMap.comp/Submodule.subtype, natDegree_modByMonic_lt, natDegree_X_pow_sub_C, sum_modByMonic_coeff, eval₂_modByMonic_eq_self_of_root and earlier exact anchored_zero_iff.
3. Objects: remainderLinear f=(modByMonicHom(curveEquation f)).restrictScalars K. kernelRemainder composes it with coefficientEmbedding and the actual finiteOperator kernel subtype.
4. Chain: exact outer degree<2 -> reconstruct output as C(coeff0)+C(coeff1)*Y -> zero iff both actual coefficient polynomials zero. Actual kernel mapzero iff curve divisibility, and exact existence test uses this same map. Public target/pair evaluations vanish on valid curve input.
5. Does not prove inner weighted degree, nonzero image, full local jet transfer, extracted solver/norm/scalar/runtime. Actual degree<2 is outerY only.
6. Uses actual constructors and existing library linear maps; rejects hidden favorable reduction/kernel or rank assumptions. New RingDivision imported input pinned before code execution; no generic law rebuilt. [pcc:unverified]
