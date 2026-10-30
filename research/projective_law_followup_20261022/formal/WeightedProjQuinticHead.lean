import WeightedProjSmoothPoints
import Common22BranchStalk
import Common22BranchParameter
import Mathlib.Tactic.ComputeDegree

/-!
Bind the separately defined monic quintic used by the weighted-Proj smoothness
proof to the affine branch evaluator and the normalised polynomial head needed
by the proof-side common 2+2 norm identity.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjQuinticHead

open Polynomial Common22BranchStalk WeightedProjSmoothPoints

variable {K : Type*} [Field K]

theorem quintic_eval_eq_fAt (a3 a2 a1 a0 x : K) :
    (quintic a3 a2 a1 a0).eval x = fAt a3 a2 a1 a0 x := by
  simp [quintic, fAt]

theorem quintic_natDegree_le_five (a3 a2 a1 a0 : K) :
    (quintic a3 a2 a1 a0).natDegree ≤ 5 := by
  unfold quintic
  compute_degree!

theorem quintic_coeff_five (a3 a2 a1 a0 : K) :
    (quintic a3 a2 a1 a0).coeff 5 = 1 := by
  simp [quintic]

theorem quintic_coeff_four (a3 a2 a1 a0 : K) :
    (quintic a3 a2 a1 a0).coeff 4 = 0 := by
  simp [quintic]

theorem quintic_derivative_eval_eq_derivativeAt (a3 a2 a1 a0 x : K) :
    (quintic a3 a2 a1 a0).derivative.eval x =
      Common22BranchParameter.derivativeAt a3 a2 a1 a0 x := by
  simp [quintic, Common22BranchParameter.derivativeAt,
    derivative_add, derivative_mul, derivative_pow]
  ring

theorem derivativeAt_ne_zero_of_separable (a3 a2 a1 a0 x : K)
    (hsep : (quintic a3 a2 a1 a0).Separable)
    (hroot : fAt a3 a2 a1 a0 x = 0) :
    Common22BranchParameter.derivativeAt a3 a2 a1 a0 x ≠ 0 := by
  have hr : (quintic a3 a2 a1 a0).eval x = 0 := by
    rw [quintic_eval_eq_fAt]
    exact hroot
  rw [← quintic_derivative_eval_eq_derivativeAt]
  exact hsep.eval₂_derivative_ne_zero (RingHom.id K) (by simpa using hr)

#print axioms quintic_eval_eq_fAt
#print axioms quintic_natDegree_le_five
#print axioms quintic_coeff_five
#print axioms quintic_coeff_four
#print axioms quintic_derivative_eval_eq_derivativeAt
#print axioms derivativeAt_ne_zero_of_separable

end WeightedProjQuinticHead
