import FiveBasis

namespace SevenTorsionRankAudit
noncomputable section
open Polynomial FiveTorsionRankAudit
variable {R : Type*} [CommRing R]

abbrev psiSeven (W : WeierstrassCurve R) : R[X] := W.preΨ' 7
def sevenAux (W : WeierstrassCurve R) : R[X] :=
  2*central W*W.preΨ₄*W.Ψ₃^3-2*W.Ψ₃^5-3*W.Ψ₃^2*W.preΨ₄^2-central W*W.preΨ₄^3

theorem seven_formula (W : WeierstrassCurve R) :
    psiSeven W=psiFive W*W.Ψ₃^3-W.Ψ₂Sq^2*W.preΨ₄^3 := by
  simpa [psiSeven,mul_comm] using W.preΨ'_odd 1

theorem seven_derivative (W : WeierstrassCurve R) :
    (psiSeven W).derivative=7*W.Ψ₂Sq*sevenAux W := by
  rw [seven_formula]
  simp only [Polynomial.derivative_sub,Polynomial.derivative_mul,Polynomial.derivative_pow,
    five_derivative,two_derivative,three_derivative,fourth_derivative]
  norm_num only [map_ofNat]
  unfold sevenAux
  rw [five_formula,fourth_identity]
  linear_combination -3*W.Ψ₂Sq*W.Ψ₃*(W.Ψ₃*central W-W.Ψ₂Sq^2)^2*central_identity W

theorem seven_root_derivative_identity (W : WeierstrassCurve R) :
    W.Ψ₃*sevenAux W-2*psiSeven W=W.preΨ₄^2*(psiFive W-W.preΨ₄^2) := by
  unfold sevenAux
  rw [seven_formula,five_formula,fourth_identity]
  ring

theorem seven_collapse_identity (W : WeierstrassCurve R) :
    psiSeven W+W.preΨ₄^2*psiFive W=(psiFive W-W.preΨ₄^2)*W.Ψ₃^3 := by
  rw [seven_formula,five_formula]
  ring

theorem seven_eval (W : WeierstrassCurve R) (x : R) :
    (psiSeven W).eval x=(psiFive W).eval x*(W.Ψ₃.eval x)^3-
      (W.Ψ₂Sq.eval x)^2*(W.preΨ₄.eval x)^3 := by simp [seven_formula]

theorem seven_derivative_eval (W : WeierstrassCurve R) (x : R) :
    (psiSeven W).derivative.eval x=7*W.Ψ₂Sq.eval x*(sevenAux W).eval x := by
  simp [seven_derivative]

theorem seven_root_derivative_eval (W : WeierstrassCurve R) (x : R) :
    W.Ψ₃.eval x*(sevenAux W).eval x-2*(psiSeven W).eval x=
      (W.preΨ₄.eval x)^2*((psiFive W).eval x-(W.preΨ₄.eval x)^2) := by
  simpa using congrArg (Polynomial.evalRingHom x) (seven_root_derivative_identity W)

theorem seven_collapse_eval (W : WeierstrassCurve R) (x : R) :
    (psiSeven W).eval x+(W.preΨ₄.eval x)^2*(psiFive W).eval x=
      ((psiFive W).eval x-(W.preΨ₄.eval x)^2)*(W.Ψ₃.eval x)^3 := by
  simpa using congrArg (Polynomial.evalRingHom x) (seven_collapse_identity W)

#print axioms seven_derivative
#print axioms seven_root_derivative_identity
end
end SevenTorsionRankAudit
