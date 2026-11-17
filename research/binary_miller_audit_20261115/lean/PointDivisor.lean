module
public import LineMultiplicity
public import Mathlib.Data.Finsupp.Basic

@[expose] public section
namespace MillerFinitePointAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerInfinityChartAudit
variable {K : Type*} [Field K]

/-- Actual normalized local order at every nonsingular point, including the actual infinity chart.
Over an algebraically closed field and an elliptic curve these are all geometric points.
No identification with a separate Mathlib projective Scheme.localRing is asserted. -/
def pointOrder (W : WeierstrassCurve.Affine K) (f : W.FunctionField) : W.Point → ℤ
  | .zero => -WithZero.log (chartAdicValuation W f)
  | .some _ _ h => -WithZero.log (finiteAdic W h f)

/-- Divisor defined from actual local orders and an explicit proof of their finite support. -/
def principalPointDivisor (W : WeierstrassCurve.Affine K) (f : W.FunctionField)
    (hf : (Function.support (pointOrder W f)).Finite) : W.Point →₀ ℤ :=
  Finsupp.ofSupportFinite (pointOrder W f) hf

variable [DecidableEq K]

def chordDivisor (W : WeierstrassCurve.Affine K) (P Q : W.Point) : W.Point →₀ ℤ :=
  Finsupp.single P 1 + Finsupp.single Q 1 + Finsupp.single (-(P + Q)) 1 - Finsupp.single 0 3

def verticalDivisor (W : WeierstrassCurve.Affine K) (P : W.Point) : W.Point →₀ ℤ :=
  Finsupp.single P 1 + Finsupp.single (-P) 1 - Finsupp.single 0 2

theorem coordinate_multiplicity_single (W : WeierstrassCurve.Affine K) {xr yr x y : K}
    (hR : W.Nonsingular xr yr) (h : W.Nonsingular x y) :
    (coordinateMultiplicity xr yr x y : ℤ) =
      Finsupp.single (Point.some x y h) (1 : ℤ) (Point.some xr yr hR) := by
  classical
  simp only [coordinateMultiplicity, Finsupp.single_apply, Point.some.injEq]
  by_cases hx : xr = x <;> by_cases hy : yr = y <;> simp_all [eq_comm]

theorem chord_point_order (W : WeierstrassCurve.Affine K) {x₁ y₁ x₂ y₂ : K}
    (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (hxy : ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂)) (R : W.Point) :
    pointOrder W (MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂)) R =
      chordDivisor W (Point.some x₁ y₁ h₁) (Point.some x₂ y₂ h₂) R := by
  classical
  cases R with
  | zero =>
    simp only [pointOrder, chart_adic_line, WithZero.log_exp, chordDivisor,
      Point.add_some hxy, Point.neg_some, Finsupp.sub_apply, Finsupp.add_apply]
    simp [← Point.zero_def]
  | some xr yr hR =>
    rw [pointOrder, finite_chord_order W hR h₁ h₂ hxy]
    have hS := W.nonsingular_add h₁ h₂ hxy
    have hN := (W.nonsingular_neg _ _).mpr hS
    simp only [chordMultiplicity, Nat.cast_add, coordinate_multiplicity_single W hR h₁,
      coordinate_multiplicity_single W hR h₂, coordinate_multiplicity_single W hR hN,
      chordDivisor, Point.add_some hxy, Point.neg_some,
      Finsupp.sub_apply, Finsupp.add_apply]
    simp

theorem chord_support_finite (W : WeierstrassCurve.Affine K) {x₁ y₁ x₂ y₂ : K}
    (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (hxy : ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂)) :
    (Function.support (pointOrder W
      (MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂)))).Finite := by
  have he : pointOrder W
      (MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂)) =
      (chordDivisor W (Point.some x₁ y₁ h₁) (Point.some x₂ y₂ h₂) : W.Point → ℤ) :=
    funext (chord_point_order W h₁ h₂ hxy)
  rw [he]
  exact Finsupp.hasFiniteSupport _

/-- The actual chord/tangent principal point divisor, derived from the actual local valuations. -/
theorem chord_principal_point_divisor (W : WeierstrassCurve.Affine K) {x₁ y₁ x₂ y₂ : K}
    (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (hxy : ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂)) :
    principalPointDivisor W
      (MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂))
      (chord_support_finite W h₁ h₂ hxy) =
      chordDivisor W (Point.some x₁ y₁ h₁) (Point.some x₂ y₂ h₂) := by
  ext R
  exact chord_point_order W h₁ h₂ hxy R

theorem vertical_point_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (R : W.Point) :
    pointOrder W (MillerNormValuationAudit.verticalFunction W x) R =
      verticalDivisor W (Point.some x y h) R := by
  classical
  cases R with
  | zero =>
    simp only [pointOrder, chart_adic_vertical, WithZero.log_exp, verticalDivisor,
      Point.neg_some, Finsupp.sub_apply, Finsupp.add_apply]
    simp [← Point.zero_def]
  | some xr yr hR =>
    rw [pointOrder, finite_vertical_order W hR h]
    have hN := (W.nonsingular_neg x y).mpr h
    simp only [Nat.cast_add, coordinate_multiplicity_single W hR h,
      coordinate_multiplicity_single W hR hN, verticalDivisor, Point.neg_some,
      Finsupp.sub_apply, Finsupp.add_apply]
    simp [Point.some_ne_zero, add_comm]

theorem vertical_support_finite (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    (Function.support (pointOrder W (MillerNormValuationAudit.verticalFunction W x))).Finite := by
  have he : pointOrder W (MillerNormValuationAudit.verticalFunction W x) =
      (verticalDivisor W (Point.some x y h) : W.Point → ℤ) := funext (vertical_point_order W h)
  rw [he]
  exact Finsupp.hasFiniteSupport _

/-- The vertical principal divisor, including the repeated two-torsion case. -/
theorem vertical_principal_point_divisor (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    principalPointDivisor W (MillerNormValuationAudit.verticalFunction W x)
      (vertical_support_finite W h) = verticalDivisor W (Point.some x y h) := by
  ext R
  exact vertical_point_order W h R

/-- Actual three-torsion tangent has divisor 3(P)-3(O), not just an affine ideal cube. -/
theorem three_tangent_principal_point_divisor (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (h3 : (3 : ℕ) • Point.some x y h = 0) :
    principalPointDivisor W (MillerNormValuationAudit.lineFunction W x y (W.slope x x y y))
      (chord_support_finite W h h (fun hn => three_tangent_nonvertical W h h3 hn.2)) =
      Finsupp.single (Point.some x y h) 3 - Finsupp.single 0 3 := by
  have hd : Point.some x y h + Point.some x y h = -Point.some x y h := by
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [show (3 : ℕ) = 2 + 1 by decide, add_nsmul, two_nsmul, one_nsmul] using h3
  rw [chord_principal_point_divisor W h h
    (fun hn => three_tangent_nonvertical W h h3 hn.2), chordDivisor, hd, neg_neg,
    ← Finsupp.single_add, ← Finsupp.single_add]
  norm_num

#print axioms chord_principal_point_divisor
#print axioms vertical_principal_point_divisor
#print axioms three_tangent_principal_point_divisor
end
end MillerFinitePointAudit
