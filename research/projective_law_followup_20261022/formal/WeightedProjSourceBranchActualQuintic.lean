import WeightedProjSourceBranchZeroOrder
import WeightedProjQuinticHead

/-!
Specialize the proof-side common 2+2 local order result to the actual
weighted-Proj monic quintic. The curve-polynomial evaluation and normalized
head are proved, rather than supplied as assumptions. Valid input divisor
congruences, the determinant guard, rational simple branch, and support root
remain explicit. This is not compiled-runtime or global Picard verification.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjSourceBranchActualQuintic

open Polynomial Common22BranchStalk Common22BranchParameter
  WeightedProjSmoothPoints WeightedProjFiniteBranchDVR
  WeightedProjBranchPolynomialEval

variable {K : Type*} [Field K]

theorem source_style_actual_quintic_branch_zero_order
    (a3 a2 a1 a0 branchX : K)
    (hroot : fAt a3 a2 a1 a0 branchX = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 branchX ≠ 0)
    (q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0)
    (valid1 : Common22Coprime.firstSupport q1 r1 ∣
      quintic a3 a2 a1 a0 - (C s1 * X + C t1) ^ 2)
    (valid2 : Common22Coprime.secondSupport q2 r2 ∣
      quintic a3 a2 a1 a0 - (C s2 * X + C t2) ^ 2) :
    let (b, alpha, c, q, r) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + alpha)) * X ^ 2 +
      C (b * s1 - c * r1 - alpha * q1) * X + C (b * t1 - alpha * r1)
    let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
    u1.eval branchX * u2.eval branchX * support.eval branchX = 0 →
    let R := ProjBranchRing a3 a2 a1 a0 branchX hroot
    @IsDiscreteValuationRing.addVal R _
      (projective_branch_domain a3 a2 a1 a0 branchX hroot hsimple)
      (projective_branch_dvr a3 a2 a1 a0 branchX hroot hsimple)
      (n.eval₂ (projectiveCoeffHom a3 a2 a1 a0 branchX hroot)
          (projectiveX a3 a2 a1 a0 branchX hroot) -
        projectiveCoeffHom a3 a2 a1 a0 branchX hroot b *
          projectiveY a3 a2 a1 a0 branchX hroot) = 1 := by
  have hbranch : (quintic a3 a2 a1 a0).eval branchX = 0 := by
    rw [WeightedProjQuinticHead.quintic_eval_eq_fAt]
    exact hroot
  exact WeightedProjSourceBranchZeroOrder.source_style_projective_branch_zero_order
    a3 a2 a1 a0 branchX hroot hsimple q1 r1 s1 t1 q2 r2 s2 t2
    (quintic a3 a2 a1 a0) hb valid1 valid2
    (WeightedProjQuinticHead.quintic_natDegree_le_five a3 a2 a1 a0)
    (WeightedProjQuinticHead.quintic_coeff_five a3 a2 a1 a0)
    (WeightedProjQuinticHead.quintic_coeff_four a3 a2 a1 a0)
    hbranch

theorem source_style_actual_quintic_branch_zero_order_of_separable
    (a3 a2 a1 a0 branchX : K)
    (hroot : fAt a3 a2 a1 a0 branchX = 0)
    (hsep : (quintic a3 a2 a1 a0).Separable)
    (q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0)
    (valid1 : Common22Coprime.firstSupport q1 r1 ∣
      quintic a3 a2 a1 a0 - (C s1 * X + C t1) ^ 2)
    (valid2 : Common22Coprime.secondSupport q2 r2 ∣
      quintic a3 a2 a1 a0 - (C s2 * X + C t2) ^ 2) :
    let (b, alpha, c, q, r) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + alpha)) * X ^ 2 +
      C (b * s1 - c * r1 - alpha * q1) * X + C (b * t1 - alpha * r1)
    let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
    u1.eval branchX * u2.eval branchX * support.eval branchX = 0 →
    let R := ProjBranchRing a3 a2 a1 a0 branchX hroot
    @IsDiscreteValuationRing.addVal R _
      (projective_branch_domain a3 a2 a1 a0 branchX hroot
        (WeightedProjQuinticHead.derivativeAt_ne_zero_of_separable
          a3 a2 a1 a0 branchX hsep hroot))
      (projective_branch_dvr a3 a2 a1 a0 branchX hroot
        (WeightedProjQuinticHead.derivativeAt_ne_zero_of_separable
          a3 a2 a1 a0 branchX hsep hroot))
      (n.eval₂ (projectiveCoeffHom a3 a2 a1 a0 branchX hroot)
          (projectiveX a3 a2 a1 a0 branchX hroot) -
        projectiveCoeffHom a3 a2 a1 a0 branchX hroot b *
          projectiveY a3 a2 a1 a0 branchX hroot) = 1 := by
  exact source_style_actual_quintic_branch_zero_order
    a3 a2 a1 a0 branchX hroot
    (WeightedProjQuinticHead.derivativeAt_ne_zero_of_separable
      a3 a2 a1 a0 branchX hsep hroot)
    q1 r1 s1 t1 q2 r2 s2 t2 hb valid1 valid2

#print axioms source_style_actual_quintic_branch_zero_order
#print axioms source_style_actual_quintic_branch_zero_order_of_separable

end WeightedProjSourceBranchActualQuintic
