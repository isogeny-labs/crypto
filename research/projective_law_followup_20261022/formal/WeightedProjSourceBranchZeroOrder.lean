import WeightedProjNormBranchZeroOrder
import Common22SourceHead

/-!
Specialize the projective-stalk branch order theorem to the proof-side common
2+2 source coefficient expressions. Valid Mumford input congruences, the
normalised quintic head, rational branch/support root, and determinant guard
are explicit hypotheses. This does not prove Rust execution semantics or a
global divisor class.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjSourceBranchZeroOrder

open Polynomial Common22BranchStalk Common22BranchParameter
  WeightedProjFiniteBranchDVR WeightedProjBranchPolynomialEval

variable {K : Type*} [Field K]

theorem source_style_projective_branch_zero_order
    (a3 a2 a1 a0 branchX : K)
    (hroot : fAt a3 a2 a1 a0 branchX = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 branchX ≠ 0)
    (q1 r1 s1 t1 q2 r2 s2 t2 : K) (f : K[X])
    (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0)
    (valid1 : Common22Coprime.firstSupport q1 r1 ∣
      f - (C s1 * X + C t1) ^ 2)
    (valid2 : Common22Coprime.secondSupport q2 r2 ∣
      f - (C s2 * X + C t2) ^ 2)
    (f_degree : f.natDegree ≤ 5)
    (f_five : f.coeff 5 = 1)
    (f_four : f.coeff 4 = 0)
    (hbranch : f.eval branchX = 0) :
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
  dsimp
  intro hsupport
  let data := Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
  let b := data.1
  let alpha := data.2.1
  let c := data.2.2.1
  let q := data.2.2.2.1
  let r := data.2.2.2.2
  let u1 : K[X] := X ^ 2 + C q1 * X + C r1
  let u2 : K[X] := X ^ 2 + C q2 * X + C r2
  let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + alpha)) * X ^ 2 +
    C (b * s1 - c * r1 - alpha * q1) * X + C (b * t1 - alpha * r1)
  let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
  have hpoly : n ^ 2 - u1 * u2 * support = C (b ^ 2) * f := by
    have h := Common22SourceHead.source_curve_eq q1 r1 s1 t1 q2 r2 s2 t2
      f hb valid1 valid2 f_degree f_five f_four
    unfold Common22SourceHead.sourceE at h
    exact h
  exact WeightedProjNormBranchZeroOrder.projective_norm_branch_scalar_zero_order
    a3 a2 a1 a0 branchX hroot hsimple f u1 u2 n support b hb
    hbranch hpoly hsupport

#print axioms source_style_projective_branch_zero_order

end WeightedProjSourceBranchZeroOrder
