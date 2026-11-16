import SplitBase
import SplitImports

namespace EllipticSplitNormProjectionAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

def xFiber (W : WeierstrassCurve.Affine K) (P : W.Point) : Set W.Point :=
  {R | R.xRep=P.xRep}

/-- Full actual point fiber, not a supplied population count. -/
theorem actual_x_fiber (W : WeierstrassCurve.Affine K) (P : W.Point) :
    xFiber W P={P,-P} := by
  ext R
  simpa only [xFiber,Set.mem_ofPred_eq,Set.mem_insert_iff,Set.mem_singleton_iff] using
    (Point.xRep_eq_xRep_iff (P:=R) (Q:=P))

theorem actual_split_fiber_card (W : WeierstrassCurve.Affine K) (P : W.Point) (hP : P≠ -P) :
    (xFiber W P).ncard=2 := by
  rw [actual_x_fiber]
  exact Set.ncard_pair hP

theorem base_vertical_polynomial_nonzero (x : K) :
    algebraMap K[X] (RatFunc K) (X-C x)≠0 := by
  intro he
  apply X_sub_C_ne_zero x
  apply IsFractionRing.injective K[X] (RatFunc K)
  simpa only [map_zero] using he

def baseVerticalUnit (x : K) : (RatFunc K)ˣ :=
  Units.mk0 (algebraMap K[X] (RatFunc K) (X-C x)) (base_vertical_polynomial_nonzero x)

theorem base_vertical_unit_image (W : WeierstrassCurve.Affine K) (x : K) :
    algebraMap (RatFunc K) W.FunctionField (baseVerticalUnit x : RatFunc K)=verticalFunction W x := by
  change algebraMap (RatFunc K) W.FunctionField (algebraMap K[X] (RatFunc K) (X-C x))=_
  rw [rational_base_polynomial,map_sub,polynomial_x_function,polynomial_function_constant,vertical_formula]

/-- Actual split order1 is derived from the full vertical principal divisor. -/
theorem split_base_parameter_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) :
    pointBaseOrder W (Point.some x y h) (baseVerticalUnit x : RatFunc K)=1 := by
  classical
  rw [point_base_order_embedding,base_vertical_unit_image,vertical_point_order W h]
  let P : W.Point:=Point.some x y h
  have hn : -P≠P:=Ne.symm hP
  have hz : (0 : W.Point)≠P:=(Point.some_ne_zero h).symm
  change ((Finsupp.single P 1+Finsupp.single (-P) 1-Finsupp.single 0 2 : W.Point →₀ ℤ)) P=1
  rw [Finsupp.sub_apply,Finsupp.add_apply,Finsupp.single_eq_same,
    Finsupp.single_eq_of_ne hn.symm,Finsupp.single_eq_of_ne hz.symm]
  norm_num

/-- Actual fixed-point order2 is retained rather than silently normalized to1. -/
theorem fixed_base_parameter_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    pointBaseOrder W (Point.some x y h) (baseVerticalUnit x : RatFunc K)=2 := by
  classical
  rw [point_base_order_embedding,base_vertical_unit_image,vertical_point_order W h]
  let P : W.Point:=Point.some x y h
  have hn : -P=P:=hP.symm
  have hz : (0 : W.Point)≠P:=(Point.some_ne_zero h).symm
  change ((Finsupp.single P 1+Finsupp.single (-P) 1-Finsupp.single 0 2 : W.Point →₀ ℤ)) P=2
  rw [Finsupp.sub_apply,Finsupp.add_apply,hn,Finsupp.single_eq_same,
    Finsupp.single_eq_of_ne hz.symm]
  norm_num

theorem split_base_parameter_value (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) :
    pointBaseValuation W (Point.some x y h) (baseVerticalUnit x : RatFunc K)=WithZero.exp (-1 : ℤ) := by
  have hv : pointBaseValuation W (Point.some x y h) (baseVerticalUnit x : RatFunc K)≠0 :=
    (Valuation.ne_zero_iff _).mpr (baseVerticalUnit x).ne_zero
  have ho:=split_base_parameter_order W h hP
  unfold pointBaseOrder at ho
  have hl : WithZero.log (pointBaseValuation W (Point.some x y h) (baseVerticalUnit x : RatFunc K))= -1 := by omega
  rw [←WithZero.exp_log hv,hl]

/-- Actual normalized split base valuation attains every value, proved from its actual parameter. -/
theorem split_base_valuation_surjective (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) :
    Function.Surjective (pointBaseValuation W (Point.some x y h)) := by
  intro a
  refine WithZero.expRecOn a ?_ (fun n=>?_)
  · exact ⟨0,map_zero _⟩
  · refine ⟨(baseVerticalUnit x : RatFunc K)^(-n),?_⟩
    rw [map_zpow₀,split_base_parameter_value W h hP,←WithZero.exp_zsmul]
    congr 1
    simp

theorem actual_point_base_norm_order (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) :
    pointBaseOrder W P (normUnit W f : RatFunc K)=pointOrder W f P+pointOrder W f (-P) := by
  rw [point_base_order_embedding]
  change pointOrder W (baseFieldUnit W (normUnit W f)) P=_
  rw [base_field_norm_unit,actual_norm_point_order]

#print axioms actual_x_fiber
#print axioms actual_split_fiber_card
#print axioms split_base_parameter_value
#print axioms split_base_valuation_surjective
#print axioms actual_point_base_norm_order
end
end EllipticSplitNormProjectionAudit
