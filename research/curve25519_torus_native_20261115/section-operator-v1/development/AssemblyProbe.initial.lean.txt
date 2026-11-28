module
public import Mathlib.LinearAlgebra.Matrix.Nondegenerate

open scoped Matrix

-- Assembly only: this does not bind the native matrix to elliptic geometry.
example {K : Type*} [Field K] (M : Matrix (Fin 6) (Fin 6) K)
    (c : Fin 6 → K) (hne : c ≠ 0) (hker : M *ᵥ c = 0) : M.det = 0 := by
  by_contra hdet
  exact hne (Matrix.eq_zero_of_mulVec_eq_zero hdet hker)

#print axioms Matrix.eq_zero_of_mulVec_eq_zero
