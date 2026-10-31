import Common22BranchParameter
import Mathlib.RingTheory.DiscreteValuationRing.Basic

/-!
# Conditional valuation relation at the actual affine branch stalk

The local factor `q_a` is a unit at a simple branch root.  If the actual
localization is a DVR, the valuation of `x-a` is twice the valuation of `y`.
This theorem does not establish the DVR instance or that `y` has order one.
-/

set_option autoImplicit false
noncomputable section

namespace Common22BranchValuation

open Common22SeparateChartMap Common22BranchStalk Common22BranchParameter

variable {K : Type*} [Field K]

theorem branch_order_relation (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0)
    [IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot))]
    [IsDiscreteValuationRing (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot))] :
    let A := AffineRing a3 a2 a1 a0
    let R := Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)
    IsDiscreteValuationRing.addVal R
      ((algebraMap A R) (xbar a3 a2 a1 a0 -
        algebraMap K A a)) =
      2 • IsDiscreteValuationRing.addVal R
        ((algebraMap A R) (ybar a3 a2 a1 a0)) := by
  dsimp
  let A := AffineRing a3 a2 a1 a0
  let R := Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)
  let φ := algebraMap A R
  have hrel := congrArg φ (branch_factorization a3 a2 a1 a0 a hroot)
  simp only [map_pow, map_mul, map_sub] at hrel
  have hu : IsUnit (φ (qAt a3 a2 a1 a0 a)) :=
    branch_qAt_local_unit a3 a2 a1 a0 a hroot hsimple
  have hqval : IsDiscreteValuationRing.addVal R (φ (qAt a3 a2 a1 a0 a)) = 0 :=
    IsDiscreteValuationRing.addVal_eq_zero_iff.mpr hu
  have hv := congrArg (IsDiscreteValuationRing.addVal R) hrel
  rw [IsDiscreteValuationRing.addVal_pow,
    IsDiscreteValuationRing.addVal_mul,
    hqval, add_zero] at hv
  simpa only [map_sub] using hv.symm

#print axioms branch_order_relation

end Common22BranchValuation
