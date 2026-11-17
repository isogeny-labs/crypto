import Mathlib.Tactic
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

/-!
# The roots of Semaev's `S₃(x_R, x_F, X)` are `x(R + F)` and `x(R − F)`

Companion to `papers/rootfinding-decomposition/rootfinding_decomposition.tex`,
Proposition 1.

Setting: a field `K` and the short-Weierstrass curve `y² = x³ + a x + b`.
For affine points `R = (xR, yR)` and `F = (xF, yF)` on the curve with
`xR ≠ xF`, the chord through `R` and `F` has slope
`λ₊ = (yF − yR)/(xF − xR)` and the chord through `R` and `−F = (xF, −yF)`
has slope `λ₋ = (−yF − yR)/(xF − xR)`; the affine addition law gives
`x(R + F) = λ₊² − xR − xF` and `x(R − F) = λ₋² − xR − xF`.
(These are the chord formulas of the affine group law. The section
"Bridge to Mathlib's group law" at the end of this file proves it:
`addX_eq_mathlib_addX` identifies `addX` with Mathlib's
`WeierstrassCurve.Affine.addX` applied to `WeierstrassCurve.Affine.slope`
when `a₁ = a₂ = 0` and `xR ≠ xF`, and `pointX_add_eq_xPlus` and
`pointX_add_neg_eq_xMinus` identify `xPlus` and `xMinus` with the
`x`-coordinates of Mathlib's `Point` sums `R + F` and `R + (−F)`, which in
addition need `a₃ = 0`.)

Semaev's third summation polynomial for this curve is
`S₃(x₁, x₂, x₃) = (x₁ − x₂)² x₃² − 2[(x₁ + x₂)(x₁x₂ + a) + 2b] x₃
                 + [(x₁x₂ − a)² − 4b(x₁ + x₂)]`.

Main theorem `S3_factor`: as a function of `X`,
`S₃(xR, xF, X) = (xR − xF)² · (X − x(R+F)) · (X − x(R−F))`.
Since `(xR − xF)² ≠ 0`, this says the roots of `X ↦ S₃(xR, xF, X)` are
exactly `x(R+F)` and `x(R−F)`, counted with multiplicity (`S3_root_iff`).
No characteristic assumption is needed for the identity itself.
-/

namespace SemaevS3

variable {K : Type*} [Field K]

/-- Semaev's third summation polynomial for `y² = x³ + a x + b`,
evaluated at `(x₁, x₂, x₃)`. -/
def S3 (a b x₁ x₂ x₃ : K) : K :=
  (x₁ - x₂) ^ 2 * x₃ ^ 2
    - 2 * ((x₁ + x₂) * (x₁ * x₂ + a) + 2 * b) * x₃
    + ((x₁ * x₂ - a) ^ 2 - 4 * b * (x₁ + x₂))

/-- Slope of the chord through `(xR, yR)` and `(xF, yF)` (the affine group-law slope). -/
def slope (xR yR xF yF : K) : K := (yF - yR) / (xF - xR)

/-- `x`-coordinate of the sum of two affine points with distinct `x`
(the affine chord formula `λ² − x₁ − x₂`). -/
def addX (xR yR xF yF : K) : K := slope xR yR xF yF ^ 2 - xR - xF

/-- `x(R + F)`. -/
abbrev xPlus (xR yR xF yF : K) : K := addX xR yR xF yF
/-- `x(R − F)`, i.e. the sum with `−F = (xF, −yF)`. -/
abbrev xMinus (xR yR xF yF : K) : K := addX xR yR xF (-yF)

/-- Clearing the denominator in the chord formula. -/
lemma d2_mul_addX (xR yR xF yF : K) (hx : xF - xR ≠ 0) :
    (xF - xR) ^ 2 * addX xR yR xF yF = (yF - yR) ^ 2 - (xF - xR) ^ 2 * (xR + xF) := by
  unfold addX slope
  rw [div_pow]
  field_simp
  ring

/-- **Proposition 1 (factorization form).** On the curve, with `xR ≠ xF`,
`S₃(xR, xF, X) = (xR − xF)² (X − x(R+F)) (X − x(R−F))` for every `X`. -/
theorem S3_factor (a b xR yR xF yF X : K) (hx : xR ≠ xF)
    (hR : yR ^ 2 = xR ^ 3 + a * xR + b) (hF : yF ^ 2 = xF ^ 3 + a * xF + b) :
    S3 a b xR xF X
      = (xR - xF) ^ 2 * (X - xPlus xR yR xF yF) * (X - xMinus xR yR xF yF) := by
  have hd : xF - xR ≠ 0 := sub_ne_zero.mpr (Ne.symm hx)
  have hd2 : (xF - xR) ^ 2 ≠ 0 := pow_ne_zero 2 hd
  -- Clear the two chord denominators.
  have hp := d2_mul_addX xR yR xF yF hd          -- d² · x(R+F)
  have hm := d2_mul_addX xR yR xF (-yF) hd       -- d² · x(R−F)
  -- Multiply the goal by d² ≠ 0 and reduce to a polynomial identity.
  apply mul_left_cancel₀ hd2
  have expand :
      (xF - xR) ^ 2 * ((xR - xF) ^ 2 * (X - xPlus xR yR xF yF) * (X - xMinus xR yR xF yF))
        = ((xF - xR) ^ 2 * X - (xF - xR) ^ 2 * addX xR yR xF yF)
          * ((xF - xR) ^ 2 * X - (xF - xR) ^ 2 * addX xR yR xF (-yF)) := by
    unfold xPlus xMinus; ring
  rw [expand, hp, hm]
  unfold S3
  -- The remaining identity holds modulo the two curve equations; the
  -- cofactors were computed symbolically (sympy) and are checked by `ring`.
  linear_combination
    (2 * X * xF ^ 2 - 4 * X * xF * xR + 2 * X * xR ^ 2 - a * xR - b + 2 * xF ^ 3
        - 2 * xF ^ 2 * xR - 2 * xF * xR ^ 2 + xR ^ 3 + 2 * yF ^ 2 - yR ^ 2) * hR
    + (2 * X * xF ^ 2 - 4 * X * xF * xR + 2 * X * xR ^ 2 - a * xF + 2 * a * xR + b
        + xF ^ 3 - 2 * xF ^ 2 * xR - 2 * xF * xR ^ 2 + 4 * xR ^ 3 - yF ^ 2) * hF

/-- **Proposition 1 (root form).** On the curve, with `xR ≠ xF`,
`S₃(xR, xF, X) = 0` iff `X = x(R+F)` or `X = x(R−F)`. -/
theorem S3_root_iff (a b xR yR xF yF X : K) (hx : xR ≠ xF)
    (hR : yR ^ 2 = xR ^ 3 + a * xR + b) (hF : yF ^ 2 = xF ^ 3 + a * xF + b) :
    S3 a b xR xF X = 0 ↔ X = xPlus xR yR xF yF ∨ X = xMinus xR yR xF yF := by
  rw [S3_factor a b xR yR xF yF X hx hR hF]
  have hd2 : (xR - xF) ^ 2 ≠ 0 := pow_ne_zero 2 (sub_ne_zero.mpr hx)
  constructor
  · intro h
    rcases mul_eq_zero.mp h with h1 | h2
    · rcases mul_eq_zero.mp h1 with h0 | h1'
      · exact absurd h0 hd2
      · exact Or.inl (sub_eq_zero.mp h1')
    · exact Or.inr (sub_eq_zero.mp h2)
  · rintro (h | h)
    · rw [h]; ring
    · rw [h]; ring

/-! ## Bridge to Mathlib's group law

The definitions above are stated with bare field elements. The lemmas below
show that, for a Mathlib Weierstrass curve `W` with `a₁ = a₂ = 0` (and
`a₃ = 0` where negation enters), they are the `x`-coordinates computed by
Mathlib's affine group law `WeierstrassCurve.Affine.Point.add`. They do not
assume `a₄ = a` or `a₆ = b`; `S3_factor` above is where the curve equation
enters, and `S3_factor_mathlib` restates it for Mathlib points. -/

/-- The affine `x`-coordinate of a Mathlib point, `none` at infinity. -/
def pointX {W : WeierstrassCurve.Affine K} : W.Point → Option K
  | .zero => none
  | .some x _ _ => some x

/-- `addX` is Mathlib's `x`-coordinate formula `W.addX` at Mathlib's chord
slope `W.slope`, for any Weierstrass curve with `a₁ = a₂ = 0`, when
`xR ≠ xF`. No curve equation is needed. -/
theorem addX_eq_mathlib_addX [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha₁ : W.a₁ = 0) (ha₂ : W.a₂ = 0) (xR yR xF yF : K) (hx : xR ≠ xF) :
    addX xR yR xF yF = W.addX xR xF (W.slope xR xF yR yF) := by
  have hs : slope xR yR xF yF = (yR - yF) / (xR - xF) := by
    unfold slope
    rw [← neg_sub yR yF, ← neg_sub xR xF, neg_div_neg_eq]
  rw [WeierstrassCurve.Affine.slope_of_X_ne hx]
  simp only [addX, WeierstrassCurve.Affine.addX, hs, ha₁, ha₂] <;> ring

/-- `xPlus` is the `x`-coordinate of the Mathlib sum `R + F` of two nonsingular
affine points with `xR ≠ xF` on a curve with `a₁ = a₂ = 0`. -/
theorem pointX_add_eq_xPlus [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha₁ : W.a₁ = 0) (ha₂ : W.a₂ = 0) {xR yR xF yF : K}
    (hR : W.Nonsingular xR yR) (hF : W.Nonsingular xF yF) (hx : xR ≠ xF) :
    pointX (WeierstrassCurve.Affine.Point.some xR yR hR
        + WeierstrassCurve.Affine.Point.some xF yF hF) = some (xPlus xR yR xF yF) := by
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne hx]
  show some (W.addX xR xF (W.slope xR xF yR yF)) = some (addX xR yR xF yF)
  rw [addX_eq_mathlib_addX W ha₁ ha₂ xR yR xF yF hx]

/-- `xMinus` is the `x`-coordinate of the Mathlib sum `R + (−F)` of two
nonsingular affine points with `xR ≠ xF` on a curve with `a₁ = a₂ = a₃ = 0`
(`a₃ = 0` makes Mathlib's `−(xF, yF)` equal to `(xF, −yF)`). -/
theorem pointX_add_neg_eq_xMinus [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha₁ : W.a₁ = 0) (ha₂ : W.a₂ = 0) (ha₃ : W.a₃ = 0) {xR yR xF yF : K}
    (hR : W.Nonsingular xR yR) (hF : W.Nonsingular xF yF) (hx : xR ≠ xF) :
    pointX (WeierstrassCurve.Affine.Point.some xR yR hR
        + -WeierstrassCurve.Affine.Point.some xF yF hF) = some (xMinus xR yR xF yF) := by
  rw [WeierstrassCurve.Affine.Point.neg_some, WeierstrassCurve.Affine.Point.add_of_X_ne hx]
  show some (W.addX xR xF (W.slope xR xF yR (W.negY xF yF))) = some (addX xR yR xF (-yF))
  have hneg : W.negY xF yF = -yF := by
    simp only [WeierstrassCurve.Affine.negY, ha₁, ha₃] <;> ring
  rw [hneg, addX_eq_mathlib_addX W ha₁ ha₂ xR yR xF (-yF) hx]

/-- **Proposition 1 for Mathlib points.** For nonsingular affine points `R`, `F`
with `xR ≠ xF` on the Mathlib curve `y² = x³ + a₄ x + a₆` (`a₁ = a₂ = a₃ = 0`),
`S₃(xR, xF, X)` factors with roots the `x`-coordinates of Mathlib's `R + F`
and `R + (−F)`. -/
theorem S3_factor_mathlib [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (ha₁ : W.a₁ = 0) (ha₂ : W.a₂ = 0) (ha₃ : W.a₃ = 0) {xR yR xF yF : K}
    (hR : W.Nonsingular xR yR) (hF : W.Nonsingular xF yF) (hx : xR ≠ xF) (X : K) :
    S3 W.a₄ W.a₆ xR xF X
        = (xR - xF) ^ 2 * (X - xPlus xR yR xF yF) * (X - xMinus xR yR xF yF)
      ∧ pointX (WeierstrassCurve.Affine.Point.some xR yR hR
          + WeierstrassCurve.Affine.Point.some xF yF hF) = some (xPlus xR yR xF yF)
      ∧ pointX (WeierstrassCurve.Affine.Point.some xR yR hR
          + -WeierstrassCurve.Affine.Point.some xF yF hF) = some (xMinus xR yR xF yF) := by
  have hEq : ∀ {x y : K}, W.Equation x y → y ^ 2 = x ^ 3 + W.a₄ * x + W.a₆ := by
    intro x y h
    rw [WeierstrassCurve.Affine.equation_iff, ha₁, ha₂, ha₃] at h
    linear_combination h
  refine ⟨S3_factor W.a₄ W.a₆ xR yR xF yF X hx (hEq hR.1) (hEq hF.1),
    pointX_add_eq_xPlus W ha₁ ha₂ hR hF hx,
    pointX_add_neg_eq_xMinus W ha₁ ha₂ ha₃ hR hF hx⟩

end SemaevS3
