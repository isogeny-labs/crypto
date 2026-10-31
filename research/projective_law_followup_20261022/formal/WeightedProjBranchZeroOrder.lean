import WeightedProjFiniteBranchDVR
import Common22BranchZeroOrder

/-!
At a rational simple branch of the actual weighted-Proj curve, the local
numerator `N - B*y` has order one when `N` is divisible by `x-a` and `B` is a
unit. This does not establish the factorization for the compiled law or a
global divisor-class equation.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjBranchZeroOrder

open WeightedProjFiniteBranchDVR Common22BranchStalk Common22BranchParameter

variable {K : Type*} [Field K]

theorem projective_branch_x_factor_zero_order (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    let R := ProjBranchRing a3 a2 a1 a0 a hroot
    ∀ (N B t : R),
      N = projectiveXSubA a3 a2 a1 a0 a hroot * t → IsUnit B →
      @IsDiscreteValuationRing.addVal R _
        (projective_branch_domain a3 a2 a1 a0 a hroot hsimple)
        (projective_branch_dvr a3 a2 a1 a0 a hroot hsimple)
        (N - B * projectiveY a3 a2 a1 a0 a hroot) = 1 := by
  dsimp
  intro N B t hN hB
  let R := ProjBranchRing a3 a2 a1 a0 a hroot
  letI : IsDomain R := projective_branch_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDiscreteValuationRing R := projective_branch_dvr a3 a2 a1 a0 a hroot hsimple
  let hq := projectiveQ_unit a3 a2 a1 a0 a hroot hsimple
  let q : Rˣ := hq.unit
  have hqeq : (q : R) = projectiveQ a3 a2 a1 a0 a hroot := hq.unit_spec
  have hqmul : projectiveQ a3 a2 a1 a0 a hroot * (↑(q⁻¹) : R) = 1 := by
    rw [← hqeq]
    simp
  have hx : projectiveXSubA a3 a2 a1 a0 a hroot =
      projectiveY a3 a2 a1 a0 a hroot *
        projectiveY a3 a2 a1 a0 a hroot * (↑(q⁻¹) : R) := by
    have hrel := projective_relation a3 a2 a1 a0 a hroot
    calc
      projectiveXSubA a3 a2 a1 a0 a hroot =
          projectiveXSubA a3 a2 a1 a0 a hroot *
            (projectiveQ a3 a2 a1 a0 a hroot * (↑(q⁻¹) : R)) := by
              rw [hqmul]
              ring
      _ = (projectiveXSubA a3 a2 a1 a0 a hroot *
          projectiveQ a3 a2 a1 a0 a hroot) * (↑(q⁻¹) : R) := by ring
      _ = projectiveY a3 a2 a1 a0 a hroot *
          projectiveY a3 a2 a1 a0 a hroot * (↑(q⁻¹) : R) := by
            rw [← hrel]
            ring
  have hy : projectiveY a3 a2 a1 a0 a hroot ∈ nonunits R :=
    (projectiveY_irreducible a3 a2 a1 a0 a hroot hsimple).not_isUnit
  have hyorder := projectiveY_order_one a3 a2 a1 a0 a hroot hsimple
  have h := Common22BranchZeroOrder.branch_zero_order
    (projectiveY a3 a2 a1 a0 a hroot) ((↑(q⁻¹) : R) * t) B
    hy hyorder hB
  rw [hN, hx]
  convert h using 1 <;> ring

#print axioms projective_branch_x_factor_zero_order

end WeightedProjBranchZeroOrder
