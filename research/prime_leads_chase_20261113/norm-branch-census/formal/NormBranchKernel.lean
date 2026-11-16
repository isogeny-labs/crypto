import Mathlib.Tactic.Ring

/-!
# The two-by-two norm-branch product-kernel factor

This is the polynomial identity behind the smallest coefficient-block
counterexample.  It proves the exact determinant factorization, not a growing
rank defect or an algorithmic consequence.
-/

set_option autoImplicit false

namespace NormBranchKernel

variable {R : Type*} [CommRing R]

/-- Raw unsigned Laplace coefficients for global point order `a,b,c,d,e`. -/
def c00 (a b c d e : R) := (c-a) * (d-b) * (e-b) * (e-d)
def c01 (a b c d e : R) := (d-a) * (c-b) * (e-b) * (e-c)
def c10 (a b c d e : R) := (c-b) * (d-a) * (e-a) * (e-d)
def c11 (a b c d e : R) := (d-b) * (c-a) * (e-a) * (e-c)

/-- The raw block determinant is a nonzero-node factor times the determinant
of the squared cross-difference product kernel. -/
theorem determinant_product_kernel (a b c d e : R) :
    c00 a b c d e * c11 a b c d e -
        c01 a b c d e * c10 a b c d e =
      ((e-a)*(e-b)*(e-c)*(e-d)) *
        (((c-a)*(c-a))*((d-b)*(d-b)) -
          ((d-a)*(d-a))*((c-b)*(c-b))) := by
  simp [c00, c01, c10, c11]
  ring

#print axioms determinant_product_kernel

end NormBranchKernel
