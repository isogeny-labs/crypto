import GeneralBasis

namespace FiveTorsionRankAudit
noncomputable section
open Polynomial
variable {R : Type*} [CommRing R]

/-- The literal pinned-library fifth division polynomial. -/
abbrev psiFive (W : WeierstrassCurve R) : R[X] := W.preΨ' 5

def central (W : WeierstrassCurve R) : R[X] := 6*X^2+C W.b₂*X+C W.b₄

theorem five_formula (W : WeierstrassCurve R) :
    psiFive W=W.Ψ₂Sq^2*W.preΨ₄-W.Ψ₃^3 := by
  simpa [psiFive,mul_comm] using W.preΨ'_odd 0

theorem fourth_identity (W : WeierstrassCurve R) :
    W.preΨ₄=W.Ψ₃*central W-W.Ψ₂Sq^2 := by
  simp only [WeierstrassCurve.preΨ₄,WeierstrassCurve.Ψ₃,WeierstrassCurve.Ψ₂Sq,central,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  simp only [map_add,map_sub,map_mul,map_pow,map_ofNat]
  ring

theorem two_derivative (W : WeierstrassCurve R) : W.Ψ₂Sq.derivative=2*central W := by
  simp [WeierstrassCurve.Ψ₂Sq,central,Polynomial.derivative_pow]
  norm_num only [map_ofNat]
  ring

theorem three_derivative (W : WeierstrassCurve R) : W.Ψ₃.derivative=3*W.Ψ₂Sq := by
  simp [WeierstrassCurve.Ψ₃,WeierstrassCurve.Ψ₂Sq,Polynomial.derivative_pow]
  norm_num only [map_ofNat]
  ring

theorem central_identity (W : WeierstrassCurve R) :
    W.Ψ₂Sq*(central W).derivative=(central W)^2+4*W.Ψ₃ := by
  simp [central,WeierstrassCurve.Ψ₂Sq,WeierstrassCurve.Ψ₃,Polynomial.derivative_pow,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  norm_num only [map_ofNat]
  ring

theorem fourth_derivative (W : WeierstrassCurve R) :
    W.preΨ₄.derivative=W.Ψ₃*(central W).derivative-W.Ψ₂Sq*central W := by
  rw [fourth_identity]
  simp only [Polynomial.derivative_sub,Polynomial.derivative_mul,Polynomial.derivative_pow,
    two_derivative,three_derivative]
  norm_num only [map_ofNat]
  ring

theorem five_derivative (W : WeierstrassCurve R) :
    (psiFive W).derivative=5*W.Ψ₂Sq*(central W*W.preΨ₄-W.Ψ₃^2) := by
  rw [five_formula]
  simp only [Polynomial.derivative_sub,Polynomial.derivative_mul,Polynomial.derivative_pow,
    two_derivative,three_derivative,fourth_derivative]
  norm_num only [map_ofNat]
  rw [fourth_identity]
  linear_combination W.Ψ₂Sq*W.Ψ₃*central_identity W

/-- A small universal identity replaces a large resultant/discriminant certificate. -/
theorem root_derivative_identity (W : WeierstrassCurve R) :
    W.Ψ₃^2*W.preΨ₄=
      W.Ψ₂Sq^2*(central W*W.preΨ₄-W.Ψ₃^2)-central W*psiFive W := by
  rw [five_formula,fourth_identity]
  ring

theorem five_eval (W : WeierstrassCurve R) (x : R) :
    (psiFive W).eval x=(W.Ψ₂Sq.eval x)^2*W.preΨ₄.eval x-(W.Ψ₃.eval x)^3 := by
  simp [five_formula]

theorem fourth_eval (W : WeierstrassCurve R) (x : R) :
    W.preΨ₄.eval x=W.Ψ₃.eval x*(central W).eval x-(W.Ψ₂Sq.eval x)^2 := by
  simp [fourth_identity]

theorem five_derivative_eval (W : WeierstrassCurve R) (x : R) :
    (psiFive W).derivative.eval x=
      5*W.Ψ₂Sq.eval x*((central W).eval x*W.preΨ₄.eval x-(W.Ψ₃.eval x)^2) := by
  simp [five_derivative]

theorem five_root_identity_eval (W : WeierstrassCurve R) (x : R) :
    (W.Ψ₃.eval x)^2*W.preΨ₄.eval x=
      (W.Ψ₂Sq.eval x)^2*((central W).eval x*W.preΨ₄.eval x-(W.Ψ₃.eval x)^2)-
        (central W).eval x*(psiFive W).eval x := by
  simpa using congrArg (Polynomial.evalRingHom x) (root_derivative_identity W)

#print axioms fourth_identity
#print axioms five_derivative
#print axioms root_derivative_identity
end
end FiveTorsionRankAudit
