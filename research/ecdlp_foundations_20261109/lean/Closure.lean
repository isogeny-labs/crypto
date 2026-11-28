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

/-! ## Bridge to Mathlib's elliptic-curve group law

Mathlib proves that nonsingular affine points of a Weierstrass curve form an
`AddCommGroup`. These lemmas identify our explicit chord–tangent formulas with
Mathlib's `addX`/`addY`, so associativity of the explicit law is inherited
from Mathlib's group structure instead of being reproved by brute algebra. -/

namespace ECDLPClosure

variable {K : Type*} [Field K]
variable [DecidableEq K]

open WeierstrassCurve

/-- The short Weierstrass curve `y² = x³ + a x + b` as a Mathlib curve. -/
def curve (a b : K) : WeierstrassCurve K := ⟨0, 0, 0, a, b⟩

/-- Mathlib's slope for distinct x-coordinates is our chord slope. -/
theorem curve_slope (a b x₁ x₂ y₁ y₂ : K) (hx : x₁ ≠ x₂) :
    WeierstrassCurve.Affine.slope (curve a b) x₁ x₂ y₁ y₂ = (y₂ - y₁) / (x₂ - x₁) := by
  rw [WeierstrassCurve.Affine.slope_of_X_ne (W := curve a b) hx]
  have hx' : x₂ - x₁ ≠ 0 := sub_ne_zero.mpr (Ne.symm hx)
  rw [div_eq_div_iff (sub_ne_zero.mpr hx) hx']
  ring

/-- Mathlib's `addX` agrees with our explicit `addX`. -/
theorem curve_addX (a b x₁ x₂ y₁ y₂ : K) (hx : x₁ ≠ x₂) :
    WeierstrassCurve.Affine.addX (curve a b) x₁ x₂ (WeierstrassCurve.Affine.slope (curve a b) x₁ x₂ y₁ y₂) =
      ((y₂ - y₁) / (x₂ - x₁)) ^ 2 - x₁ - x₂ := by
  rw [curve_slope a b x₁ x₂ y₁ y₂ hx]
  simp [WeierstrassCurve.Affine.addX, curve]

/-- Mathlib's `addY` agrees with our explicit `addY`. -/
theorem curve_addY (a b x₁ x₂ y₁ y₂ : K) (hx : x₁ ≠ x₂) :
    WeierstrassCurve.Affine.addY (curve a b) x₁ x₂ y₁ (WeierstrassCurve.Affine.slope (curve a b) x₁ x₂ y₁ y₂) =
      ((y₂ - y₁) / (x₂ - x₁)) * (x₁ - (((y₂ - y₁) / (x₂ - x₁)) ^ 2 - x₁ - x₂)) - y₁ := by
  rw [curve_slope a b x₁ x₂ y₁ y₂ hx]
  simp [WeierstrassCurve.Affine.addY, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, WeierstrassCurve.Affine.addX, curve]
  ring

/-- Associativity of Mathlib's affine group law (the theorem our explicit law
inherits through `curve_addX`/`curve_addY`). -/
theorem curve_add_assoc (a b : K) (P Q R : WeierstrassCurve.Affine.Point (curve a b)) :
    P + Q + R = P + (Q + R) :=
  add_assoc P Q R

end ECDLPClosure

/-! ## Exceptional cases

The generic bridge above covers `x₁ ≠ x₂`. The remaining cases are the
identity, opposite points and doubling. Identity is Mathlib's `zero_add`/
`add_zero`; opposite points use `add_of_Y_eq`; here we add the coordinate
bridge for doubling. -/

namespace ECDLPClosure

variable {K : Type*} [Field K] [DecidableEq K]

/-- Mathlib's slope at a point with itself is our doubling slope (short form). -/
theorem curve_slope_self (a b x y : K) (hy : y ≠ -y) :
    WeierstrassCurve.Affine.slope (curve a b) x x y y =
      (3 * x ^ 2 + a) / (2 * y) := by
  have hy' : y ≠ WeierstrassCurve.Affine.negY (curve a b) x y := by
    simpa [curve, WeierstrassCurve.Affine.negY] using hy
  rw [WeierstrassCurve.Affine.slope_of_Y_ne rfl hy']
  simp [curve, WeierstrassCurve.Affine.negY]
  ring

/-- Mathlib's `addX` for a doubled point is our `dblX`. -/
theorem curve_dblX (a b x y : K) (hy : y ≠ -y) :
    WeierstrassCurve.Affine.addX (curve a b) x x
        (WeierstrassCurve.Affine.slope (curve a b) x x y y) =
      ((3 * x ^ 2 + a) / (2 * y)) ^ 2 - 2 * x := by
  rw [curve_slope_self a b x y hy]
  simp [WeierstrassCurve.Affine.addX, curve]
  ring

/-- Mathlib's `addY` for a doubled point is our `dblY`. -/
theorem curve_dblY (a b x y : K) (hy : y ≠ -y) :
    WeierstrassCurve.Affine.addY (curve a b) x x y
        (WeierstrassCurve.Affine.slope (curve a b) x x y y) =
      ((3 * x ^ 2 + a) / (2 * y)) * (x - (((3 * x ^ 2 + a) / (2 * y)) ^ 2 - 2 * x)) - y := by
  rw [curve_slope_self a b x y hy]
  simp [WeierstrassCurve.Affine.addY, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, WeierstrassCurve.Affine.addX, curve]
  ring

/-- Opposite affine points on the short curve sum to the identity. -/
theorem curve_add_neg {a b x y : K} {h₁ : WeierstrassCurve.Affine.Nonsingular (curve a b) x y}
    {h₂ : WeierstrassCurve.Affine.Nonsingular (curve a b) x (WeierstrassCurve.Affine.negY (curve a b) x y)} :
    WeierstrassCurve.Affine.Point.some x y h₁ +
      WeierstrassCurve.Affine.Point.some x
        (WeierstrassCurve.Affine.negY (curve a b) x y) h₂ = 0 := by
  apply WeierstrassCurve.Affine.Point.add_of_Y_eq (x₁ := x) (x₂ := x)
    (y₁ := y) (y₂ := WeierstrassCurve.Affine.negY (curve a b) x y) rfl
  simp [curve, WeierstrassCurve.Affine.negY]

end ECDLPClosure
