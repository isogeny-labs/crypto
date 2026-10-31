import Common22PointAllocation

/-!
The common-branch norm identity forces the interpolating numerator to contain
the linear branch factor at any rational branch point lying on the norm
support. This is a polynomial statement; it does not identify a compiled
runtime numerator or a global divisor.
-/

set_option autoImplicit false
noncomputable section

namespace Common22BranchFactorization

open Polynomial

variable {K : Type*} [Field K]

theorem branch_norm_forces_numerator_factor
    (f u1 u2 n support : K[X]) (b a : K)
    (hbranch : f.eval a = 0)
    (hpoly : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f)
    (hroot : u1.eval a * u2.eval a * support.eval a = 0) :
    X - C a ∣ n := by
  have hcurve : (0 : K) ^ 2 = f.eval a := by simp [hbranch]
  have hzeros := Common22PointAllocation.branch_overlap f u1 u2 n support
    b a 0 hcurve hpoly hroot rfl
  have hn : n.eval a = 0 := by simpa using hzeros.1
  exact dvd_iff_isRoot.mpr hn

#print axioms branch_norm_forces_numerator_factor

end Common22BranchFactorization
