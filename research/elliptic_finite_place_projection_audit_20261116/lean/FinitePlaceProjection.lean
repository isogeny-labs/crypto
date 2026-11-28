import FinitePlaceSymbol

namespace EllipticFinitePlaceProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem finite_place_projection_fixed (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    finitePlaceTameSymbol W h (normUnit W f) c=tameSymbol W (Point.some x y h) f (baseFieldUnit W c) := by
  rw [finite_place_tame_fixed W h hP]
  exact actual_ramified_norm_projection W h hP f c

theorem finite_place_projection_split (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    finitePlaceTameSymbol W h (normUnit W f) c=
      tameSymbol W (Point.some x y h) f (baseFieldUnit W c)*tameSymbol W (-Point.some x y h) f (baseFieldUnit W c) := by
  rw [finite_place_tame_split_embedding W h hP,base_field_norm_unit]
  exact actual_rational_norm_tame_orbit W (Point.some x y h) f c

/-- Actual normalized projection over the distinct full fiber, with no external split/fixed guard. -/
theorem actual_finite_place_norm_projection (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    finitePlaceTameSymbol W h (normUnit W f) c=
      ∏ R ∈ finiteFiber W (Point.some x y h),tameSymbol W R f (baseFieldUnit W c) := by
  classical
  by_cases hP : (Point.some x y h : W.Point)= -Point.some x y h
  · rw [finite_fiber_prod_fixed W (Point.some x y h) hP]
    exact finite_place_projection_fixed W h hP f c
  · rw [finite_fiber_prod_split W (Point.some x y h) hP]
    exact finite_place_projection_split W h hP f c

theorem actual_finite_place_inverse_projection (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    (finitePlaceTameSymbol W h (normUnit W f) c)⁻¹=
      ∏ R ∈ finiteFiber W (Point.some x y h),milnorSymbol W R f (baseFieldUnit W c) := by
  rw [actual_finite_place_norm_projection,←Finset.prod_inv_distrib]
  rfl

#print axioms finite_place_projection_fixed
#print axioms finite_place_projection_split
#print axioms actual_finite_place_norm_projection
#print axioms actual_finite_place_inverse_projection
end
end EllipticFinitePlaceProjectionAudit
