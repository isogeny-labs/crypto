import Common22BranchDomain

set_option autoImplicit false
noncomputable section

namespace Common22BranchZeroOrder

variable {R : Type*} [CommRing R] [IsDomain R] [IsDiscreteValuationRing R]

theorem branch_zero_order (y t B : R)
    (hy : y ∈ nonunits R)
    (hy_order : IsDiscreteValuationRing.addVal R y = 1)
    (hB : IsUnit B) :
    IsDiscreteValuationRing.addVal R (y * y * t - B * y) = 1 := by
  have hyt : y * t ∈ nonunits R := by
    intro h
    exact hy ((IsUnit.mul_iff.mp h).1)
  have hneg : -(y * t) ∈ nonunits R := by
    intro h
    exact hyt (by simpa using h.neg)
  have hunit : IsUnit (y * t - B) := by
    by_contra hh
    have hsum : (y * t - B) + -(y * t) ∈ nonunits R :=
      IsLocalRing.nonunits_add hh hneg
    have heq : (y * t - B) + -(y * t) = -B := by ring
    rw [heq] at hsum
    exact hsum hB.neg
  have hfactor : y * y * t - B * y = y * (y * t - B) := by ring
  rw [hfactor, IsDiscreteValuationRing.addVal_mul,
    IsDiscreteValuationRing.addVal_eq_zero_iff.mpr hunit,
    hy_order, add_zero]

#print axioms branch_zero_order

open Common22BranchStalk Common22BranchDomain Common22BranchLocalParameter

theorem actual_simple_branch_zero_order
    {K : Type*} [Field K] (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : Common22BranchParameter.derivativeAt a3 a2 a1 a0 a ≠ 0) :
    let R := Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)
    ∀ (t B : R), IsUnit B →
      @IsDiscreteValuationRing.addVal R _
        (branch_stalk_domain a3 a2 a1 a0 a hroot hsimple)
        (branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple)
        (stalkY a3 a2 a1 a0 a hroot * stalkY a3 a2 a1 a0 a hroot * t -
          B * stalkY a3 a2 a1 a0 a hroot) = 1 := by
  dsimp
  intro t B hB
  letI : IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDiscreteValuationRing
      (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple
  have hy : stalkY a3 a2 a1 a0 a hroot ∈
      nonunits (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    (branch_stalk_y_irreducible a3 a2 a1 a0 a hroot hsimple).not_isUnit
  have hy_order := branch_order_y_eq_one a3 a2 a1 a0 a hroot hsimple
  exact branch_zero_order (stalkY a3 a2 a1 a0 a hroot) t B hy hy_order hB

#print axioms actual_simple_branch_zero_order

theorem actual_branch_x_factor_zero_order
    {K : Type*} [Field K] (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : Common22BranchParameter.derivativeAt a3 a2 a1 a0 a ≠ 0) :
    let R := Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)
    ∀ (N B t : R),
      N = stalkXSubA a3 a2 a1 a0 a hroot * t → IsUnit B →
      @IsDiscreteValuationRing.addVal R _
        (branch_stalk_domain a3 a2 a1 a0 a hroot hsimple)
        (branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple)
        (N - B * stalkY a3 a2 a1 a0 a hroot) = 1 := by
  dsimp
  intro N B t hN hB
  let R := Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)
  let qUnit : Rˣ :=
    (Common22BranchParameter.branch_qAt_local_unit a3 a2 a1 a0 a hroot hsimple).unit
  have hq : (qUnit : R) = stalkQ a3 a2 a1 a0 a hroot :=
    (Common22BranchParameter.branch_qAt_local_unit a3 a2 a1 a0 a hroot hsimple).unit_spec
  have hqmul : stalkQ a3 a2 a1 a0 a hroot * (↑(qUnit⁻¹) : R) = 1 := by
    rw [← hq]
    simp
  have hx : stalkXSubA a3 a2 a1 a0 a hroot =
      stalkY a3 a2 a1 a0 a hroot * stalkY a3 a2 a1 a0 a hroot *
        (↑(qUnit⁻¹) : R) := by
    have hrel := stalk_relation a3 a2 a1 a0 a hroot
    calc
      stalkXSubA a3 a2 a1 a0 a hroot =
          stalkXSubA a3 a2 a1 a0 a hroot *
            (stalkQ a3 a2 a1 a0 a hroot * (↑(qUnit⁻¹) : R)) := by rw [hqmul]; ring
      _ = (stalkXSubA a3 a2 a1 a0 a hroot * stalkQ a3 a2 a1 a0 a hroot) *
            (↑(qUnit⁻¹) : R) := by ring
      _ = stalkY a3 a2 a1 a0 a hroot * stalkY a3 a2 a1 a0 a hroot *
            (↑(qUnit⁻¹) : R) := by rw [← hrel]; ring
  have h := actual_simple_branch_zero_order a3 a2 a1 a0 a hroot hsimple
    ((↑(qUnit⁻¹) : R) * t) B hB
  rw [hN, hx]
  convert h using 1 <;> ring

#print axioms actual_branch_x_factor_zero_order

end Common22BranchZeroOrder
