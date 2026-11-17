import DivisionQuartic

namespace ThreeTorsionRankAudit.General
open Polynomial WeierstrassCurve.Affine
noncomputable section
variable {K : Type*} [Field K]

def numerator (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  3*x^2+2*W.a₂*x+W.a₄-W.a₁*y
def denominator (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  2*y+W.a₁*x+W.a₃
def duplicationDen (W : WeierstrassCurve.Affine K) (x : K) : K :=
  4*x^3+W.b₂*x^2+2*W.b₄*x+W.b₆

/-- Universal coordinate identity, before imposing the curve equation. -/
theorem psi_coordinate_identity (W : WeierstrassCurve.Affine K) (x y : K) :
    W.Ψ₃.eval x = (3*x+W.a₂)*(denominator W x y)^2-
      W.a₁*numerator W x y*denominator W x y-(numerator W x y)^2-
      (12*x+W.b₂)*W.polynomial.evalEval x y := by
  simp [WeierstrassCurve.Ψ₃,numerator,denominator,WeierstrassCurve.Affine.evalEval_polynomial,
    WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  ring

theorem psi_at_point (W : WeierstrassCurve.Affine K) {x y : K} (he : W.Equation x y) :
    W.Ψ₃.eval x = (3*x+W.a₂)*(denominator W x y)^2-
      W.a₁*numerator W x y*denominator W x y-(numerator W x y)^2 := by
  rw [psi_coordinate_identity W x y]
  rw [show W.polynomial.evalEval x y=0 from he]
  ring

theorem psi_root_denominator_nonzero (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hx : W.Ψ₃.IsRoot x) : denominator W x y ≠ 0 := by
  intro hd
  have hv := psi_at_point W hN.1
  have hx' : W.Ψ₃.eval x=0 := hx
  have hn : (numerator W x y)^2=0 := by rw [hx',hd] at hv; linear_combination hv
  have hn0 := sq_eq_zero_iff.mp hn
  have hX : W.a₁*y-(3*x^2+2*W.a₂*x+W.a₄)=0 := by
    unfold numerator at hn0
    linear_combination -hn0
  have hs := (W.nonsingular_iff' x y).mp hN
  exact hs.2.elim (fun h => h hX) (fun h => h hd)

theorem denominator_ne_iff (W : WeierstrassCurve.Affine K) (x y : K) :
    denominator W x y ≠ 0 ↔ y ≠ W.negY x y := by
  unfold denominator WeierstrassCurve.Affine.negY
  constructor
  · intro hd he
    apply hd
    linear_combination he
  · intro hy hd
    apply hy
    linear_combination hd

theorem psi_duplication_identity (W : WeierstrassCurve.Affine K) (x : K) :
    W.Ψ₃.eval x = x*duplicationDen W x-(x^4-W.b₄*x^2-2*W.b₆*x-W.b₈) := by
  simp [WeierstrassCurve.Ψ₃,duplicationDen]
  ring

variable [DecidableEq K]

theorem double_eq_neg_iff_x (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hy : y ≠ W.negY x y) :
    Point.some x y hN+Point.some x y hN = -Point.some x y hN ↔
      W.addX x x (W.slope x x y y)=x := by
  have hd := W.nonsingular_add hN hN (fun hxy => hy hxy.2)
  rw [Point.add_self_of_Y_ne (h₁ := hN) hy]
  constructor
  · intro he
    exact Point.X_eq_iff.mpr (Or.inr he)
  · intro hx
    rcases (Point.X_eq_iff (h₁ := hd) (h₂ := hN)).mp hx with hh | hh
    · exfalso
      have hPP := (Point.add_self_of_Y_ne (h₁ := hN) hy).trans hh
      apply Point.some_ne_zero hN
      exact add_left_cancel (hPP.trans (add_zero _).symm)
    · exact hh

theorem double_x_eq_iff_psi (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hy : y ≠ W.negY x y) :
    W.addX x x (W.slope x x y y)=x ↔ W.Ψ₃.IsRoot x := by
  have hd : duplicationDen W x ≠ 0 :=
    (W.den_duplication_eq_zero_iff hN.1).not.mpr hy
  rw [WeierstrassCurve.Affine.addX_self_of_Y_ne hN.1 hy]
  change ((x^4-W.b₄*x^2-2*W.b₆*x-W.b₈)/duplicationDen W x=x) ↔ W.Ψ₃.eval x=0
  rw [div_eq_iff hd,psi_duplication_identity]
  constructor <;> intro h <;> linear_combination -h

theorem three_point_not_inverse (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (h3 : (3 : ℕ) • Point.some x y hN=0) : y ≠ W.negY x y := by
  intro hy
  have htwo := Point.add_self_of_Y_eq (h₁ := hN) hy
  rw [show (3 : ℕ)=2+1 by rfl,add_nsmul,two_nsmul,one_nsmul,htwo,zero_add] at h3
  exact Point.some_ne_zero hN h3

/-- Actual nonsingular point criterion in every characteristic; no discriminant or rank premise. -/
theorem three_point_iff (W : WeierstrassCurve.Affine K) {x y : K} (hN : W.Nonsingular x y) :
    (3 : ℕ) • Point.some x y hN=0 ↔ W.Ψ₃.IsRoot x := by
  have hconditional (hy : y ≠ W.negY x y) :
      (3 : ℕ) • Point.some x y hN=0 ↔ W.Ψ₃.IsRoot x := by
    rw [show (3 : ℕ)=2+1 by rfl,add_nsmul,two_nsmul,one_nsmul,add_eq_zero_iff_eq_neg,
      double_eq_neg_iff_x W hN hy,double_x_eq_iff_psi W hN hy]
  constructor
  · intro h3
    exact (hconditional (three_point_not_inverse W hN h3)).mp h3
  · intro hx
    exact (hconditional ((denominator_ne_iff W x y).mp (psi_root_denominator_nonzero W hN hx))).mpr hx

end
end ThreeTorsionRankAudit.General
