module
public import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
public import Mathlib.Tactic

@[expose] public section
namespace MillerPrincipalIdealAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing

theorem cancel_principal {A : Type*} [CommRing A] [IsDomain A]
    (a : A) (ha : a ≠ 0) (I J : Ideal A)
    (h : Ideal.span {a} * I = Ideal.span {a} * J) : I = J := by
  ext x
  constructor
  · intro hx
    have hm : a * x ∈ Ideal.span {a} * I :=
      Ideal.mul_mem_mul (Ideal.subset_span (by simp)) hx
    rw [h] at hm
    obtain ⟨y, hy, he⟩ := Ideal.mem_span_singleton_mul.mp hm
    have hyx : y = x := mul_left_cancel₀ ha he
    simpa only [hyx] using hy
  · intro hx
    have hm : a * x ∈ Ideal.span {a} * J :=
      Ideal.mul_mem_mul (Ideal.subset_span (by simp)) hx
    rw [← h] at hm
    obtain ⟨y, hy, he⟩ := Ideal.mem_span_singleton_mul.mp hm
    have hyx : y = x := mul_left_cancel₀ ha he
    simpa only [hyx] using hy

variable {K : Type*} [Field K] [DecidableEq K]

def pointIdeal (W : WeierstrassCurve.Affine K) (x y : K) : Ideal W.CoordinateRing :=
  XYIdeal W x (C y)

def lineClass (W : WeierstrassCurve.Affine K) (x y slope : K) : W.CoordinateRing :=
  YClass W (linePolynomial x y slope)

def lineIdeal (W : WeierstrassCurve.Affine K) (x y slope : K) : Ideal W.CoordinateRing :=
  Ideal.span {lineClass W x y slope}

/-- Actual affine principal ideal of the chord/tangent, including multiplicities. -/
theorem chord_principal_ideal (W : WeierstrassCurve.Affine K)
    {x₁ x₂ y₁ y₂ : K} (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (hxy : ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂)) :
    pointIdeal W x₁ y₁ * pointIdeal W x₂ y₂ *
      pointIdeal W (W.addX x₁ x₂ (W.slope x₁ x₂ y₁ y₂))
        (W.negY (W.addX x₁ x₂ (W.slope x₁ x₂ y₁ y₂))
          (W.addY x₁ x₂ y₁ (W.slope x₁ x₂ y₁ y₂))) =
      lineIdeal W x₁ y₁ (W.slope x₁ x₂ y₁ y₂) := by
  let slope := W.slope x₁ x₂ y₁ y₂
  let xs := W.addX x₁ x₂ slope
  let ys := W.addY x₁ x₂ y₁ slope
  let Ineg := pointIdeal W xs (W.negY xs ys)
  have hs : W.Nonsingular xs ys := W.nonsingular_add h₁ h₂ hxy
  have hp := XYIdeal_mul_XYIdeal h₁.1 h₂.1 hxy
  change XIdeal W xs * (pointIdeal W x₁ y₁ * pointIdeal W x₂ y₂) =
    lineIdeal W x₁ y₁ slope * pointIdeal W xs ys at hp
  have hn := XYIdeal_neg_mul hs
  change Ineg * pointIdeal W xs ys = XIdeal W xs at hn
  apply cancel_principal (XClass W xs) (XClass_ne_zero xs)
  change XIdeal W xs * (pointIdeal W x₁ y₁ * pointIdeal W x₂ y₂ * Ineg) =
    XIdeal W xs * lineIdeal W x₁ y₁ slope
  calc
    _ = (XIdeal W xs * (pointIdeal W x₁ y₁ * pointIdeal W x₂ y₂)) * Ineg := by ac_rfl
    _ = (lineIdeal W x₁ y₁ slope * pointIdeal W xs ys) * Ineg :=
      congrArg (fun I : Ideal W.CoordinateRing => I * Ineg) hp
    _ = (Ineg * pointIdeal W xs ys) * lineIdeal W x₁ y₁ slope := by ac_rfl
    _ = _ := by rw [hn]

/-- Actual three-torsion derives the nonvertical tangent guard. -/
theorem three_tangent_nonvertical (W : WeierstrassCurve.Affine K)
    {x y : K} (hN : W.Nonsingular x y)
    (h3 : (3 : ℕ) • Point.some x y hN = 0) : y ≠ W.negY x y := by
  intro he
  have h2 := Point.add_self_of_Y_eq (h₁ := hN) he
  rw [show (3 : ℕ) = 2 + 1 by decide, add_nsmul, two_nsmul, one_nsmul,
    h2, zero_add] at h3
  exact Point.some_ne_zero hN h3

theorem tangent_three_principal_ideal (W : WeierstrassCurve.Affine K)
    {x y : K} (hN : W.Nonsingular x y)
    (h3 : (3 : ℕ) • Point.some x y hN = 0) :
    pointIdeal W x y ^ 3 = lineIdeal W x y (W.slope x x y y) := by
  have hy := three_tangent_nonvertical W hN h3
  have hd : Point.some x y hN + Point.some x y hN = -Point.some x y hN := by
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [show (3 : ℕ) = 2 + 1 by decide, add_nsmul, two_nsmul, one_nsmul] using h3
  rw [Point.add_self_of_Y_ne hy, Point.neg_some] at hd
  obtain ⟨hx, hy'⟩ := Point.some.inj hd
  have hn : W.negY x (W.negY x y) = y := by
    unfold WeierstrassCurve.Affine.negY
    ring
  have hp := chord_principal_ideal W hN hN (fun h => hy h.2)
  rw [hx, hy', hn] at hp
  simpa only [pow_succ, pow_zero, one_mul] using hp

omit [DecidableEq K] in
theorem line_class_nonzero (W : WeierstrassCurve.Affine K) (x y slope : K) :
    lineClass W x y slope ≠ 0 := YClass_ne_zero _

def lineFunction (W : WeierstrassCurve.Affine K) (x y slope : K) : W.FunctionField :=
  algebraMap W.CoordinateRing W.FunctionField (lineClass W x y slope)

omit [DecidableEq K] in
theorem line_function_nonzero (W : WeierstrassCurve.Affine K) (x y slope : K) :
    lineFunction W x y slope ≠ 0 := by
  intro h
  apply line_class_nonzero W x y slope
  apply IsFractionRing.injective W.CoordinateRing W.FunctionField
  simpa only [lineFunction, map_zero] using h

#print axioms chord_principal_ideal
#print axioms tangent_three_principal_ideal
#print axioms line_function_nonzero
end
end MillerPrincipalIdealAudit
