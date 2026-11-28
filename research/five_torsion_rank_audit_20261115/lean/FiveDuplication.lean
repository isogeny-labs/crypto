import FiveRootData

namespace FiveTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine ThreeTorsionRankAudit.General
variable {K : Type*} [Field K]

def doubleNumerator (W : WeierstrassCurve K) (x : K) : K :=
  x^4-W.b₄*x^2-2*W.b₆*x-W.b₈

theorem two_eval (W : WeierstrassCurve K) (x : K) :
    W.Ψ₂Sq.eval x=4*x^3+W.b₂*x^2+2*W.b₄*x+W.b₆ := by
  simp [WeierstrassCurve.Ψ₂Sq]

theorem doubleNumerator_identity (W : WeierstrassCurve K) (x : K) :
    doubleNumerator W x=x*W.Ψ₂Sq.eval x-W.Ψ₃.eval x := by
  simp [doubleNumerator,WeierstrassCurve.Ψ₂Sq,WeierstrassCurve.Ψ₃]
  ring

/-- A quartic Taylor identity keeps twice-doubling elimination small. -/
theorem doubleTaylor (W : WeierstrassCurve K) (x u : K) :
    doubleNumerator W u-x*W.Ψ₂Sq.eval u=
      (u-x)^4-(central W).eval x*(u-x)^2-2*W.Ψ₂Sq.eval x*(u-x)-W.Ψ₃.eval x := by
  simp [doubleNumerator,WeierstrassCurve.Ψ₂Sq,WeierstrassCurve.Ψ₃,central]
  ring

theorem twice_double_elimination (W : WeierstrassCurve K) (x u : K)
    (hd : W.Ψ₂Sq.eval x≠0) (h : doubleNumerator W x=W.Ψ₂Sq.eval x*u) :
    (W.Ψ₂Sq.eval x)^4*(doubleNumerator W u-W.Ψ₂Sq.eval u*x)=
      -W.Ψ₃.eval x*(psiFive W).eval x := by
  have hδ : u-x= -W.Ψ₃.eval x/W.Ψ₂Sq.eval x := by
    rw [doubleNumerator_identity] at h
    apply (eq_div_iff hd).mpr
    linear_combination -h
  rw [mul_comm (W.Ψ₂Sq.eval u) x,doubleTaylor,hδ,five_eval,fourth_eval]
  field_simp
  ring

variable [DecidableEq K]

theorem actual_double_relation (W : WeierstrassCurve.Affine K) {x y u v : K}
    (hN : W.Nonsingular x y) (hM : W.Nonsingular u v) (hy : y≠W.negY x y)
    (h : Point.some x y hN+Point.some x y hN=Point.some u v hM) :
    doubleNumerator W x=W.Ψ₂Sq.eval x*u := by
  have hd : W.Ψ₂Sq.eval x≠0 := by
    rw [two_eval]
    exact (W.den_duplication_eq_zero_iff hN.1).not.mpr hy
  have hh := (Point.add_self_of_Y_ne (h₁:=hN) hy).symm.trans h
  have hx := (Point.some.inj hh).1
  rw [W.addX_self_of_Y_ne hN.1 hy] at hx
  rw [←two_eval W x] at hx
  change doubleNumerator W x/W.Ψ₂Sq.eval x=u at hx
  exact (div_eq_iff hd).mp hx |>.trans (mul_comm u _)

theorem actual_double_x (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hy : y≠W.negY x y) :
    doubleNumerator W x=W.Ψ₂Sq.eval x*W.addX x x (W.slope x x y y) := by
  apply actual_double_relation W hN _ hy
  exact Point.add_self_of_Y_ne hy

#print axioms twice_double_elimination
#print axioms actual_double_relation
end
end FiveTorsionRankAudit
