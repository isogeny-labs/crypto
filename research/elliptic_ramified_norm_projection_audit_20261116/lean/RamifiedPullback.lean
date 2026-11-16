import RamifiedBasePlace

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- An actual polynomial cofactor with nonzero evaluation has actual local valuation1. -/
theorem elliptic_polynomial_finite_unit (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (p : K[X]) (hp : p.eval x≠0) :
    localFunctionValuation W (Point.some x y h) (polynomialFunction W p)=1 := by
  obtain ⟨r,hr,hv⟩:=finite_fraction_evaluation W h (mk W (C p)) 1
    (by rw [map_one];exact one_ne_zero)
  have he : regularEvaluation W (Point.some x y h) r≠0 := by
    rw [hv,map_one,div_one,point_eval_polynomial]
    exact hp
  have hu:=(regular_evaluation_value_iff W (Point.some x y h) r).mp he
  rw [hr,map_one,div_one] at hu
  exact hu

theorem polynomial_vertical_function (W : WeierstrassCurve.Affine K) (x : K) :
    polynomialFunction W (X-C x)=verticalFunction W x := by
  rw [map_sub,polynomial_x_function,polynomial_function_constant,vertical_formula]

/-- Full actual polynomial local valuation from actual root multiplicity and proper cofactor. -/
theorem elliptic_polynomial_finite_value (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (p : K[X]) (hp : p≠0) :
    localFunctionValuation W (Point.some x y h) (polynomialFunction W p)=
      (localFunctionValuation W (Point.some x y h) (verticalFunction W x))^(p.rootMultiplicity x) := by
  obtain ⟨q,he,hq⟩:=p.exists_eq_pow_rootMultiplicity_mul_and_not_dvd hp x
  have hvq:=elliptic_polynomial_finite_unit W h q
    (by simpa only [dvd_iff_isRoot,IsRoot.def] using hq)
  conv_lhs => rw [he]
  rw [map_mul,map_pow,map_mul,map_pow,hvq,mul_one,polynomial_vertical_function]

theorem fixed_vertical_value (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    localFunctionValuation W (Point.some x y h) (verticalFunction W x)=WithZero.exp (-2 : ℤ) := by
  have hn:=(baseFieldUnit W (baseVerticalUnit x)).ne_zero
  change algebraMap (RatFunc K) W.FunctionField (baseVerticalUnit x : RatFunc K)≠0 at hn
  rw [base_vertical_unit_image] at hn
  have hv : localFunctionValuation W (Point.some x y h) (verticalFunction W x)≠0 :=
    (Valuation.ne_zero_iff _).mpr hn
  have ho:=fixed_base_parameter_order W h hP
  rw [point_base_order_embedding,base_vertical_unit_image,point_order_formula] at ho
  have hl : WithZero.log (localFunctionValuation W (Point.some x y h) (verticalFunction W x))= -2 := by omega
  rw [←WithZero.exp_log hv,hl]

theorem fixed_base_valuation_polynomial (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (p : K[X]) :
    localFunctionValuation W (Point.some x y h)
      (algebraMap (RatFunc K) W.FunctionField (algebraMap K[X] (RatFunc K) p))=
      (finiteBaseValuation x (algebraMap K[X] (RatFunc K) p))^2 := by
  by_cases hp : p=0
  · simp only [hp,map_zero,zero_pow (by decide : 2≠0)]
  rw [rational_base_polynomial,elliptic_polynomial_finite_value W h p hp,
    fixed_vertical_value W h hP,finite_base_polynomial_value x p hp]
  simp only [←WithZero.exp_nsmul,nsmul_eq_mul]
  congr 1
  ring

/-- Actual factor2 on every rational base function, not inferred from a single parameter. -/
theorem fixed_base_valuation (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (c : RatFunc K) :
    localFunctionValuation W (Point.some x y h) (algebraMap (RatFunc K) W.FunctionField c)=
      (finiteBaseValuation x c)^2 := by
  obtain ⟨p,q,_hq,hpq⟩:=IsFractionRing.div_surjective K[X] c
  rw [←hpq]
  simp only [map_div₀,fixed_base_valuation_polynomial W h hP,div_pow]

theorem fixed_base_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (c : RatFunc K) :
    pointOrder W (algebraMap (RatFunc K) W.FunctionField c) (Point.some x y h)=2*finiteBaseOrder x c := by
  rw [point_order_formula,fixed_base_valuation W h hP,WithZero.log_pow]
  simp only [finiteBaseOrder]
  ring

theorem fixed_norm_base_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (f : W.FunctionFieldˣ) :
    finiteBaseOrder x (normUnit W f : RatFunc K)=pointOrder W f (Point.some x y h) := by
  have hn:=actual_point_base_norm_order W (Point.some x y h) f
  have hb : pointBaseOrder W (Point.some x y h) (normUnit W f : RatFunc K)=
      2*finiteBaseOrder x (normUnit W f : RatFunc K) := by
    rw [point_base_order_embedding]
    exact fixed_base_order W h hP _
  rw [hb,←hP] at hn
  omega

#print axioms elliptic_polynomial_finite_unit
#print axioms fixed_base_valuation
#print axioms fixed_norm_base_order
end
end EllipticRamifiedNormProjectionAudit
