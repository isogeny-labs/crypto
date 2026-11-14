import DivisionQuartic

namespace ThreeTorsionRankAudit
open WeierstrassCurve.Affine
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K] [NeZero (2 : K)]

omit [DecidableEq K] in
theorem ordinate_ne_inverse (a b x y : K) (hy : y ≠ 0) :
    y ≠ (short a b).negY x y := by
  intro he
  simp [short,WeierstrassCurve.Affine.negY] at he
  apply mul_ne_zero (NeZero.ne (2 : K)) hy
  linear_combination he

theorem double_eq_neg_iff_x (a b x y : K) (hN : (short a b).Nonsingular x y)
    (hy : y ≠ 0) :
    Point.some x y hN + Point.some x y hN = -Point.some x y hN ↔
      (short a b).addX x x ((short a b).slope x x y y) = x := by
  have hY := ordinate_ne_inverse a b x y hy
  have hd := (short a b).nonsingular_add hN hN (fun hxy => hY hxy.2)
  rw [Point.add_self_of_Y_ne (h₁ := hN) hY]
  constructor
  · intro he
    exact Point.X_eq_iff.mpr (Or.inr he)
  · intro hx
    rcases (Point.X_eq_iff (h₁ := hd) (h₂ := hN)).mp hx with hh | hh
    · exfalso
      have hPP := (Point.add_self_of_Y_ne (h₁ := hN) hY).trans hh
      apply Point.some_ne_zero hN
      exact add_left_cancel (hPP.trans (add_zero _).symm)
    · exact hh

theorem double_x_formula (a b x y : K) (hN : (short a b).Nonsingular x y)
    (hy : y ≠ 0) :
    (short a b).addX x x ((short a b).slope x x y y) =
      (x^4-2*a*x^2-8*b*x+a^2)/(4*curveValue a b x) := by
  rw [WeierstrassCurve.Affine.addX_self_of_Y_ne hN.1 (ordinate_ne_inverse a b x y hy)]
  simp [short,WeierstrassCurve.b₂,WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  congr 1
  · ring
  · unfold curveValue; ring

theorem double_x_eq_iff_psi (a b x y : K) (hN : (short a b).Nonsingular x y)
    (hy : y ≠ 0) :
    (short a b).addX x x ((short a b).slope x x y y) = x ↔ (psi a b).IsRoot x := by
  have heq := (curve_equation a b x y).mp hN.1
  have hf : curveValue a b x ≠ 0 := by
    intro hz
    exact hy (sq_eq_zero_iff.mp (heq.trans hz))
  have h4 : (4 : K) ≠ 0 := by
    have hid : (4 : K)=2^2 := by norm_num
    rw [hid]
    exact pow_ne_zero _ (NeZero.ne _)
  rw [double_x_formula a b x y hN hy,div_eq_iff (mul_ne_zero h4 hf)]
  change (x^4-2*a*x^2-8*b*x+a^2 = x*(4*curveValue a b x)) ↔ (psi a b).eval x=0
  rw [psi_eval]
  unfold curveValue
  constructor <;> intro h
  · linear_combination -h
  · linear_combination -h

theorem three_point_iff_of_ordinate_ne (a b x y : K) (hN : (short a b).Nonsingular x y)
    (hy : y ≠ 0) : (3 : ℕ) • Point.some x y hN = 0 ↔ (psi a b).IsRoot x := by
  rw [show (3 : ℕ)=2+1 by rfl,add_nsmul,two_nsmul,one_nsmul,
    add_eq_zero_iff_eq_neg,double_eq_neg_iff_x a b x y hN hy,
    double_x_eq_iff_psi a b x y hN hy]

omit [NeZero (2 : K)] in
theorem three_ordinate_nonzero (a b x y : K) (hN : (short a b).Nonsingular x y)
    (h3 : (3 : ℕ) • Point.some x y hN = 0) : y ≠ 0 := by
  intro hy
  subst y
  have htwo : Point.some x 0 hN + Point.some x 0 hN = 0 :=
    Point.add_self_of_Y_eq (by simp [short,WeierstrassCurve.Affine.negY])
  rw [show (3 : ℕ)=2+1 by rfl,add_nsmul,two_nsmul,one_nsmul,htwo,zero_add] at h3
  exact Point.some_ne_zero hN h3

theorem three_point_iff (a b x y : K) (hΔ : (short a b).Δ ≠ 0)
    (hN : (short a b).Nonsingular x y) :
    (3 : ℕ) • Point.some x y hN = 0 ↔ (psi a b).IsRoot x := by
  constructor
  · intro h3
    exact (three_point_iff_of_ordinate_ne a b x y hN (three_ordinate_nonzero a b x y hN h3)).mp h3
  · intro hx
    have hf := psi_root_curve_value_nonzero a b hΔ hx
    have hy : y ≠ 0 := by
      intro hz
      apply hf
      have he := (curve_equation a b x y).mp hN.1
      simpa [hz] using he.symm
    exact (three_point_iff_of_ordinate_ne a b x y hN hy).mpr hx

end
end ThreeTorsionRankAudit
