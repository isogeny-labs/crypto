import Mathlib

/-! Layer 1a: closure of the short-Weierstrass chord–tangent law.

This is the first curve-specific lemma in the ECDLP hardness ledger. It says
that the affine addition and doubling formulas return a point on the curve,
which is what makes the chord–tangent operation a candidate group law. -/

namespace ECDLPClosure

variable {K : Type*} [Field K]

/-- Affine short-Weierstrass curve `y² = x³ + a x + b`. -/
def OnCurve (a b : K) (P : K × K) : Prop := P.2 ^ 2 = P.1 ^ 3 + a * P.1 + b

/-- Chord slope through two points with distinct x-coordinates. -/
noncomputable def slope (P Q : K × K) : K := (Q.2 - P.2) / (Q.1 - P.1)

/-- x-coordinate of the chord sum. -/
noncomputable def addX (P Q : K × K) : K := slope P Q ^ 2 - P.1 - Q.1

/-- y-coordinate of the chord sum (the reflection of the third intersection). -/
noncomputable def addY (P Q : K × K) : K := slope P Q * (P.1 - addX P Q) - P.2

/-- Doubling slope. -/
noncomputable def dblSlope (a : K) (P : K × K) : K :=
  (3 * P.1 ^ 2 + a) / (2 * P.2)

/-- x-coordinate of the double. -/
noncomputable def dblX (a : K) (P : K × K) : K := dblSlope a P ^ 2 - 2 * P.1

/-- y-coordinate of the double. -/
noncomputable def dblY (a : K) (P : K × K) : K :=
  dblSlope a P * (P.1 - dblX a P) - P.2

/-- Chord addition stays on the curve. -/
theorem add_on_curve (a b : K) (P Q : K × K)
    (hP : OnCurve a b P) (hQ : OnCurve a b Q) (hx : Q.1 ≠ P.1) :
    OnCurve a b (addX P Q, addY P Q) := by
  have hx' : Q.1 - P.1 ≠ 0 := sub_ne_zero.mpr hx
  unfold OnCurve at hP hQ
  have hline : slope P Q * (Q.1 - P.1) = Q.2 - P.2 := by
    unfold slope
    rw [div_mul_cancel₀ _ hx']
  have hsum : slope P Q * (Q.2 + P.2) = Q.1 ^ 2 + Q.1 * P.1 + P.1 ^ 2 + a := by
    unfold slope
    rw [div_mul_eq_mul_div, div_eq_iff hx']
    linear_combination hQ - hP
  simp only [OnCurve, addX, addY] at *
  linear_combination -hP + 2*hQ
    + (slope P Q^3 - 2*slope P Q*P.1 - slope P Q*Q.1 + 2*P.2 + 2*Q.2) * hline
    + (slope P Q^2 - 3*Q.1) * hsum

/-- Doubling stays on the curve. -/
theorem dbl_on_curve (a b : K) (P : K × K)
    (hP : OnCurve a b P) (h2 : (2 : K) ≠ 0) (hy : P.2 ≠ 0) :
    OnCurve a b (dblX a P, dblY a P) := by
  unfold OnCurve at hP
  have hy' : (2 : K) * P.2 ≠ 0 := mul_ne_zero h2 hy
  have hslope : dblSlope a P * (2 * P.2) = 3 * P.1 ^ 2 + a := by
    unfold dblSlope
    rw [div_mul_cancel₀ _ hy']
  simp only [OnCurve, dblX, dblY] at *
  linear_combination hP + (dblSlope a P^2 - 3*P.1) * hslope

/-- The chord sum is commutative (x-coordinate). -/
theorem addX_comm (P Q : K × K) : addX P Q = addX Q P := by
  unfold addX slope
  have h : (Q.2 - P.2) / (Q.1 - P.1) = (P.2 - Q.2) / (P.1 - Q.1) := by
    rw [← neg_sub (P.2) (Q.2), ← neg_sub (P.1) (Q.1), neg_div_neg_eq]
  rw [h]
  ring

end ECDLPClosure
