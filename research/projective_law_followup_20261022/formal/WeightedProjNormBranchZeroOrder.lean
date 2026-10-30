import WeightedProjBranchPolynomialEval
import WeightedProjBranchZeroOrder

/-!
Compose the checked polynomial norm factor, its transport into the actual
weighted-Proj stalk, and the conditional local order-one theorem. The norm
identity and unit coefficient remain explicit hypotheses; no compiled-law or
global divisor-class claim is made.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjNormBranchZeroOrder

open Polynomial Common22BranchStalk Common22BranchParameter
  WeightedProjFiniteBranchDVR WeightedProjBranchPolynomialEval

variable {K : Type*} [Field K]

theorem projective_norm_branch_zero_order (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0)
    (f u1 u2 n support : K[X]) (b : K)
    (hbranch : f.eval a = 0)
    (hpoly : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f)
    (hsupport : u1.eval a * u2.eval a * support.eval a = 0) :
    let R := ProjBranchRing a3 a2 a1 a0 a hroot
    ∀ B : R, IsUnit B →
      @IsDiscreteValuationRing.addVal R _
        (projective_branch_domain a3 a2 a1 a0 a hroot hsimple)
        (projective_branch_dvr a3 a2 a1 a0 a hroot hsimple)
        (n.eval₂ (projectiveCoeffHom a3 a2 a1 a0 a hroot)
            (projectiveX a3 a2 a1 a0 a hroot) -
          B * projectiveY a3 a2 a1 a0 a hroot) = 1 := by
  dsimp
  intro B hB
  obtain ⟨t, ht⟩ := projective_norm_branch_factor
    a3 a2 a1 a0 a hroot f u1 u2 n support b hbranch hpoly hsupport
  exact WeightedProjBranchZeroOrder.projective_branch_x_factor_zero_order
    a3 a2 a1 a0 a hroot hsimple
    (n.eval₂ (projectiveCoeffHom a3 a2 a1 a0 a hroot)
      (projectiveX a3 a2 a1 a0 a hroot)) B t ht hB

theorem projective_coeff_unit (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (b : K) (hb : b ≠ 0) :
    IsUnit (projectiveCoeffHom a3 a2 a1 a0 a hroot b) := by
  have h : IsUnit b := isUnit_iff_ne_zero.mpr hb
  exact h.map (projectiveCoeffHom a3 a2 a1 a0 a hroot)

theorem projective_norm_branch_scalar_zero_order (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0)
    (f u1 u2 n support : K[X]) (b : K)
    (hb : b ≠ 0)
    (hbranch : f.eval a = 0)
    (hpoly : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f)
    (hsupport : u1.eval a * u2.eval a * support.eval a = 0) :
    let R := ProjBranchRing a3 a2 a1 a0 a hroot
    @IsDiscreteValuationRing.addVal R _
      (projective_branch_domain a3 a2 a1 a0 a hroot hsimple)
      (projective_branch_dvr a3 a2 a1 a0 a hroot hsimple)
      (n.eval₂ (projectiveCoeffHom a3 a2 a1 a0 a hroot)
          (projectiveX a3 a2 a1 a0 a hroot) -
        projectiveCoeffHom a3 a2 a1 a0 a hroot b *
          projectiveY a3 a2 a1 a0 a hroot) = 1 := by
  exact projective_norm_branch_zero_order a3 a2 a1 a0 a
    hroot hsimple f u1 u2 n support b hbranch hpoly hsupport
    (projectiveCoeffHom a3 a2 a1 a0 a hroot b)
    (projective_coeff_unit a3 a2 a1 a0 a hroot b hb)

#print axioms projective_norm_branch_zero_order
#print axioms projective_coeff_unit
#print axioms projective_norm_branch_scalar_zero_order

end WeightedProjNormBranchZeroOrder
