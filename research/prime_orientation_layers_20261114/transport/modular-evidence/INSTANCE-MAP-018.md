# Actual paired weighted matrix unit

Above structures: existing integralRationals subring of Q, Mathlib Units of Pi and Matrix monoids, Matrix.diagonalRingHom and permutation matrices. Use Units.map, Units.val_inv/inv_val; Matrix.permMatrix_mul/permMatrix_one, Matrix.diagonal_mul and PEquiv.toMatrix_toPEquiv_apply. No generic inversion theorem is copied.

Actual object: weightUnit x packages -actual matching weight and its negative rational inverse, both integral by135. rowUnits is the pointwise product instance; originUnit is diagonalRingHom(rowUnits) times actual pairing permutation unit. Its rational entry S is supplied to actual kernel131.

Probe136 builds these objects and applies their unit laws directly. Missing instance obligations: actual sign/product coefficients; zero diagonal from pairing_no_fixed; matrix-entry equation from named diagonal/permutation laws. Then131 origin/node/diagonal/degree endpoints applied to this explicit S. No abstract favorable inverse premise, no matrix determinant computation or new generic induction.

Inheritance table: scalar two-sided inverse by existing nonzero rational mul_inv_cancel; Pi unit by funext of scalar unit laws; matrix diagonal unit via Units.map ring hom; permutation unit via named permMatrix composition; full inverse by Units.val_inv; integrality from subring membership; kernel endpoints via131. No retained chart gain/recovery/cheap preparation follows from this table. [pcc:unverified]
