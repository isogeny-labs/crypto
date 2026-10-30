import WeightedProjFiniteBranchDVR
import Common22BranchFactorization

/-!
Polynomial evaluation through the actual weighted-Proj finite-branch stalk
isomorphism. A factor `X-C a` maps to the local `x-a` germ. This is an
algebraic bridge, not a compiled-runtime or global divisor theorem.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjBranchPolynomialEval

open Polynomial Common22BranchStalk Common22BranchLocalParameter
  Common22SeparateChartMap WeightedProjFiniteBranchDVR

variable {K : Type*} [Field K]

private abbrev AffStalk (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :=
  Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)

def projectiveCoeffHom (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    K →+* ProjBranchRing a3 a2 a1 a0 a hroot :=
  (branchRingEquiv a3 a2 a1 a0 a hroot).symm.toRingHom.comp
    (algebraMap K (AffStalk a3 a2 a1 a0 a hroot))

def projectiveX (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    ProjBranchRing a3 a2 a1 a0 a hroot :=
  (branchRingEquiv a3 a2 a1 a0 a hroot).symm
    ((algebraMap (AffineRing a3 a2 a1 a0)
      (AffStalk a3 a2 a1 a0 a hroot)) (xbar a3 a2 a1 a0))

theorem projectiveX_sub_scalar (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    projectiveX a3 a2 a1 a0 a hroot -
      projectiveCoeffHom a3 a2 a1 a0 a hroot a =
        projectiveXSubA a3 a2 a1 a0 a hroot := by
  change (branchRingEquiv a3 a2 a1 a0 a hroot).symm
      ((algebraMap (AffineRing a3 a2 a1 a0)
        (AffStalk a3 a2 a1 a0 a hroot)) (xbar a3 a2 a1 a0)) -
    (branchRingEquiv a3 a2 a1 a0 a hroot).symm
      ((algebraMap K (AffStalk a3 a2 a1 a0 a hroot)) a) =
    (branchRingEquiv a3 a2 a1 a0 a hroot).symm
      ((algebraMap (AffineRing a3 a2 a1 a0)
        (AffStalk a3 a2 a1 a0 a hroot))
          (xbar a3 a2 a1 a0 -
            Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (MvPolynomial.C a)))
  simp only [map_sub]
  congr 1

theorem projective_eval_factor (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (n : K[X]) (hfactor : X - C a ∣ n) :
    ∃ t : ProjBranchRing a3 a2 a1 a0 a hroot,
      n.eval₂ (projectiveCoeffHom a3 a2 a1 a0 a hroot)
          (projectiveX a3 a2 a1 a0 a hroot) =
        projectiveXSubA a3 a2 a1 a0 a hroot * t := by
  rcases hfactor with ⟨q, rfl⟩
  refine ⟨q.eval₂ (projectiveCoeffHom a3 a2 a1 a0 a hroot)
    (projectiveX a3 a2 a1 a0 a hroot), ?_⟩
  rw [eval₂_mul, eval₂_sub, eval₂_X, eval₂_C,
    projectiveX_sub_scalar]

theorem projective_norm_branch_factor (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (f u1 u2 n support : K[X]) (b : K)
    (hbranch : f.eval a = 0)
    (hpoly : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f)
    (hsupport : u1.eval a * u2.eval a * support.eval a = 0) :
    ∃ t : ProjBranchRing a3 a2 a1 a0 a hroot,
      n.eval₂ (projectiveCoeffHom a3 a2 a1 a0 a hroot)
          (projectiveX a3 a2 a1 a0 a hroot) =
        projectiveXSubA a3 a2 a1 a0 a hroot * t :=
  projective_eval_factor a3 a2 a1 a0 a hroot n
    (Common22BranchFactorization.branch_norm_forces_numerator_factor
      f u1 u2 n support b a hbranch hpoly hsupport)

#print axioms projectiveX_sub_scalar
#print axioms projective_eval_factor
#print axioms projective_norm_branch_factor

end WeightedProjBranchPolynomialEval
