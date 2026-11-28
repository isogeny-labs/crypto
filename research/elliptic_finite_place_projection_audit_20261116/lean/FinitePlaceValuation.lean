import RamifiedCertificate
import FinitePlaceImports

namespace EllipticFinitePlaceProjectionAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

def finitePointIndex (W : WeierstrassCurve.Affine K) (P : W.Point) : ℕ := by
  classical
  exact if P= -P then 2 else 1

theorem finite_point_index_pos (W : WeierstrassCurve.Affine K) (P : W.Point) : 0<finitePointIndex W P := by
  classical
  unfold finitePointIndex
  split_ifs <;> norm_num

theorem split_official_vertical_value (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) :
    localFunctionValuation W (Point.some x y h) (verticalFunction W x)=WithZero.exp (-1 : ℤ) := by
  have hv:=split_base_parameter_value W h hP
  change localFunctionValuation W (Point.some x y h)
    (algebraMap (RatFunc K) W.FunctionField (baseVerticalUnit x : RatFunc K))=WithZero.exp (-1 : ℤ) at hv
  rw [base_vertical_unit_image] at hv
  exact hv

theorem split_official_polynomial_value (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) (p : K[X]) :
    localFunctionValuation W (Point.some x y h)
      (algebraMap (RatFunc K) W.FunctionField (algebraMap K[X] (RatFunc K) p))=
      finiteBaseValuation x (algebraMap K[X] (RatFunc K) p) := by
  by_cases hp : p=0
  · simp only [hp,map_zero]
  rw [rational_base_polynomial,elliptic_polynomial_finite_value W h p hp,
    split_official_vertical_value W h hP,finite_base_polynomial_value x p hp]
  rw [←WithZero.exp_nsmul]
  congr 1
  simp

/-- Allfunction split-to-official equality, derived using actual cofactors, not one parameter. -/
theorem split_official_base_valuation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) (c : RatFunc K) :
    localFunctionValuation W (Point.some x y h) (algebraMap (RatFunc K) W.FunctionField c)=finiteBaseValuation x c := by
  obtain ⟨p,q,_hq,hpq⟩:=IsFractionRing.div_surjective K[X] c
  rw [←hpq]
  simp only [map_div₀,split_official_polynomial_value W h hP]

theorem split_restriction_official (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) :
    pointBaseValuation W (Point.some x y h)=finiteBaseValuation x := by
  ext c
  exact split_official_base_valuation W h hP c

/-- The explicit point index is an actual allfunction valuation pullback exponent. -/
theorem actual_finite_place_valuation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (c : RatFunc K) :
    localFunctionValuation W (Point.some x y h) (algebraMap (RatFunc K) W.FunctionField c)=
      (finiteBaseValuation x c)^(finitePointIndex W (Point.some x y h)) := by
  classical
  by_cases hP : (Point.some x y h : W.Point)= -Point.some x y h
  · rw [finitePointIndex,ite_eq_left hP]
    exact fixed_base_valuation W h hP c
  · rw [finitePointIndex,ite_eq_right hP,pow_one]
    exact split_official_base_valuation W h hP c

theorem actual_finite_place_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (c : RatFunc K) :
    pointOrder W (algebraMap (RatFunc K) W.FunctionField c) (Point.some x y h)=
      (finitePointIndex W (Point.some x y h) : ℤ)*finiteBaseOrder x c := by
  rw [point_order_formula,actual_finite_place_valuation W h,WithZero.log_pow]
  simp only [finiteBaseOrder]
  ring

theorem split_restricted_order_official (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) (c : RatFunc K) :
    pointBaseOrder W (Point.some x y h) c=finiteBaseOrder x c := by
  rw [pointBaseOrder,split_restriction_official W h hP]
  rfl

#print axioms split_official_base_valuation
#print axioms split_restriction_official
#print axioms actual_finite_place_valuation
#print axioms actual_finite_place_order
end
end EllipticFinitePlaceProjectionAudit
